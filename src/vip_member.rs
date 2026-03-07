//! DingTalk API bindings for the vip_member module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn vip_member_list(&self, access_token: &str) -> Result<VipMemberListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/vip_member/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct VipMemberListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
