use axum::{
    Json,
    extract::{Path, State},
};
use solana_sdk::pubkey::Pubkey;

use crate::app_state::AppState;
use crate::domain::AccountResponse;
use crate::error::{ApiError, map_error};
use common::error::HttpError;

#[tracing::instrument(skip(state), fields(program_id = %program_id_str))]
pub async fn get_program_accounts(
    State(state): State<AppState>,
    Path(program_id_str): Path<String>,
) -> Result<Json<Vec<AccountResponse>>, HttpError> {
    let program_id: Pubkey = program_id_str
        .parse()
        .map_err(|_| map_error(ApiError::InvalidPubkey))?;

    let accounts = state
        .rpc
        .get_program_accounts(&program_id)
        .map_err(|e| map_error(ApiError::Rpc(e)))?;

    let result = accounts
        .into_iter()
        .map(|(pubkey, acc)| AccountResponse {
            pubkey: pubkey.to_string(),
            lamports: acc.lamports,
            owner: acc.owner.to_string(),
            executable: acc.executable,
            data_len: acc.data.len(),
        })
        .collect();

    Ok(Json(result))
}
