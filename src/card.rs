//! DingTalk API bindings for the card module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn card_create(
        &self,
        access_token: &str,
        request: &CardCreateRequest,
    ) -> Result<CardCreateResponse> {
        self.post("/topapi/card/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn card_get(&self, access_token: &str, card_id: &str) -> Result<Card> {
        let body = serde_json::json!({ "card_id": card_id });
        self.post("/topapi/card/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn card_update(&self, access_token: &str, request: &CardUpdateRequest) -> Result<()> {
        self.post("/topapi/card/update", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn card_delete(&self, access_token: &str, card_id: &str) -> Result<()> {
        let body = serde_json::json!({ "card_id": card_id });
        self.post("/topapi/card/delete", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CardCreateRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CardCreateResponse {
    pub card_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Card {
    pub card_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CardUpdateRequest {
    pub card_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
