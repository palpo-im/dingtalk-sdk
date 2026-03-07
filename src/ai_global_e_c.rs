//! DingTalk API bindings for the ai_global_e_c module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn ai_global_ec_list(&self, access_token: &str) -> Result<AiGlobalEcListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/ai_global_e_c/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AiGlobalEcListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
