//! DingTalk API bindings for the badge module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn badge_create(
        &self,
        access_token: &str,
        request: &BadgeCreateRequest,
    ) -> Result<BadgeCreateResponse> {
        self.post("/topapi/badge/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn badge_get(&self, access_token: &str, badge_id: &str) -> Result<Badge> {
        let body = serde_json::json!({ "badge_id": badge_id });
        self.post("/topapi/badge/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn badge_list(&self, access_token: &str) -> Result<BadgeListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/badge/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BadgeCreateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BadgeCreateResponse {
    pub badge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Badge {
    pub badge_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BadgeListResponse {
    pub badges: Vec<Badge>,
}
