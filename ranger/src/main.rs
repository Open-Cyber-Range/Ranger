use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use anyhow::Error;
use ranger::app_setup;
use ranger::claims::validator;
use ranger::middleware::keycloak::KeycloakAccessMiddlewareFactory;
use ranger::routes::admin::groups::get_participant_groups_users;
use ranger::routes::deployers::get_deployers;
use ranger::routes::email::{get_email_form, send_email};
use ranger::routes::exercise::{
    add_participant, delete_exercise_deployment, delete_participant, get_exercise,
    get_exercise_deployment, get_exercise_deployment_elements, get_exercise_deployment_scores,
    get_exercise_deployments, get_exercises, get_participants, subscribe_to_exercise,
    update_exercise,
};
use ranger::routes::scenario::get_exercise_deployment_scenario;
use ranger::routes::{
    admin::groups::get_participant_groups,
    basic::{status, version},
    exercise::{add_exercise, add_exercise_deployment, delete_exercise},
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (host, port, app_state) = app_setup(std::env::args().collect()).await?;
    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(app_data.to_owned())
            .service(status)
            .service(version)
            .service(
                scope("/api/v1")
                    .wrap(auth)
                    .service(
                        scope("/admin").service(
                            scope("/group")
                                .wrap(KeycloakAccessMiddlewareFactory)
                                .service(get_participant_groups)
                                .service(get_participant_groups_users),
                        ),
                    )
                    .service(get_exercise_deployment_elements)
                    .service(get_exercise_deployments)
                    .service(add_exercise_deployment)
                    .service(get_exercise_deployment)
                    .service(get_exercise_deployment_scenario)
                    .service(get_email_form)
                    .service(delete_exercise_deployment)
                    .service(subscribe_to_exercise)
                    .service(get_exercises)
                    .service(add_exercise)
                    .service(delete_exercise)
                    .service(update_exercise)
                    .service(get_exercise)
                    .service(get_deployers)
                    .service(get_exercise_deployment_scores)
                    .service(send_email)
                    .service(add_participant)
                    .service(get_participants)
                    .service(delete_participant),
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
