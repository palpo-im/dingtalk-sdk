//! DingTalk API bindings for the ding_one module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn ding_one_list(&self, access_token: &str) -> Result<DingOneListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/ding_one/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DingOneListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
