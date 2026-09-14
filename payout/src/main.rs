mod payout;
mod currency;

use payout::{PaidIn, PayOut, greedy_min_cash_flow};
use std::collections::HashMap;

use axum::{Json, Router, routing::get, routing::post};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn hello_world() -> &'static str {
    "root"
}

async fn create_payout(Json(payload): Json<HashMap<String, f32>>) -> Json<PayOut> {
    let payouts: Vec<PaidIn> = payload
        .into_iter()
        .map(|(username, amount)| PaidIn { username, amount })
        .collect();
    Json(greedy_min_cash_flow(payouts))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting axum server...");
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/payout", post(create_payout))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    info!("Server listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
