use crate::{
    database::{AddScenario, GetScenario},
    AppState, scenario::deploy_scenario,
};
use actix_web::{
    post,
    web::{Data, Path},
    HttpResponse,
};
use log::error;
use sdl_parser::parse_sdl;

#[post("exercise")]
pub async fn add_exercise(text: String, app_state: Data<AppState>) -> HttpResponse {
    match parse_sdl(&text) {
        Ok(schema) => {
            if let Err(error) = app_state
                .database_address
                .send(AddScenario(schema.scenario))
                .await
            {
                error!("Database actor mailbox error: {}", error);
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().body("Ok")
        }
        Err(error) => {
            error!("Failed to parse SDL: {}", error);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[post("exercise/{scenario_name}/deployment")]
pub async fn deploy_exercise(
    path_variables: Path<String>,
    app_state: Data<AppState>,
) -> HttpResponse {
    let scenario_name = path_variables.into_inner();
    println!("Adding scenario: {}", scenario_name);
    let scenario = app_state
        .database_address
        .send(GetScenario(scenario_name))
        .await
        .unwrap()
        .unwrap();
        deploy_scenario(scenario).await.unwrap();
    HttpResponse::Ok().body("Ok")
}
