use common::telemetry::init_tracing;
use common::utils::SOLANA_RPC_URL;
use solana::{Application, app_state::AppState};

#[tokio::main]
async fn main() {
    init_tracing("Solana api");

    let app_state = AppState::new(SOLANA_RPC_URL.as_str());

    let addr = "0.0.0.0:8081";
    let app = Application::build(app_state, addr)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
// curl http://localhost:8081/contracts/TB1Dqt8JeKQh7RLDzfYDJsq8KS4fS2yt87avRjyRxMv/program_accounts
// curl http://localhost:8081/account//balance
