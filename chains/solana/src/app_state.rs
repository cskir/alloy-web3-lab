use std::sync::Arc;

use solana_client::rpc_client::RpcClient;

#[derive(Clone)]
pub struct AppState {
    pub rpc: Arc<RpcClient>,
}

impl AppState {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        let client = RpcClient::new(rpc_url.into());
        Self {
            rpc: Arc::new(client),
        }
    }
}
