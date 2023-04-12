use actix_web::web::scope;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Error;
use ranger::app_setup;
use ranger::routes::deployers::get_deployers;
use ranger::routes::email::{get_email_form, send_email};
use ranger::routes::exercise::{
    delete_exercise_deployment, get_exercise, get_exercise_deployment,
    get_exercise_deployment_elements, get_exercise_deployment_nodes,
    get_exercise_deployment_scores, get_exercise_deployments, get_exercises, subscribe_to_exercise,
    update_exercise,
};
use ranger::routes::scenario::get_exercise_deployment_scenario;
use ranger::routes::{
    basic::{status, version},
    exercise::{add_exercise, add_exercise_deployment, delete_exercise},
};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let (host, port, app_state) = app_setup(std::env::args().collect()).await?;
    let app_data = Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.to_owned())
            .service(status)
            .service(version)
            .service(
                scope("/api/v1")
                    .service(get_exercise_deployment_elements)
                    .service(get_exercise_deployments)
                    .service(add_exercise_deployment)
                    .service(get_exercise_deployment)
                    .service(delete_exercise_deployment)
                    .service(subscribe_to_exercise)
                    .service(get_exercises)
                    .service(add_exercise)
                    .service(delete_exercise)
                    .service(update_exercise)
                    .service(get_exercise)
                    .service(get_deployers)
                    .service(get_exercise_deployment_scores)
                    .service(get_exercise_deployment_scenario)
                    .service(send_email)
                    .service(get_email_form)
                    .service(get_exercise_deployment_nodes),
            )
    })
    .bind((host, port))?
    .run()
    .await?;
    Ok(())
}
