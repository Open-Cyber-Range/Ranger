use crate::{
    errors::RangerError,
    middleware::authentication::User,
    models::{helpers::uuid::Uuid, Deployment},
    roles::RangerRole,
    services::database::deployment::GetDeployment,
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

pub struct DeploymentInfo(pub Deployment);

impl DeploymentInfo {
    pub fn into_inner(self) -> Deployment {
        self.0
    }
}

impl FromRequest for DeploymentInfo {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<Deployment>().cloned();
        let result = match value {
            Some(v) => Ok(DeploymentInfo(v)),
            None => Err(RangerError::KeycloakQueryFailed.into()),
        };
        ready(result)
    }
}

impl std::ops::Deref for DeploymentInfo {
    type Target = Deployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DeploymentMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for DeploymentMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DeploymentMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DeploymentMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct DeploymentMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for DeploymentMiddleware<S>
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
            let deployment_uuid = req.match_info().get("deployment_uuid");

            let deployment = match (user.role, deployment_uuid) {
                (RangerRole::Admin, Some(deployment_uuid)) => {
                    let deployment_uuid = Uuid::try_from(deployment_uuid).map_err(|_| {
                        error!("Invalid deployment uuid");
                        RangerError::UuidParsingFailed
                    })?;
                    debug!("Getting deployment with uuid: {:?}", deployment_uuid);

                    let deployment = app_state
                        .database_address
                        .send(GetDeployment(deployment_uuid))
                        .await
                        .map_err(create_mailbox_error_handler("Database"))?
                        .map_err(create_database_error_handler("Get exercises"))?;

                    std::result::Result::Ok(deployment)
                }
                _ => Err(RangerError::ExericseNotFound),
            }?;
            req.extensions_mut().insert(deployment);

            let res = service.call(req).await?;
            Ok(res)
        })
    }
}
