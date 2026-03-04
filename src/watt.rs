//! DingTalk API bindings for the watt module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn watt_list(&self, access_token: &str) -> Result<WattListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/watt/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WattListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
