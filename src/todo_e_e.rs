//! DingTalk API bindings for the todo_e_e module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn todo_ee_list(&self, access_token: &str) -> Result<TodoEeListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/todo_e_e/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct TodoEeListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
