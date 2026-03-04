//! DingTalk API bindings for the customer_service module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn customer_service_send_message(
        &self,
        access_token: &str,
        request: &CustomerServiceSendMessageRequest,
    ) -> Result<CustomerServiceSendMessageResponse> {
        self.post(
            "/topapi/customer_service/message/send",
            access_token,
            request,
        )
        .await
    }

    /// Executes this API call.
    pub async fn customer_service_get_message(
        &self,
        access_token: &str,
        msg_id: &str,
    ) -> Result<CustomerServiceMessage> {
        let body = serde_json::json!({ "msg_id": msg_id });
        self.post("/topapi/customer_service/message/get", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CustomerServiceSendMessageRequest {
    pub conversation_id: String,
    pub msg_type: String,
    pub content: CustomerServiceContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CustomerServiceContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CustomerServiceSendMessageResponse {
    pub msg_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct CustomerServiceMessage {
    pub msg_id: String,
    pub conversation_id: String,
    pub msg_type: String,
    pub content: CustomerServiceContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}
