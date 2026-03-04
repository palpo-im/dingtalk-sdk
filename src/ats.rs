//! DingTalk API bindings for the ats module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn ats_create_job(
        &self,
        access_token: &str,
        request: &AtsCreateJobRequest,
    ) -> Result<AtsCreateJobResponse> {
        self.post("/topapi/ats/job/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn ats_get_job(&self, access_token: &str, job_id: &str) -> Result<AtsJob> {
        let body = serde_json::json!({ "job_id": job_id });
        self.post("/topapi/ats/job/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn ats_list_jobs(
        &self,
        access_token: &str,
        request: &AtsListJobsRequest,
    ) -> Result<AtsListJobsResponse> {
        self.post("/topapi/ats/job/list", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn ats_update_job(
        &self,
        access_token: &str,
        request: &AtsUpdateJobRequest,
    ) -> Result<()> {
        self.post("/topapi/ats/job/update", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn ats_delete_job(&self, access_token: &str, job_id: &str) -> Result<()> {
        let body = serde_json::json!({ "job_id": job_id });
        self.post("/topapi/ats/job/delete", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsCreateJobRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsCreateJobResponse {
    pub job_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsJob {
    pub job_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsListJobsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsListJobsResponse {
    pub jobs: Vec<AtsJob>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AtsUpdateJobRequest {
    pub job_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
