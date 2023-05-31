use crate::{
    errors::RangerError,
    middleware::authentication::User,
    models::{helpers::uuid::Uuid, Exercise},
    roles::RangerRole,
    services::database::exercise::GetExercise,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_http::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, FromRequest,
};
use futures_util::future::LocalBoxFuture;
use log::{debug, error};
use std::{
    future::{ready, Ready},
    rc::Rc,
};

pub struct ExerciseInfo(pub Rc<Exercise>);

impl FromRequest for ExerciseInfo {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<Rc<Exercise>>().cloned();
        let result = match value {
            Some(v) => Ok(ExerciseInfo(v)),
            None => Err(RangerError::KeycloakQueryFailed.into()),
        };
        ready(result)
    }
}

impl std::ops::Deref for ExerciseInfo {
    type Target = Rc<Exercise>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ExerciseMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for ExerciseMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ExerciseMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ExerciseMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct ExerciseMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for ExerciseMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let user = req.extensions().get::<Rc<User>>().cloned();
        let app_state = req.app_data::<Data<AppState>>().cloned();

        Box::pin(async move {
            let user = user.ok_or_else(|| {
                error!("User not found");
                RangerError::UserInfoMissing
            })?;
            let app_state = app_state.ok_or_else(|| {
                error!("App state not found");
                RangerError::AppStateMissing
            })?;
            let exercise_uuid = req.match_info().get("exercise_uuid");

            let exercise = match (user.role, exercise_uuid) {
                (RangerRole::Admin, Some(exercise_uuid)) => {
                    let exercise_uuid = Uuid::try_from(exercise_uuid).map_err(|_| {
                        error!("Invalid exercise uuid");
                        RangerError::UuidParsingFailed
                    })?;
                    debug!("Getting exercise with uuid: {:?}", exercise_uuid);

                    let exercise = app_state
                        .database_address
                        .send(GetExercise(exercise_uuid))
                        .await
                        .map_err(create_mailbox_error_handler("Database"))?
                        .map_err(create_database_error_handler("Get exercises"))?;

                    std::result::Result::Ok(exercise)
                }
                _ => Err(RangerError::ExericseNotFound),
            }?;
            req.extensions_mut().insert(Rc::new(exercise));

            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
