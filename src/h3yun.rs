//! DingTalk API bindings for the h3yun module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn h3yun_list(&self, access_token: &str) -> Result<H3yunListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/h3yun/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct H3yunListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
