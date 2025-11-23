use axum::{
    Json,
    extract::{Path, State},
};
use solana_sdk::pubkey::Pubkey;

use crate::app_state::AppState;
use crate::domain::AccountBalance;
use crate::error::{ApiError, map_error};
use common::error::HttpError;

#[tracing::instrument(skip(state), fields(program_id = %account_str))]
pub async fn get_balance(
    State(state): State<AppState>,
    Path(account_str): Path<String>,
) -> Result<Json<AccountBalance>, HttpError> {
    let account_pubkey: Pubkey = account_str
        .parse()
        .map_err(|_| map_error(ApiError::InvalidPubkey))?;

    let balance = state
        .rpc
        .get_balance(&account_pubkey)
        .map_err(|e| map_error(ApiError::Rpc(e)))?;

    let result = AccountBalance {
        pubkey: account_pubkey.to_string(),
        balance_lamport: balance,
    };

    Ok(Json(result))
}
