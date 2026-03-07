//! DingTalk API bindings for the org_culture module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn org_culture_list(&self, access_token: &str) -> Result<OrgCultureListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/org_culture/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OrgCultureListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
