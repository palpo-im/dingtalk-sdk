//! DingTalk API bindings for the attendance_1_0 module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn attendance_1_0_get_attendance_list(
        &self,
        access_token: &str,
        request: &AttendanceListRequest,
    ) -> Result<AttendanceListResponse> {
        self.post("/attendance/list", access_token, request).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AttendanceListRequest {
    pub work_date_from: String,
    pub work_date_to: String,
    pub user_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AttendanceListResponse {
    pub recordresult: Vec<AttendanceRecord>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AttendanceRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_check_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_check_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_result: Option<String>,
}
