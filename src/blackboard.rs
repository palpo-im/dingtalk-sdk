//! DingTalk API bindings for the blackboard module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn blackboard_create(
        &self,
        access_token: &str,
        request: &BlackboardCreateRequest,
    ) -> Result<BlackboardCreateResponse> {
        self.post("/topapi/blackboard/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn blackboard_get(
        &self,
        access_token: &str,
        blackboard_id: &str,
    ) -> Result<Blackboard> {
        let body = serde_json::json!({ "blackboard_id": blackboard_id });
        self.post("/topapi/blackboard/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn blackboard_list(&self, access_token: &str) -> Result<BlackboardListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/blackboard/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BlackboardCreateRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BlackboardCreateResponse {
    pub blackboard_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Blackboard {
    pub blackboard_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BlackboardListResponse {
    pub blackboards: Vec<Blackboard>,
}
