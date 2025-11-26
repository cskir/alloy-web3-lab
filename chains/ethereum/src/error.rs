use axum::http::StatusCode;
use common::error::{HttpError, http_error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("invalid address")]
    InvalidAddress,
    #[error("provider error: {0}")]
    Provider(#[from] ethers::providers::ProviderError),
    #[error("contract call error: {0}")]
    Contract(String),
    #[error("no logs found for this contract")]
    NoLogs,
}

pub fn map_error(e: ApiError) -> HttpError {
    match e {
        ApiError::InvalidAddress => http_error(StatusCode::BAD_REQUEST, e),
        ApiError::NoLogs => http_error(StatusCode::NOT_FOUND, e),
        ApiError::Provider(_) | ApiError::Contract(_) => {
            http_error(StatusCode::INTERNAL_SERVER_ERROR, e)
        }
    }
}
