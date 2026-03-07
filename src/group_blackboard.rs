//! DingTalk API bindings for the group_blackboard module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn group_blackboard_list(
        &self,
        access_token: &str,
    ) -> Result<GroupBlackboardListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/group_blackboard/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct GroupBlackboardListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
