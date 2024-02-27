use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub enum AppError {
    Validation(String),
    DetailedValidation(String, Vec<String>),
    Error(String),
    DetailedError(String, Vec<String>),
    // AnyhowError(anyhow::Error),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
    details: Vec<String>,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: Vec::new(),
        }
    }

    pub fn new_detailed(message: String, details: Vec<String>) -> Self {
        Self { message, details }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::Validation(_) | Self::DetailedValidation(_, _) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_response = match self {
            AppError::Validation(m) => ErrorResponse::new(m),
            AppError::DetailedValidation(m, d) => ErrorResponse::new_detailed(m, d),
            AppError::Error(m) => ErrorResponse::new(m),
            AppError::DetailedError(m, d) => ErrorResponse::new_detailed(m, d),
            // AppError::AnyhowError(e) => ErrorResponse::new(e.to_string()),
        };

        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&error_response).unwrap()))
            .unwrap()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error> + std::fmt::Display,
{
    fn from(err: E) -> Self {
        Self::Error(format!("{:#}", err))
    }
}
