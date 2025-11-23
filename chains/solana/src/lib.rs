use std::error::Error;
pub mod app_state;
use app_state::AppState;
use axum::{Router, routing::get, serve::Serve};
use common::telemetry::{make_span_with_request_id, on_request, on_response};
use tower_http::trace::TraceLayer;

pub mod domain;
pub mod error;
pub mod routes;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .route(
                "/contracts/:program_id/program_accounts",
                get(routes::get_program_accounts),
            )
            .route("/account/:account_pubkey/balance", get(routes::get_balance))
            .with_state(app_state)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}
