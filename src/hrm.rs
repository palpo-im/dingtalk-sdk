//! DingTalk API bindings for the hrm module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn hrm_get_employee(&self, access_token: &str, user_id: &str) -> Result<HrmEmployee> {
        let body = serde_json::json!({ "userid": user_id });
        self.post("/topapi/smartwork/hrm/employee/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn hrm_create_employee(
        &self,
        access_token: &str,
        employee: &HrmCreateEmployeeRequest,
    ) -> Result<HrmCreateEmployeeResponse> {
        self.post("/topapi/smartwork/hrm/employee/add", access_token, employee)
            .await
    }

    /// Executes this API call.
    pub async fn hrm_update_employee(
        &self,
        access_token: &str,
        employee: &HrmUpdateEmployeeRequest,
    ) -> Result<()> {
        self.post(
            "/topapi/smartwork/hrm/employee/update",
            access_token,
            employee,
        )
        .await
    }

    /// Executes this API call.
    pub async fn hrm_delete_employee(&self, access_token: &str, user_id: &str) -> Result<()> {
        let body = serde_json::json!({ "userid": user_id });
        self.post("/topapi/smartwork/hrm/employee/delete", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn hrm_get_field_list(&self, access_token: &str) -> Result<HrmFieldListResponse> {
        let body = serde_json::json!({});
        self.post(
            "/topapi/smartwork/hrm/employee/field/list",
            access_token,
            &body,
        )
        .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmEmployee {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiring_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probation_period_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_join_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regular_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmCreateEmployeeRequest {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiring_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmCreateEmployeeResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HrmEmployee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmUpdateEmployeeRequest {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiring_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmFieldListResponse {
    pub field_list: Vec<HrmField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct HrmField {
    pub field_id: String,
    pub field_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
