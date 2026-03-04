//! DingTalk API bindings for the content module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn content_create(
        &self,
        access_token: &str,
        request: &ContentCreateRequest,
    ) -> Result<ContentCreateResponse> {
        self.post("/topapi/content/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn content_get(&self, access_token: &str, content_id: &str) -> Result<Content> {
        let body = serde_json::json!({ "content_id": content_id });
        self.post("/topapi/content/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn content_list(
        &self,
        access_token: &str,
        request: &ContentListRequest,
    ) -> Result<ContentListResponse> {
        self.post("/topapi/content/list", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContentCreateRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContentCreateResponse {
    pub content_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Content {
    pub content_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContentListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContentListResponse {
    pub contents: Vec<Content>,
    pub has_more: bool,
}
