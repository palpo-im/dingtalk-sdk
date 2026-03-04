//! DingTalk Check-in OpenAPI (v1.0) endpoints.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCheckinRecordByUserRequest {
    #[serde(rename = "endTime")]
    pub end_time: i64,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    #[serde(rename = "nextToken")]
    pub next_token: i64,
    #[serde(rename = "operatorUserId")]
    pub operator_user_id: String,
    #[serde(rename = "startTime")]
    pub start_time: i64,
    #[serde(rename = "userIdList")]
    pub user_id_list: Vec<String>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCheckinRecordByUserResponse {
    pub result: GetCheckinRecordByUserResult,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCheckinRecordByUserResult {
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<i64>,
    #[serde(rename = "pageList", default)]
    pub page_list: Vec<serde_json::Value>,
}

impl DingTalkClient {
    /// Executes this API call.
    pub async fn check_in_get_checkin_record_by_user(
        &self,
        access_token: &str,
        request: &GetCheckinRecordByUserRequest,
    ) -> Result<GetCheckinRecordByUserResponse> {
        let body = serde_json::to_value(request)?;
        self.post_openapi("/v1.0/checkIn/records/query", access_token, Some(&body))
            .await
    }
}
