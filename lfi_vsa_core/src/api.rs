// ============================================================
// LFI Web API — Hardened REST Interface
// Section 3: "The Backend Daemon (axum / Rust): The lfi_vsa_core
// runs as a headless service exposing a hardened REST API."
// ============================================================

use axum::{
    routing::{get, post},
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Serialize, Deserialize};
use crate::agent::LfiAgent;
use crate::debuglog;
use std::net::SocketAddr;
use std::sync::Arc;

/// API Response for agent status.
#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub version: String,
    pub active_axioms: usize,
}

/// Request payload for task execution.
#[derive(Deserialize)]
pub struct TaskRequest {
    pub task_name: String,
}

/// Shared state for the API server. Must be Clone + Send + Sync.
#[derive(Clone)]
pub struct ApiState {
    pub agent: Arc<LfiAgent>,
}

/// Initializes and starts the axum API server.
pub async fn start_api_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    debuglog!("start_api_server: initializing server on {}", addr);

    let agent = Arc::new(LfiAgent::new().map_err(|e| format!("Agent init failed: {:?}", e))?);
    let state = ApiState { agent };

    let app = Router::new()
        .route("/status", get(get_status))
        .route("/task", post(execute_task))
        .with_state(state);

    debuglog!("start_api_server: routes configured, listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_status(
    State(state): State<ApiState>,
) -> Json<StatusResponse> {
    debuglog!("API: GET /status");
    Json(StatusResponse {
        status: "Operational".to_string(),
        version: "5.6.4".to_string(),
        active_axioms: state.agent.supervisor.axiom_count(),
    })
}

async fn execute_task(
    State(state): State<ApiState>,
    Json(payload): Json<TaskRequest>,
) -> impl IntoResponse {
    debuglog!("API: POST /task '{}'", payload.task_name);

    match state.agent.execute_task(&payload.task_name) {
        Ok(_) => {
            debuglog!("API: POST /task '{}' succeeded", payload.task_name);
            (StatusCode::OK, Json(format!("Task '{}' executed and audited successfully.", payload.task_name)))
        }
        Err(e) => {
            debuglog!("API: POST /task '{}' FAILED: {:?}", payload.task_name, e);
            (StatusCode::UNPROCESSABLE_ENTITY, Json(format!("Task '{}' failed forensic audit: {:?}", payload.task_name, e)))
        }
    }
}
