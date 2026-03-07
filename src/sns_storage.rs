//! DingTalk API bindings for the sns_storage module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn sns_storage_list(&self, access_token: &str) -> Result<SnsStorageListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/sns_storage/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct SnsStorageListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
