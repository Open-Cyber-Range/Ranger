use actix_web::{web::scope, web::Data, App, HttpServer};
use anyhow::Error;
use ranger::app_setup;
use ranger::middleware::authentication::AuthenticationMiddlewareFactory;
use ranger::middleware::deployment::DeploymentMiddlewareFactory;
use ranger::middleware::exercise::ExerciseMiddlewareFactory;
use ranger::middleware::keycloak::KeycloakAccessMiddlewareFactory;
use ranger::middleware::metric::MetricMiddlewareFactory;
use ranger::roles::RangerRole;
use ranger::routes::admin::groups::get_participant_groups_users;
use ranger::routes::admin::metric::{
    delete_metric, download_metric_artifact, get_admin_metric, get_admin_metrics,
    update_admin_metric,
};
use ranger::routes::admin::scenario::get_admin_exercise_deployment_scenario;
use ranger::routes::deployers::get_deployers;
use ranger::routes::email::send_email;
use ranger::routes::exercise::{
    add_participant, delete_exercise_deployment, delete_participant, get_admin_participants,
    get_exercise, get_exercise_deployment, get_exercise_deployment_elements,
    get_exercise_deployment_scores, get_exercise_deployment_users, get_exercise_deployments,
    get_exercises, subscribe_to_exercise, update_exercise,
};
use ranger::routes::participant::deployment::{
    get_participant_deployment, get_participant_deployments,
};
use ranger::routes::participant::events::get_participant_events;
use ranger::routes::participant::metric::{
    add_metric, get_participant_metric, get_participant_metrics, update_participant_metric,
};
use ranger::routes::participant::participants::get_own_participants;
use ranger::routes::participant::scenario::get_participant_exercise_deployment_scenario;
use ranger::routes::participant::{get_participant_exercise, get_participant_exercises};

use ranger::routes::{
    admin::groups::get_participant_groups,
    basic::{status, version},
    exercise::{add_exercise, add_exercise_deployment, delete_exercise},
    upload::upload_participant_artifact,
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
                    .wrap(KeycloakAccessMiddlewareFactory)
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
                                                            .wrap(DeploymentMiddlewareFactory)
                                                            .service(get_exercise_deployment)
                                                            .service(
                                                                get_exercise_deployment_elements,
                                                            )
                                                            .service(delete_exercise_deployment)
                                                            .service(get_admin_participants)
                                                            .service(add_participant)
                                                            .service(delete_participant)
                                                            .service(get_exercise_deployment_scores)
                                                            .service(
                                                                get_admin_exercise_deployment_scenario,
                                                            )
                                                            .service(get_exercise_deployment_users)
                                                            .service(
                                                                scope("/metric") 
                                                                .service(get_admin_metrics)
                                                                .service(
                                                                    scope("/{metric_uuid}")
                                                                    .wrap(MetricMiddlewareFactory)
                                                                        .service(get_admin_metric)
                                                                        .service(update_admin_metric)
                                                                        .service(delete_metric)
                                                                        .service(download_metric_artifact)
                                                                    ),
                                                            ),
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
                            .service(
                                scope("/exercise")
                                    .service(get_participant_exercises)
                                    .service(
                                        scope("/{exercise_uuid}")
                                            .service(get_participant_exercise)
                                            .service(
                                                scope("/deployment")
                                                    .service(get_participant_deployments)
                                                    .service(
                                                        scope("/{deployment_uuid}")
                                                            .service(get_participant_deployment)
                                                            .service(
                                                                get_participant_exercise_deployment_scenario,
                                                            )
                                                            .service(get_exercise_deployment_users)
                                                            .service(get_exercise_deployment_scores)
                                                            .service(get_own_participants)
                                                            .service(get_participant_events)
                                                            .wrap(DeploymentMiddlewareFactory)
                                                            .service(
                                                                scope("/metric")
                                                                .service(get_participant_metrics)
                                                                .service(add_metric)
                                                                .service(
                                                                    scope("/{metric_uuid}")
                                                                    .wrap(MetricMiddlewareFactory)
                                                                        .service(get_participant_metric)
                                                                        .service(update_participant_metric)
                                                                        .service(upload_participant_artifact)
                                                                    ),
                                                            ),
                                                    ),
                                            )
                                            .wrap(ExerciseMiddlewareFactory),
                                    ),
                            )
                            .wrap(participant_auth_middleware),
                    ),
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
