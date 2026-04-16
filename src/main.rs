use std::{collections::HashMap, sync::Arc};

use axum::{Router, extract::{Path, State}, routing::get};

use crate::magical_square::{HashPosition, get_moves_from_graph, make_graph};
pub mod magical_square;

#[tokio::main]
async fn main() {
    let graph = make_graph();
    let shared_state = Arc::new(graph);
    let app = Router::new()
        .route("/get_moves/{hash}", get(get_moves))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_moves(State(state): State<Arc<HashMap<u128, HashPosition>>>, Path(hash): Path<u128>) -> String {
    let moves = get_moves_from_graph(&state, hash);
    format!("{:?}", moves)
}
