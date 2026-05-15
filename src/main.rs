use std::{collections::HashMap, sync::Arc};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    routing::get,
};

use crate::magical_square::{
    HashPosition, SOLUTION_NUMBER, get_moves_from_graph, get_path_from_index, make_graph,
};
pub mod magical_square;

#[tokio::main]
async fn main() {
    let graph = make_graph();
    let shared_state = Arc::new(graph);
    let app = Router::new()
        .route("/get_moves/{hash}", get(get_moves))
        .route("/get_path/{index}", get(get_path))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_moves(
    State(state): State<Arc<HashMap<u128, HashPosition>>>,
    Path(hash): Path<u128>,
) -> Response<String> {
    let moves = get_moves_from_graph(&state, hash);
    Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "max-age=604800")
        .body(format!("{:?}", moves))
        .unwrap()
}

async fn get_path(
    State(state): State<Arc<HashMap<u128, HashPosition>>>,
    Path(index): Path<u32>,
) -> Response<String> {
    // index shouldn't be higher nor equal to the number of solution
    if index >= SOLUTION_NUMBER {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("[]".to_string())
            .unwrap();
    }

    let path = get_path_from_index(&state, index);
    Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Cache-Control", "max-age=604800")
        .body(format!("{:?}", path))
        .unwrap()
}
