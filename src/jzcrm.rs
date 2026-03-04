//! DingTalk API bindings for the jzcrm module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn jzcrm_list(&self, access_token: &str) -> Result<JzcrmListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/jzcrm/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct JzcrmListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
