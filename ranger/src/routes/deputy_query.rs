use crate::AppState;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse};
use anyhow::Result;

#[get("")]
pub async fn get_deputy_packages_by_type(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    // todo
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(vec![]))
}
