use crate::{errors::RangerError, middleware::keycloak::KeycloakInfo, AppState};
use actix_web::{
    get,
    web::{Data, Json, Path},
};
use actix_web_grants::proc_macro::has_permissions;
use log::error;

#[get("")]
#[has_permissions("ranger-manager")]
pub async fn get_participant_groups(
    app_state: Data<AppState>,
    keycloak_info: KeycloakInfo,
) -> Result<Json<Vec<String>>, RangerError> {
    let roles = keycloak_info
        .service_user
        .realm_clients_with_id_roles_get(
            &app_state.configuration.keycloak.realm,
            &keycloak_info.client_id,
            None,
            None,
            None,
            None,
        )
        .await
        .map_err(|error| {
            error!("Failed to get keycloak clients: {error}");
            RangerError::KeycloakQueryFailed
        })?;
    let groups: Vec<String> = roles.into_iter().filter_map(|role| role.name).collect();

    Ok(Json(groups))
}

#[get("/{group_name}/users")]
#[has_permissions("ranger-manager")]
pub async fn get_participant_groups_users(
    path_variables: Path<String>,
    app_state: Data<AppState>,
    keycloak_info: KeycloakInfo,
) -> Result<Json<Vec<String>>, RangerError> {
    let role_name = path_variables.into_inner();
    let users = keycloak_info
        .service_user
        .realm_clients_with_id_roles_with_role_name_users_get(
            &app_state.configuration.keycloak.realm,
            &keycloak_info.client_id,
            &role_name,
            None,
            None,
        )
        .await
        .map_err(|error| {
            error!("Failed to get keycloak clients: {error}");
            RangerError::KeycloakQueryFailed
        })?;
    let usernames: Vec<String> = users.into_iter().filter_map(|user| user.username).collect();

    Ok(Json(usernames))
}
