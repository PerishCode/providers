use axum::Json;

use crate::models::health::HealthResponse;

#[utoipa::path(
    get,
    path = "/openai/v1/health",
    tag = "openai-codex-server",
    responses((status = 200, description = "Service health status", body = HealthResponse))
)]
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse { ok: true })
}
