use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::{
    config::server::AppError,
    data::file_source::{insert_file_source, FileSource, FileSourceCreation},
};

pub async fn create_file_source(
    State(db): State<PgPool>,
    Json(creatable_file_source): Json<FileSourceCreation>,
) -> anyhow::Result<(StatusCode, Json<FileSource>), AppError> {
    let mut tx = db.begin().await?;

    let created_file_source = insert_file_source(creatable_file_source, &mut *tx).await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(created_file_source)))
}
