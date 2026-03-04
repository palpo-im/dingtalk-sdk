//! DingTalk API bindings for the miniapp module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn miniapp_upload_code(
        &self,
        access_token: &str,
        request: &MiniappUploadCodeRequest,
    ) -> Result<MiniappUploadCodeResponse> {
        self.post("/topapi/miniprogram/upload", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn miniapp_get_version(
        &self,
        access_token: &str,
        app_id: &str,
    ) -> Result<MiniappVersionResponse> {
        let body = serde_json::json!({ "app_id": app_id });
        self.post("/topapi/miniprogram/version/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn miniapp_get_qrcode(
        &self,
        access_token: &str,
        request: &MiniappQrcodeRequest,
    ) -> Result<MiniappQrcodeResponse> {
        self.post("/topapi/miniprogram/qrcode/get", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn miniapp_list(&self, access_token: &str) -> Result<MiniappListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/miniprogram/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappUploadCodeRequest {
    pub app_id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappUploadCodeResponse {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappVersionResponse {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappQrcodeRequest {
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappQrcodeResponse {
    pub qrcode_url: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappListResponse {
    pub list: Vec<MiniappInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MiniappInfo {
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
