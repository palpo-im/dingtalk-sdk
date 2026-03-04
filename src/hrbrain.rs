//! DingTalk API bindings for the hrbrain module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn hrbrain_list(&self, access_token: &str) -> Result<HrbrainListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/hrbrain/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrbrainListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
