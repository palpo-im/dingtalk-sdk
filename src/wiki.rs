//! DingTalk API bindings for the wiki module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn wiki_create_space(
        &self,
        access_token: &str,
        request: &WikiCreateSpaceRequest,
    ) -> Result<WikiCreateSpaceResponse> {
        self.post("/topapi/wiki/space/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn wiki_get_space(&self, access_token: &str, space_id: &str) -> Result<WikiSpace> {
        let body = serde_json::json!({ "space_id": space_id });
        self.post("/topapi/wiki/space/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn wiki_list_spaces(
        &self,
        access_token: &str,
        cursor: i64,
        size: i64,
    ) -> Result<WikiListSpacesResponse> {
        let body = serde_json::json!({ "cursor": cursor, "size": size });
        self.post("/topapi/wiki/space/list", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn wiki_create_doc(
        &self,
        access_token: &str,
        request: &WikiCreateDocRequest,
    ) -> Result<WikiCreateDocResponse> {
        self.post("/topapi/wiki/doc/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn wiki_get_doc(&self, access_token: &str, doc_id: &str) -> Result<WikiDoc> {
        let body = serde_json::json!({ "doc_id": doc_id });
        self.post("/topapi/wiki/doc/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn wiki_list_docs(
        &self,
        access_token: &str,
        space_id: &str,
        cursor: i64,
        size: i64,
    ) -> Result<WikiListDocsResponse> {
        let body = serde_json::json!({ "space_id": space_id, "cursor": cursor, "size": size });
        self.post("/topapi/wiki/doc/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiCreateSpaceRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiCreateSpaceResponse {
    pub space_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiSpace {
    pub space_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiListSpacesResponse {
    pub list: Vec<WikiSpace>,
    pub has_more: bool,
    pub next_cursor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiCreateDocRequest {
    pub space_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiCreateDocResponse {
    pub doc_id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiDoc {
    pub doc_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct WikiListDocsResponse {
    pub list: Vec<WikiDoc>,
    pub has_more: bool,
    pub next_cursor: i64,
}
