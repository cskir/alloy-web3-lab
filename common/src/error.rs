use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub type HttpError = (axum::http::StatusCode, Json<ErrorResponse>);

pub fn http_error(status: axum::http::StatusCode, msg: impl ToString) -> HttpError {
    (
        status,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
}
