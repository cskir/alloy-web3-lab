use axum::http::StatusCode;
use common::error::{HttpError, http_error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("invalid pubkey")]
    InvalidPubkey,
    #[error("rpc error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
}

pub fn map_error(e: ApiError) -> HttpError {
    match e {
        ApiError::InvalidPubkey => http_error(StatusCode::BAD_REQUEST, e),
        ApiError::Rpc(_) => http_error(StatusCode::INTERNAL_SERVER_ERROR, e),
    }
}
