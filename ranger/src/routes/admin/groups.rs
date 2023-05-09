use crate::{errors::RangerError, AppState};
use actix_web::{
    get,
    web::{Data, Json},
};
use actix_web_grants::proc_macro::has_permissions;
use keycloak::{KeycloakAdmin, KeycloakServiceAccountAdminTokenRetriever};
use log::error;

#[get("group")]
#[has_permissions("ranger-manager")]
pub async fn get_exercise_participant_groups(
    app_state: Data<AppState>,
) -> Result<Json<Vec<String>>, RangerError> {
    let client = reqwest::Client::new();
    let token = KeycloakServiceAccountAdminTokenRetriever::create_with_custom_realm(
        &app_state.configuration.keycloak.client_id,
        &app_state.configuration.keycloak.client_secret,
        &app_state.configuration.keycloak.realm,
        client,
    );
    let client = reqwest::Client::new();
    let service_user =
        KeycloakAdmin::new(&app_state.configuration.keycloak.base_url, token, client);
    let keycloak_clients = service_user
        .realm_clients_get(
            &app_state.configuration.keycloak.realm,
            Some(app_state.configuration.keycloak.client_id.clone()),
            None,
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

    let keycloak_client_id = keycloak_clients
        .get(0)
        .ok_or_else(|| {
            error!("Failed to get keycloak client");
            RangerError::KeycloakQueryFailed
        })?
        .clone()
        .id
        .ok_or_else(|| {
            error!("Failed to get keycloak client id");
            RangerError::KeycloakQueryFailed
        })?;
    let roles = service_user
        .realm_clients_with_id_roles_get(
            &app_state.configuration.keycloak.realm,
            &keycloak_client_id,
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
