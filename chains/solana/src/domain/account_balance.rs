use serde::Serialize;

#[derive(Serialize)]
pub struct AccountBalance {
    pub pubkey: String,
    pub balance_lamport: u64,
}
