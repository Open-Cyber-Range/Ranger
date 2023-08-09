use crate::{
    constants::MAX_ARTIFACT_FILE_SIZE,
    errors::RangerError,
    models::upload::NewArtifact,
    services::database::upload::UploadArtifact,
    utilities::{create_database_error_handler, create_mailbox_error_handler},
    AppState,
};
use actix_multipart::Multipart;
use actix_web::{
    http::header::CONTENT_LENGTH,
    post,
    web::{self, Data},
    HttpRequest,
};
use futures_util::{StreamExt, TryStreamExt};

#[post("/upload")]
pub async fn upload_participant_artifacts(
    app_state: Data<AppState>,
    mut payload: Multipart,
    req: HttpRequest,
) -> Result<String, RangerError> {
    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(length) => match length.to_str() {
            Ok(length) => match length.parse::<usize>() {
                Ok(length) => length,
                Err(_) => return Err(RangerError::FileUploadFailed),
            },
            Err(_) => return Err(RangerError::FileUploadFailed),
        },
        None => return Err(RangerError::FileUploadFailed),
    };

    if content_length > MAX_ARTIFACT_FILE_SIZE {
        return Err(RangerError::FileUploadFailed);
    }

    let mut contents = web::BytesMut::new();
    let filename: String;

    if let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        filename = match content_type.get_filename() {
            Some(name) => name.to_string(),
            None => return Err(RangerError::FileUploadFailed),
        };

        while let Some(chunk) = field.next().await {
            let data = chunk.map_err(|_| RangerError::FileUploadFailed)?;
            contents.extend_from_slice(&data);
        }
    } else {
        return Err(RangerError::FileUploadFailed);
    }

    let file_bytes = NewArtifact::new(filename, contents.to_vec());

    let artifact_uuid = app_state
        .database_address
        .send(UploadArtifact(file_bytes))
        .await
        .map_err(create_mailbox_error_handler("Database"))?
        .map_err(create_database_error_handler("Upload file"))?;
    Ok(artifact_uuid.0.to_string())
}
