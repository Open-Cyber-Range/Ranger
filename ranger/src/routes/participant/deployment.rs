use crate::{
    errors::RangerError,
    middleware::{
        authentication::UserInfo, deployment::DeploymentInfo, exercise::ExerciseInfo,
        keycloak::KeycloakInfo,
    },
    models::{Deployment, ParticipantDeployment},
    services::database::deployment::GetDeployments,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    get,
    web::{Data, Json},
};
use futures_util::future::join_all;

#[get("")]
pub async fn get_participant_deployments(
    app_state: Data<AppState>,
    user_info: UserInfo,
    keycloak_info: KeycloakInfo,
    exercise: ExerciseInfo,
) -> Result<Json<Vec<ParticipantDeployment>>, RangerError> {
    let deployments = app_state
        .database_address
        .send(GetDeployments(exercise.id))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Get deployments"))?;

    let deployment_with_members: Vec<(Deployment, bool)> =
        join_all(deployments.into_iter().map(|deployment| async {
            let is_member = deployment
                .is_member(
                    user_info.id.clone(),
                    KeycloakInfo(keycloak_info.clone()),
                    app_state.configuration.keycloak.realm.clone(),
                )
                .await
                .unwrap_or(false);
            (deployment, is_member)
        }))
        .await;
    let deployments = deployment_with_members
        .into_iter()
        .filter_map(|(exercise, is_member)| {
            if is_member {
                return Some(exercise);
            }
            None
        })
        .collect::<Vec<_>>();

    let participant_deployments = deployments
        .into_iter()
        .map(ParticipantDeployment::from)
        .collect();

    Ok(Json(participant_deployments))
}

#[get("")]
pub async fn get_participant_deployment(
    deployment: DeploymentInfo,
) -> Result<Json<ParticipantDeployment>, RangerError> {
    let deployment = deployment.into_inner().into();

    Ok(Json(deployment))
}
