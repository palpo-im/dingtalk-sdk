//! DingTalk API bindings for the conference module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn conference_create(
        &self,
        access_token: &str,
        request: &ConferenceCreateRequest,
    ) -> Result<ConferenceCreateResponse> {
        self.post("/topapi/conference/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn conference_get(
        &self,
        access_token: &str,
        conference_id: &str,
    ) -> Result<ConferenceInfo> {
        let body = serde_json::json!({ "conference_id": conference_id });
        self.post("/topapi/conference/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn conference_update(
        &self,
        access_token: &str,
        request: &ConferenceUpdateRequest,
    ) -> Result<()> {
        self.post("/topapi/conference/update", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn conference_cancel(&self, access_token: &str, conference_id: &str) -> Result<()> {
        let body = serde_json::json!({ "conference_id": conference_id });
        self.post("/topapi/conference/cancel", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn conference_list(
        &self,
        access_token: &str,
        request: &ConferenceListRequest,
    ) -> Result<ConferenceListResponse> {
        self.post("/topapi/conference/list", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceCreateRequest {
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<ConferenceAttendee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminder: Option<ConferenceReminder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceAttendee {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceReminder {
    pub minutes: i32,
    #[serde(rename = "type")]
    pub reminder_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceCreateResponse {
    pub conference_id: String,
    pub join_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceInfo {
    pub conference_id: String,
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendees: Option<Vec<ConferenceAttendee>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceUpdateRequest {
    pub conference_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceListRequest {
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConferenceListResponse {
    pub conferences: Vec<ConferenceInfo>,
    pub has_more: bool,
    pub next_cursor: i64,
}
