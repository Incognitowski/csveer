use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::{
    app::context::validate_context_name,
    config::server::AppError,
    data::{
        context::{get_context_by_name, insert_context, CreatableContext},
        file_source::{insert_file_source, FileSource, FileSourceCreation},
    },
};

#[instrument(skip(db, creatable_file_source))]
pub async fn create_file_source(
    State(db): State<PgPool>,
    Json(creatable_file_source): Json<FileSourceCreation>,
) -> anyhow::Result<(StatusCode, Json<FileSource>), AppError> {
    let mut tx = db.begin().await?;

    validate_context_name(&creatable_file_source.context)?;

    if get_context_by_name(&creatable_file_source.context, &mut *tx)
        .await
        .is_none()
    {
        info!(
            "Context {} does not exist, will be created.",
            &creatable_file_source.context
        );
        insert_context(
            CreatableContext::new(creatable_file_source.context.clone()),
            &mut *tx,
        )
        .await?;
    }

    let created_file_source = insert_file_source(creatable_file_source, &mut *tx).await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(created_file_source)))
}
