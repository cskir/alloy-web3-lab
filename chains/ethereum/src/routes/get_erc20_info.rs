use axum::{
    Json,
    extract::{Path, State},
};
use ethers::contract::abigen;
use ethers::types::Address;

use crate::app_state::AppState;
use crate::domain::Erc20InfoResponse;
use crate::error::{ApiError, map_error};
use common::error::HttpError;

// ---- ERC20 ABI binding (ethers-rs abigen!) ----
abigen!(
    ERC20,
    r#"[
        {"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},
        {"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},
        {"inputs":[],"name":"decimals","outputs":[{"internalType":"uint8","name":"","type":"uint8"}],"stateMutability":"view","type":"function"},
        {"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"}
    ]"#
);

pub async fn get_erc20_info(
    State(state): State<AppState>,
    Path(address_str): Path<String>,
) -> Result<Json<Erc20InfoResponse>, HttpError> {
    let address: Address = address_str
        .parse()
        .map_err(|_| map_error(ApiError::InvalidAddress))?;

    let contract = ERC20::new(address, state.provider.clone());

    let name = contract
        .name()
        .call()
        .await
        .map_err(|e| map_error(ApiError::Contract(e.to_string())))?;

    let symbol = contract
        .symbol()
        .call()
        .await
        .map_err(|e| map_error(ApiError::Contract(e.to_string())))?;

    let decimals = contract
        .decimals()
        .call()
        .await
        .map_err(|e| map_error(ApiError::Contract(e.to_string())))?;

    let total_supply = contract
        .total_supply()
        .call()
        .await
        .map_err(|e| map_error(ApiError::Contract(e.to_string())))?;

    let resp = Erc20InfoResponse {
        address: address_str,
        name,
        symbol,
        decimals,
        total_supply: total_supply.to_string(),
    };

    Ok(Json(resp))
}
