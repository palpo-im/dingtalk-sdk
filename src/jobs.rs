//! DingTalk API bindings for the jobs module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn jobs_list(&self, access_token: &str) -> Result<JobsListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/jobs/list", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct JobsListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
