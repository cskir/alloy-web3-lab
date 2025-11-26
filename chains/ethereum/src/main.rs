use common::telemetry::init_tracing;
use common::utils::ETH_RPC_URL;
use etheterum::{Application, app_state::AppState};

#[tokio::main]
async fn main() {
    init_tracing("Ethereum api");

    let app_state = AppState::new(ETH_RPC_URL.as_str());

    let addr = "0.0.0.0:8082";
    let app = Application::build(app_state, addr)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

//http://localhost:8082/contracts/0x505b5eda5e25a67e1c24a2bf1a527ed9eb88bf04/erc20-info
