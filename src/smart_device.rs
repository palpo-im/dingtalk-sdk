//! DingTalk API bindings for the smart_device module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn smart_device_list(&self, access_token: &str) -> Result<SmartDeviceListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/smart_device/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct SmartDeviceListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
