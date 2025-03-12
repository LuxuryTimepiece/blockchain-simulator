use axum::{
    routing::{get, post},
    Json, Router, extract::State,
};
use log::info;
use simple_logger::SimpleLogger;
use tower_http::services::ServeDir;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

mod block;
mod blockchain;

use crate::block::Block;
use blockchain::Blockchain;

#[derive(Deserialize)]
struct AddBlockPayload {
    data: String,
}

async fn get_blocks(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<Vec<Block>> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.get_chain().clone())
}

async fn add_block(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    Json(payload): Json<AddBlockPayload>,
) -> Json<Block> {
    let mut blockchain = blockchain.lock().await;
    let block = blockchain.mine_block(payload.data);
    Json(block)
}

async fn validate(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<bool> {
    let blockchain = blockchain.lock().await;
    Json(blockchain.is_valid())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();
    info!("Server starting on http://localhost:3000...");

    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    let app = Router::new()
        .nest_service("/", ServeDir::new("dist"))
        .route("/api/blocks", get(get_blocks))
        .route("/api/blocks", post(add_block))
        .route("/api/validate", get(validate))
        .with_state(blockchain);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}