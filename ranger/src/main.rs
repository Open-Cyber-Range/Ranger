use actix_web::{web::scope, web::Data, App, HttpServer};
use anyhow::Error;
use ranger::app_setup;
use ranger::middleware::authentication::AuthenticationMiddlewareFactory;
use ranger::middleware::deployment::DeploymentMiddlewareFactory;
use ranger::middleware::exercise::ExerciseMiddlewareFactory;
use ranger::middleware::keycloak::KeycloakAccessMiddlewareFactory;
use ranger::roles::RangerRole;
use ranger::routes::{
    admin::groups::{get_participant_groups, get_participant_groups_users},
    basic::{status, version},
    deployers::get_deployers,
    email::send_email,
    exercise::{
        add_exercise, add_exercise_deployment, add_participant, delete_exercise,
        delete_exercise_deployment, delete_participant, get_exercise, get_exercise_deployment,
        get_exercise_deployment_users, get_exercise_deployment_elements,
        get_exercise_deployment_scores, get_exercise_deployments,
        get_exercises, get_participants, subscribe_to_exercise, update_exercise,
    },
    scenario::get_exercise_deployment_scenario,
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (host, port, app_state) = app_setup(std::env::args().collect()).await?;
    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        let admin_auth_middleware = AuthenticationMiddlewareFactory(RangerRole::Admin);
        let participant_auth_middleware = AuthenticationMiddlewareFactory(RangerRole::Participant);
        App::new()
            .app_data(app_data.to_owned())
            .service(status)
            .service(version)
            .service(
                scope("/api/v1")
                    .service(
                        scope("/admin")
                            .service(
                                scope("/exercise")
                                    .service(get_exercises)
                                    .service(add_exercise)
                                    .service(
                                        scope("/{exercise_uuid}")
                                            .wrap(ExerciseMiddlewareFactory)
                                            .service(get_exercise)
                                            .service(update_exercise)
                                            .service(delete_exercise)
                                            .service(subscribe_to_exercise)
                                            .service(send_email)
                                            .service(
                                                scope("/deployment")
                                                    .service(get_exercise_deployments)
                                                    .service(add_exercise_deployment)
                                                    .service(
                                                        scope("/{deployment_uuid}")
                                                            .service(get_exercise_deployment)
                                                            .service(
                                                                get_exercise_deployment_elements,
                                                            )
                                                            .service(delete_exercise_deployment)
                                                            .service(get_participants)
                                                            .service(add_participant)
                                                            .service(delete_participant)
                                                            .service(get_exercise_deployment_scores)
                                                            .service(
                                                                get_exercise_deployment_scenario,
                                                            )
                                                            .service(get_exercise_deployment_users)
                                                            .wrap(DeploymentMiddlewareFactory),
                                                    ),
                                            ),
                                    ),
                            )
                            .service(get_deployers)
                            .service(
                                scope("/group")
                                    .service(get_participant_groups)
                                    .service(get_participant_groups_users),
                            )
                            .wrap(admin_auth_middleware),
                    )
                    .service(
                        scope("/participant")
                            .service(get_exercise_deployment_scenario)
                            .service(get_exercise_deployment_users)
                            .service(get_exercise_deployment_scores)
                            .wrap(participant_auth_middleware),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
