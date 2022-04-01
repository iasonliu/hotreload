use std::{net::SocketAddr, sync::Arc};

use arc_swap_live::ServerConfig;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = ServerConfig::load().await;
    let params = Arc::new(config.params);
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/reload", post(reload_handler))
        .layer(Extension(params));

    let addr: SocketAddr = config.network.into();
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> impl IntoResponse {
    "Hello, world!"
}

async fn reload_handler() -> impl IntoResponse {
    "reloading..."
}
