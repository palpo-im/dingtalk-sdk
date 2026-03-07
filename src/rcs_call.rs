//! DingTalk API bindings for the rcs_call module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn rcs_call_list(&self, access_token: &str) -> Result<RcsCallListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/rcs_call/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct RcsCallListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
