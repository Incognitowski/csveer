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

pub fn validate_file_source(
    creatable_file_source: &FileSourceCreation,
) -> anyhow::Result<(), AppError> {
    if creatable_file_source.identifier.is_empty() {
        return Err(AppError::DetailedValidation(
            String::from("Invalid file source identifier"),
            vec![String::from("Should not be blank.")],
        ));
    }

    for char in creatable_file_source.identifier.chars() {
        if !&char.is_digit(36) && &char != &'-' {
            return Err(AppError::DetailedValidation(
                String::from("Invalid file source identifier"),
                vec![
                    format!(
                        "File source identifier should only contain numbers or charaters. Found char '{}'", 
                        &char
                    )
                ]),
            );
        }
    }

    if let Some(columns) = &creatable_file_source.hide_columns {
        for col in columns {
            if col.is_negative() {
                return Err(AppError::DetailedValidation(
                    String::from("Invalid hide column value"),
                    vec![format!("{} is not a valid column index", col)],
                ));
            }
        }
    }

    Ok(())
}

#[instrument(skip(db, creatable_file_source))]
pub async fn create_file_source(
    State(db): State<PgPool>,
    Json(creatable_file_source): Json<FileSourceCreation>,
) -> anyhow::Result<(StatusCode, Json<FileSource>), AppError> {
    validate_context_name(&creatable_file_source.context)?;
    validate_file_source(&creatable_file_source)?;

    let mut tx = db.begin().await?;

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
