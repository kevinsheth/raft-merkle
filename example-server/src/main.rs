use axum::{Router, routing::get};
use openraft::{Config, NodeId};

pub mod app;

pub async fn start_raft_node(node_id: NodeId, http_addr: String) -> std::io::Result<()> {
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

async fn health() -> &'static str {
    "Hello, World!"
}
