//! DingTalk API bindings for the team_sphere module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn team_sphere_list(&self, access_token: &str) -> Result<Team_sphereListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/team_sphere/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Team_sphereListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
