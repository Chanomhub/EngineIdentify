mod engine_detector;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use engine_detector::{detect_engine, load_config, DetectionResult, EngineConfig};

struct AppState {
    config: Vec<EngineConfig>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Load config
    let config = load_config("engines.json").expect("Failed to load engines.json");
    let shared_state = Arc::new(AppState { config });

    let app = Router::new()
        .route("/", get(root))
        .route("/identify", post(identify_handler))
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Game Engine Identification API"
}

#[derive(Deserialize)]
struct IdentifyRequest {
    files: Vec<String>,
}

async fn identify_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<IdentifyRequest>,
) -> Json<DetectionResult> {
    let result = detect_engine(&payload.files, &state.config);
    Json(result)
}
