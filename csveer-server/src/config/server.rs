use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub struct AppError(anyhow::Error);

impl AppError {
    pub fn new(message: String) -> Self {
        Self {
            0: anyhow::format_err!(message),
        }
    }
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
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_string(&ErrorResponse::new(format!(
                    "Something went wrong: {}",
                    self.0
                )))
                .unwrap(),
            ))
            .unwrap()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
