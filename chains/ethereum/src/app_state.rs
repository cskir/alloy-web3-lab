use ethers::providers::{Http, Provider};
use std::sync::Arc;

pub type EthProvider = Provider<Http>;

#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<EthProvider>,
}

impl AppState {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        let provider =
            Provider::<Http>::try_from(rpc_url.into()).expect("Failed to create Ethereum provider");
        Self {
            provider: Arc::new(provider),
        }
    }
}
