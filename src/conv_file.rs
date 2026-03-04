//! DingTalk API bindings for the conv_file module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn conv_file_list(&self, access_token: &str) -> Result<ConvFileListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/conv/file/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConvFileListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
