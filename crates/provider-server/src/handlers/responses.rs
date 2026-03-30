use axum::{
    body::Bytes,
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use tracing::{error, info};

use crate::{models::api_error::ApiErrorEnvelope, services::app_state::AppState};

#[utoipa::path(
    post,
    path = "/openai/v1/responses",
    tag = "openai-codex-server",
    request_body(
        content = crate::models::openai::ResponsesApiBody,
        description = "OpenAI-compatible responses payload"
    ),
    responses(
        (status = 200, description = "Upstream responses payload"),
        (status = 400, description = "Invalid request body", body = ApiErrorEnvelope),
        (status = 500, description = "Server error", body = ApiErrorEnvelope)
    )
)]
pub async fn create_response_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if serde_json::from_slice::<Value>(&body).is_err() {
        return (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(ApiErrorEnvelope::server_error(
                "request body must be valid json",
            )),
        )
            .into_response();
    }

    let auth = match state.auth.get_openai_auth().await {
        Ok(auth) => auth,
        Err(error) => {
            error!(error = %error, "failed to load openai auth");
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorEnvelope::server_error(error.to_string())),
            )
                .into_response();
        }
    };

    info!(endpoint = %state.config.codex_api_endpoint, "proxying responses request");

    match state
        .upstream
        .post_responses(&state.config, &auth, &headers, body)
        .await
    {
        Ok(upstream) => (upstream.status, upstream.headers, upstream.body).into_response(),
        Err(error) => {
            error!(error = %error, "failed to proxy responses request");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiErrorEnvelope::server_error(error.to_string())),
            )
                .into_response()
        }
    }
}
