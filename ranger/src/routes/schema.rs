use crate::{
    errors::RangerError,
    models::helpers::uuid::Uuid,
    services::database::deployment::GetDeployment,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use anyhow::Result;
use log::error;
use sdl_parser::{
    entity::Entities, evaluation::Evaluations, goal::Goals, metric::Metrics, parse_sdl,
    training_learning_objective::TrainingLearningObjectives,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub entities: Entities,
    pub goals: Goals,
    pub tlos: TrainingLearningObjectives,
    pub evaluations: Evaluations,
    pub metrics: Metrics,
}

#[get("exercise/{exercise_uuid}/deployment/{deployment_uuid}/schema")]
pub async fn get_exercise_deployment_schema(
    path_variables: Path<(Uuid, Uuid)>,
    app_state: Data<AppState>,
) -> Result<Option<Json<Schema>>, RangerError> {
    let (_, deployment_uuid) = path_variables.into_inner();

    let deployment = app_state
        .database_address
        .send(GetDeployment(deployment_uuid))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployment"))?;

    let scenario = parse_sdl(&deployment.sdl_schema).map_err(|error| {
        error!("Failed to parse sdl: {error}");
        RangerError::ScenarioParsingFailed
    })?;

    if let (
        Some(scenario_entities),
        Some(scenario_goal),
        Some(scenario_tlos),
        Some(scenario_evaluations),
        Some(scenario_metrics),
    ) = (
        scenario.entities,
        scenario.goals,
        scenario.tlos,
        scenario.evaluations,
        scenario.metrics,
    ) {
        let schema = Schema {
            entities: scenario_entities,
            goals: scenario_goal,
            tlos: scenario_tlos,
            evaluations: scenario_evaluations,
            metrics: scenario_metrics,
        };
        return Ok(Some(Json(schema)));
    }
    Ok(None)
}
