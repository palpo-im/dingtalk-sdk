//! DingTalk API bindings for the alitrip module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn alitrip_list(&self, access_token: &str) -> Result<AlitripListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/alitrip/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AlitripListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
