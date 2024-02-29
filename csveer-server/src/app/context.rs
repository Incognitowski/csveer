use crate::{
    config::server::AppError,
    data::context::{get_context_by_name, insert_context, CreatableContext, SourceContext},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::instrument;

pub fn validate_context_name(name: &String) -> anyhow::Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation(String::from(
            "Context name should not be empty.",
        )));
    }

    for char in name.chars() {
        if !&char.is_digit(36) && &char != &'-' {
            return Err(AppError::Validation(format!(
                "Context name should only contain numbers or charaters. Found char '{}'",
                &char
            )));
        }
    }

    Ok(())
}

#[instrument(skip(db))]
pub async fn create_context(
    State(db): State<PgPool>,
    Json(creatable_context): Json<CreatableContext>,
) -> anyhow::Result<(StatusCode, Json<SourceContext>), AppError> {
    // If you need a connection without a transaction
    // let mut db_conn = db.acquire().await?;

    validate_context_name(&creatable_context.name)?;

    let mut tx = db.begin().await?;

    if let Some(_) = get_context_by_name(&creatable_context.name, &mut *tx).await {
        return Err(AppError::Validation(format!(
            "A context with name '{}' already exists.",
            &creatable_context.name
        )));
    }

    let context = insert_context(creatable_context, &mut *tx).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(context)))
}
