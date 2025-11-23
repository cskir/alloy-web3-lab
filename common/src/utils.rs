use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

lazy_static! {
    pub static ref SOLANA_RPC_URL: String = set_solana_rpc_url();
}

fn set_solana_rpc_url() -> String {
    dotenv().ok();
    let res = std_env::var(env::SOLANA_RPC_URL_ENV_VAR).expect("SOLANA_RPC_URL must be set.");
    if res.is_empty() {
        panic!("SOLANA_RPC_URL must not be empty.");
    }
    res
}

pub mod env {
    pub const SOLANA_RPC_URL_ENV_VAR: &str = "SOLANA_RPC_URL";
}
