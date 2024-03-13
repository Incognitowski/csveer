use crate::{
    commons::file_storage::store_file_for_ingestion,
    config::server::AppError,
    data::{file_destination::list_by_file_source_id, file_source::find_by_context_and_identifier},
};
use aws_sdk_s3::Client as S3Client;
use axum::{
    extract::{multipart::Field, Multipart, Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;
use tracing::{error, info, instrument};

#[derive(Serialize)]
pub struct FileUploadResult {
    file_destinations_affected: Vec<String>,
}

impl FileUploadResult {
    fn new(file_destinations_affected: Vec<String>) -> Self {
        Self {
            file_destinations_affected,
        }
    }
}

#[instrument(skip(db, s3_client, multipart))]
pub async fn file_upload(
    State(db): State<PgPool>,
    State(s3_client): State<S3Client>,
    Path((context, identifier)): Path<(String, String)>,
    mut multipart: Multipart,
) -> anyhow::Result<(StatusCode, Json<FileUploadResult>), AppError> {
    let mut conn = db.acquire().await?;
    let file_source = match find_by_context_and_identifier(&context, &identifier, &mut conn).await?
    {
        Some(fs) => fs,
        None => {
            let message = format!(
                "No file source found for context {} and identifier {}",
                context, identifier
            );
            error!(message);
            return Err(AppError::Validation(message));
        }
    };

    while let Ok(Some(field)) = multipart.next_field().await {
        process_file(&s3_client, &context, &identifier, field).await?;
    }

    let file_destination_names: Vec<String> = list_by_file_source_id(&file_source.id, &mut conn)
        .await?
        .into_iter()
        .map(|i| i.identifier)
        .collect();

    Ok((
        StatusCode::ACCEPTED,
        Json(FileUploadResult::new(file_destination_names)),
    ))
}

#[instrument(skip(s3_client, context, identifier, field))]
async fn process_file(
    s3_client: &S3Client,
    context: &str,
    identifier: &str,
    field: Field<'_>,
) -> anyhow::Result<(), AppError> {
    let file_name = match field.file_name() {
        Some(file_name) => file_name,
        None => {
            info!("File had no file name.");
            return Ok(());
        }
    };
    let bucket_name = "pending-csv-files";
    let file_name = format!("{}/{}/{}", context, identifier, file_name);
    let file_bytes = field.bytes().await?;
    store_file_for_ingestion(&s3_client, &bucket_name, file_bytes.into(), &file_name).await?;
    Ok(())
}
