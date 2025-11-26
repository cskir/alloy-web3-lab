use serde::Serialize;

#[derive(Serialize)]
pub struct Erc20InfoResponse {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
}
