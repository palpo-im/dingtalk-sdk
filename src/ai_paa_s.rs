//! DingTalk API bindings for the ai_paa_s module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn ai_paas_list(&self, access_token: &str) -> Result<AiPaasListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/ai_paa_s/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AiPaasListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
