use axum::http::Uri;

use crate::config::server::AppError;

pub fn validate(queue_url: &String) -> anyhow::Result<(), AppError> {
    if queue_url.is_empty() {
        return Err(AppError::DetailedValidation(
            "Invalid queue URL for SQS destination".to_string(),
            vec!["Queue URL should not be empty".to_string()],
        ));
    }

    if Uri::try_from(queue_url).is_err() {
        return Err(AppError::DetailedValidation(
            "Invalid queue URL for SQS destination".to_string(),
            vec![format!("Queue URL '{}' is invalid", queue_url)],
        ));
    }

    Ok(())
}
