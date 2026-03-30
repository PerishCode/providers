use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorEnvelope {
    pub error: ApiErrorBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorBody {
    pub message: String,
    pub r#type: String,
}

impl ApiErrorEnvelope {
    pub fn invalid_api_key() -> Self {
        Self {
            error: ApiErrorBody {
                message: "invalid api key".to_string(),
                r#type: "invalid_api_key".to_string(),
            },
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            error: ApiErrorBody {
                message: message.into(),
                r#type: "forbidden".to_string(),
            },
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            error: ApiErrorBody {
                message: message.into(),
                r#type: "not_found".to_string(),
            },
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        Self {
            error: ApiErrorBody {
                message: message.into(),
                r#type: "server_error".to_string(),
            },
        }
    }
}
