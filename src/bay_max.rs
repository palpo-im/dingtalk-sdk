//! DingTalk API bindings for the bay_max module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn bay_max_list(&self, access_token: &str) -> Result<BayMaxListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/bay_max/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BayMaxListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
