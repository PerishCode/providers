use axum::{
    body::Bytes,
    http::{HeaderMap, HeaderValue, header},
};
use reqwest::Client;
use tracing::error;

use provider_codex_auth::models::auth::OpenAiAuthEntry;
use crate::services::config::AppConfig;

#[derive(Debug, thiserror::Error)]
pub enum UpstreamError {
    #[error("failed to send upstream request: {0}")]
    Request(#[from] reqwest::Error),
}

#[derive(Debug)]
pub struct UpstreamResponse {
    pub status: reqwest::StatusCode,
    pub headers: HeaderMap,
    pub body: Bytes,
}

#[derive(Debug, Default)]
pub struct UpstreamService {
    client: Client,
}

impl UpstreamService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn post_responses(
        &self,
        config: &AppConfig,
        auth: &OpenAiAuthEntry,
        request_headers: &HeaderMap,
        request_body: Bytes,
    ) -> Result<UpstreamResponse, UpstreamError> {
        let headers = upstream_headers(auth, request_headers);

        let response = self
            .client
            .post(&config.codex_api_endpoint)
            .headers(headers)
            .body(request_body)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(error) => {
                error!(error = %error, endpoint = %config.codex_api_endpoint, "upstream request failed");
                return Err(UpstreamError::Request(error));
            }
        };

        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;

        Ok(UpstreamResponse {
            status,
            headers: sanitize_response_headers(&headers),
            body,
        })
    }
}

fn upstream_headers(auth: &OpenAiAuthEntry, original: &HeaderMap) -> reqwest::header::HeaderMap {
    let mut next = reqwest::header::HeaderMap::new();

    for (name, value) in original {
        if name == header::HOST || name == header::CONTENT_LENGTH || name == header::AUTHORIZATION {
            continue;
        }

        if let Ok(converted_name) =
            reqwest::header::HeaderName::from_bytes(name.as_str().as_bytes())
        {
            next.insert(converted_name, value.clone());
        }
    }

    next.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", auth.access))
            .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid")),
    );
    next.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("openai-codex-server/0.1.0"),
    );

    if let Some(account_id) = &auth.account_id {
        if let Ok(value) = HeaderValue::from_str(account_id) {
            next.insert(
                reqwest::header::HeaderName::from_static("chatgpt-account-id"),
                value,
            );
        }
    }

    next
}

fn sanitize_response_headers(original: &HeaderMap) -> HeaderMap {
    let mut next = HeaderMap::new();

    for (name, value) in original {
        if name == header::CONTENT_ENCODING || name == header::CONTENT_LENGTH {
            continue;
        }

        next.insert(name.clone(), value.clone());
    }

    next
}
