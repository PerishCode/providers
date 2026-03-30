#[derive(Clone, Debug)]
pub struct AppConfig {
    pub port: u16,
    pub bootstrap_api_key: String,
    pub admin_api_key: String,
    pub sqlite_path: String,
    pub auth_file: String,
    pub openai_client_id: String,
    pub openai_issuer: String,
    pub codex_api_endpoint: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(8080);

        let bootstrap_api_key =
            std::env::var("API_KEY").unwrap_or_else(|_| "codex-local-dev".to_string());
        let admin_api_key =
            std::env::var("ADMIN_API_KEY").unwrap_or_else(|_| "codex-local-admin".to_string());
        let sqlite_path = std::env::var("SQLITE_PATH")
            .unwrap_or_else(|_| "./data/openai-codex-server.db".to_string());
        let auth_file = std::env::var("AUTH_FILE").unwrap_or_else(|_| "./auth.json".to_string());
        let openai_client_id = std::env::var("OPENAI_CLIENT_ID")
            .unwrap_or_else(|_| "app_EMoamEEZ73f0CkXaXp7hrann".to_string());
        let openai_issuer = std::env::var("OPENAI_ISSUER")
            .unwrap_or_else(|_| "https://auth.openai.com".to_string());
        let codex_api_endpoint = std::env::var("CODEX_API_ENDPOINT")
            .unwrap_or_else(|_| "https://chatgpt.com/backend-api/codex/responses".to_string());

        Self {
            port,
            bootstrap_api_key,
            admin_api_key,
            sqlite_path,
            auth_file,
            openai_client_id,
            openai_issuer,
            codex_api_endpoint,
        }
    }
}
