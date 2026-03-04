//! DingTalk API bindings for the exclusive module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn exclusive_list(&self, access_token: &str) -> Result<ExclusiveListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/exclusive/list", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn exclusive_query_benefits(&self, access_token: &str) -> Result<serde_json::Value> {
        self.get_openapi("/v1.0/exclusive/benefits", access_token, None)
            .await
            .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_get_conversation_categories(
        &self,
        access_token: &str,
    ) -> Result<ExclusiveConversationCategoriesResponse> {
        self.get_openapi("/v1.0/exclusive/conversationCategories", access_token, None)
            .await
            .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_set_conversation_category(
        &self,
        access_token: &str,
        request: &SetExclusiveConversationCategoryRequest,
    ) -> Result<()> {
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/exclusive/conversationCategories/set",
            access_token,
            None,
            Some(&body),
        )
        .await
        .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_send_message(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/exclusive/follow/message/send",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_query_trusted_devices(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<ExclusiveTrustedDevicesResponse> {
        self.request_openapi(
            Method::POST,
            "/v1.0/exclusive/trustedDevices/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_create_trusted_device(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/exclusive/trustedDevices",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_update_trusted_device(
        &self,
        access_token: &str,
        device_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1.0/exclusive/trustedDevices/{device_id}");
        self.request_openapi(Method::PUT, &path, access_token, None, Some(request))
            .await
            .map_err(|err| err.map_module("exclusive"))
    }

    /// Executes this API call.
    pub async fn exclusive_delete_trusted_device(
        &self,
        access_token: &str,
        request: &DeleteExclusiveTrustedDeviceRequest,
    ) -> Result<()> {
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/exclusive/trustedDevices/remove",
            access_token,
            None,
            Some(&body),
        )
        .await
        .map_err(|err| err.map_module("exclusive"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ExclusiveListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct SetExclusiveConversationCategoryRequest {
    #[serde(rename = "openConversationId")]
    pub open_conversation_id: String,
    #[serde(rename = "categoryId")]
    pub category_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ExclusiveConversationCategoriesResponse {
    #[serde(default)]
    pub categories: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ExclusiveTrustedDevicesResponse {
    #[serde(default)]
    pub devices: Vec<serde_json::Value>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DeleteExclusiveTrustedDeviceRequest {
    #[serde(rename = "deviceIds")]
    pub device_ids: Vec<String>,
}
