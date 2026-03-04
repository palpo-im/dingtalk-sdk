//! DingTalk API bindings for the report module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn report_create(
        &self,
        access_token: &str,
        request: &ReportCreateRequest,
    ) -> Result<ReportCreateResponse> {
        self.post("/topapi/report/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn report_get(&self, access_token: &str, report_id: &str) -> Result<ReportRecord> {
        let body = serde_json::json!({ "report_id": report_id });
        self.post("/topapi/report/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn report_list(
        &self,
        access_token: &str,
        request: &ReportListRequest,
    ) -> Result<ReportListResponse> {
        self.post("/topapi/report/list", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn report_get_templates(
        &self,
        access_token: &str,
    ) -> Result<ReportTemplateListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/report/template/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportCreateRequest {
    pub template_id: String,
    pub user_id: String,
    pub contents: Vec<ReportContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportContent {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportCreateResponse {
    pub report_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportRecord {
    pub report_id: String,
    pub template_id: String,
    pub user_id: String,
    pub create_time: i64,
    pub contents: Vec<ReportContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportListRequest {
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportListResponse {
    pub records: Vec<ReportRecord>,
    pub has_more: bool,
    pub next_cursor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportTemplateListResponse {
    pub templates: Vec<ReportTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ReportTemplate {
    pub template_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}
