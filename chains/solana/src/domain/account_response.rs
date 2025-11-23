use serde::Serialize;

#[derive(Serialize)]
pub struct AccountResponse {
    pub pubkey: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub data_len: usize,
}
