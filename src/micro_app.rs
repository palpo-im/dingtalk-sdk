//! DingTalk API bindings for the micro_app module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn micro_app_create(
        &self,
        access_token: &str,
        app: &MicroAppCreateRequest,
    ) -> Result<MicroAppCreateResponse> {
        self.post("/microapp/create", access_token, app).await
    }

    /// Executes this API call.
    pub async fn micro_app_update(
        &self,
        access_token: &str,
        app: &MicroAppUpdateRequest,
    ) -> Result<()> {
        self.post("/microapp/update", access_token, app).await
    }

    /// Executes this API call.
    pub async fn micro_app_list(&self, access_token: &str) -> Result<MicroAppListResponse> {
        let body = serde_json::json!({});
        self.post("/microapp/list", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn micro_app_get(&self, access_token: &str, agent_id: i64) -> Result<MicroAppInfo> {
        let body = serde_json::json!({ "agent_id": agent_id });
        self.post("/microapp/get", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MicroAppCreateRequest {
    pub app_name: String,
    pub app_icon: String,
    pub app_desc: String,
    pub homepage_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oss_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MicroAppCreateResponse {
    pub agent_id: i64,
    pub app_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MicroAppUpdateRequest {
    pub agent_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_homepage_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MicroAppListResponse {
    pub app_list: Vec<MicroAppInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MicroAppInfo {
    pub agent_id: i64,
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage_url: Option<String>,
}
