use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use tracing::{info, instrument};

use crate::{
    config::server::AppError,
    data::{
        file_destination::{insert_file_destination, FileDestination, FileDestinationCreation},
        file_source::find_by_context_and_identifier,
    },
};

pub fn validate_file_destination(
    creatable_file_destination: &FileDestinationCreation,
) -> anyhow::Result<(), AppError> {
    if creatable_file_destination.identifier.is_empty() {
        return Err(AppError::DetailedValidation(
            String::from("Invalid file destination identifier"),
            vec![String::from("Should not be blank.")],
        ));
    }

    for char in creatable_file_destination.identifier.chars() {
        if !&char.is_digit(36) && &char != &'-' {
            return Err(AppError::DetailedValidation(
                String::from("Invalid file destination identifier"),
                vec![
                    format!(
                        "File destination identifier should only contain numbers or charaters. Found char '{}'", 
                        &char
                    )
                ]),
            );
        }
    }

    // TODO: Validate everything else

    Ok(())
}

#[instrument(skip(db, creatable_file_destination))]
pub async fn create_file_destination(
    Path((context, file_source)): Path<(String, String)>,
    State(db): State<PgPool>,
    Json(creatable_file_destination): Json<FileDestinationCreation>,
) -> anyhow::Result<(StatusCode, Json<FileDestination>), AppError> {
    info!(
        "About to create file destination with context {}, identifier {} and spec {:?}",
        context, file_source, creatable_file_destination,
    );

    validate_file_destination(&creatable_file_destination)?;

    let mut tx = db.begin().await?;

    let file_source = match find_by_context_and_identifier(&context, &file_source, &mut *tx).await?
    {
        None => {
            let message = format!(
                "File source with context {} and identifier {} does not exist.",
                context, file_source
            );
            info!(message);
            return Err(AppError::Validation(message));
        }
        Some(fs) => fs,
    };

    let created_file_destination =
        insert_file_destination(&file_source.id, creatable_file_destination, &mut *tx).await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(created_file_destination)))
}
