use crate::{
    errors::RangerError,
    middleware::{deployment::DeploymentInfo, metric::MetricInfo},
    models::{helpers::uuid::Uuid, metric::UpdateMetric, Metric},
    services::database::metric::{DeleteMetric, GetMetrics},
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_web::{
    delete, get, put,
    web::{Data, Json},
    HttpResponse,
};

#[put("")]
pub async fn update_metric(
    app_state: Data<AppState>,
    metric_info: MetricInfo,
    update_metric: Json<UpdateMetric>,
) -> Result<Json<Metric>, RangerError> {
    let metric = metric_info.into_inner();
    let update_metric = update_metric.into_inner();

    let metric = app_state
        .database_address
        .send(crate::services::database::metric::UpdateMetric(
            metric.id,
            update_metric,
        ))
        .await
        .map_err(create_mailbox_error_handler("Deployment"))?
        .map_err(create_database_error_handler("Create deployment"))?;
    Ok(Json(metric))
}

#[get("")]
pub async fn get_metric(metric_info: MetricInfo) -> Result<Json<Metric>, RangerError> {
    Ok(Json(metric_info.into_inner()))
}

#[get("")]
pub async fn get_admin_metrics(
    app_state: Data<AppState>,
    deployment: DeploymentInfo,
) -> Result<Json<Vec<Metric>>, RangerError> {
    Ok(Json(
        app_state
            .database_address
            .send(GetMetrics(deployment.id))
            .await
            .map_err(create_mailbox_error_handler("Database"))?
            .map_err(create_database_error_handler("Get Manual Metrics"))?,
    ))
}

#[delete("")]
pub async fn delete_metric(
    app_state: Data<AppState>,
    metric_info: MetricInfo,
) -> Result<Json<Uuid>, RangerError> {
    let metric = metric_info.into_inner();
    app_state
        .database_address
        .send(DeleteMetric(metric.id, false))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Delete Manual Metric"))?;

    Ok(Json(metric.id))
}

#[get("/download")]
pub async fn download_metric_artifact(
    metric_info: MetricInfo,
) -> Result<HttpResponse, RangerError> {
    // TODO: get blob byte array by metric id from artifacts table
    let _metric_id = metric_info.into_inner().id;

    // red pixel jpeg
    let file_data: Vec<u8> = [
        255, 216, 255, 224, 0, 16, 74, 70, 73, 70, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 255, 219, 0, 67,
        0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 255, 219, 0, 67, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 255, 192, 0, 17, 8, 0, 1, 0, 1, 3, 1, 34, 0,
        2, 17, 1, 3, 17, 1, 255, 196, 0, 31, 0, 0, 1, 5, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 16, 0, 2, 1, 3, 3, 2, 4, 3, 5, 5, 4,
        4, 0, 0, 1, 125, 1, 2, 3, 0, 4, 17, 5, 18, 33, 49, 65, 6, 19, 81, 97, 7, 34, 113, 20, 50,
        129, 145, 161, 8, 35, 66, 177, 193, 21, 82, 209, 240, 36, 51, 98, 114, 130, 9, 10, 22, 23,
        24, 25, 26, 37, 38, 39, 40, 41, 42, 52, 53, 54, 55, 56, 57, 58, 67, 68, 69, 70, 71, 72, 73,
        74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106, 115, 116, 117,
        118, 119, 120, 121, 122, 131, 132, 133, 134, 135, 136, 137, 138, 146, 147, 148, 149, 150,
        151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178, 179, 180, 181, 182,
        183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210, 211, 212, 213, 214,
        215, 216, 217, 218, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 241, 242, 243, 244,
        245, 246, 247, 248, 249, 250, 255, 196, 0, 31, 1, 0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255, 196, 0, 181, 17, 0, 2, 1, 2, 4, 4, 3, 4,
        7, 5, 4, 4, 0, 1, 2, 119, 0, 1, 2, 3, 17, 4, 5, 33, 49, 6, 18, 65, 81, 7, 97, 113, 19, 34,
        50, 129, 8, 20, 66, 145, 161, 177, 193, 9, 35, 51, 82, 240, 21, 98, 114, 209, 10, 22, 36,
        52, 225, 37, 241, 23, 24, 25, 26, 38, 39, 40, 41, 42, 53, 54, 55, 56, 57, 58, 67, 68, 69,
        70, 71, 72, 73, 74, 83, 84, 85, 86, 87, 88, 89, 90, 99, 100, 101, 102, 103, 104, 105, 106,
        115, 116, 117, 118, 119, 120, 121, 122, 130, 131, 132, 133, 134, 135, 136, 137, 138, 146,
        147, 148, 149, 150, 151, 152, 153, 154, 162, 163, 164, 165, 166, 167, 168, 169, 170, 178,
        179, 180, 181, 182, 183, 184, 185, 186, 194, 195, 196, 197, 198, 199, 200, 201, 202, 210,
        211, 212, 213, 214, 215, 216, 217, 218, 226, 227, 228, 229, 230, 231, 232, 233, 234, 242,
        243, 244, 245, 246, 247, 248, 249, 250, 255, 218, 0, 12, 3, 1, 0, 2, 17, 3, 17, 0, 63, 0,
        252, 95, 162, 138, 43, 252, 167, 63, 239, 224, 255, 217,
    ]
    .to_vec();
    Ok(HttpResponse::Ok()
        .content_type("image/jpeg")
        .body(file_data))
}
