use std::sync::Arc;

use provider_codex_auth::{AuthConfig, AuthService};

use crate::services::{config::AppConfig, upstream::UpstreamService};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub auth: Arc<AuthService>,
    pub upstream: Arc<UpstreamService>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Self {
        let auth_config = AuthConfig {
            auth_file: config.auth_file.clone(),
            openai_client_id: config.openai_client_id.clone(),
            openai_issuer: config.openai_issuer.clone(),
        };
        let auth = Arc::new(AuthService::new(auth_config));

        Self {
            config,
            auth,
            upstream: Arc::new(UpstreamService::new()),
        }
    }
}