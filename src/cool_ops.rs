//! DingTalk API bindings for the cool_ops module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn cool_ops_list(&self, access_token: &str) -> Result<Cool_opsListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/cool_ops/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Cool_opsListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
