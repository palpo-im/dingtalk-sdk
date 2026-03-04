//! DingTalk API bindings for the flashmsg module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn flashmsg_send(
        &self,
        access_token: &str,
        request: &FlashMsgSendRequest,
    ) -> Result<FlashMsgSendResponse> {
        self.post("/topapi/flashmsg/send", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn flashmsg_get_status(
        &self,
        access_token: &str,
        msg_id: &str,
    ) -> Result<FlashMsgStatus> {
        let body = serde_json::json!({ "msg_id": msg_id });
        self.post("/topapi/flashmsg/status", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct FlashMsgSendRequest {
    pub agent_id: i64,
    pub user_ids: Vec<String>,
    pub content: FlashMsgContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct FlashMsgContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct FlashMsgSendResponse {
    pub task_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct FlashMsgStatus {
    pub status: String,
    pub send_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
}
