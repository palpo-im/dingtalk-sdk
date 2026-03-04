//! DingTalk API bindings for the doc module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn doc_create(
        &self,
        access_token: &str,
        request: &DocCreateRequest,
    ) -> Result<DocCreateResponse> {
        self.post("/topapi/doc/create", access_token, request).await
    }

    /// Executes this API call.
    pub async fn doc_get(&self, access_token: &str, doc_id: &str) -> Result<DocInfo> {
        let body = serde_json::json!({ "doc_id": doc_id });
        self.post("/topapi/doc/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn doc_update(&self, access_token: &str, request: &DocUpdateRequest) -> Result<()> {
        self.post("/topapi/doc/update", access_token, request).await
    }

    /// Executes this API call.
    pub async fn doc_delete(&self, access_token: &str, doc_id: &str) -> Result<()> {
        let body = serde_json::json!({ "doc_id": doc_id });
        self.post("/topapi/doc/delete", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn doc_list(
        &self,
        access_token: &str,
        offset: i64,
        size: i64,
    ) -> Result<DocListResponse> {
        let body = serde_json::json!({ "offset": offset, "size": size });
        self.post("/topapi/doc/list", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn doc_add_comment_openapi(
        &self,
        access_token: &str,
        doc_id: &str,
        request: &DocAddCommentRequest,
    ) -> Result<DocAddCommentResponse> {
        let path = format!("/v1.0/doc/docs/{doc_id}/comments");
        let mut query = HashMap::new();
        query.insert("operatorId".to_string(), request.operator_id.clone());
        let body = serde_json::json!({
            "commentContent": request.comment_content,
            "commentType": request.comment_type,
            "option": request.option
        });
        self.request_openapi(Method::POST, &path, access_token, Some(&query), Some(&body))
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_list_comments_openapi(
        &self,
        access_token: &str,
        doc_id: &str,
        request: Option<&DocListCommentsRequest>,
    ) -> Result<DocListCommentsResponse> {
        let path = format!("/v1.0/doc/docs/{doc_id}/comments");
        let query = request
            .map(DocListCommentsRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.get_openapi(&path, access_token, query_ref)
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_add_workspace_doc_members(
        &self,
        access_token: &str,
        workspace_id: &str,
        node_id: &str,
        request: &DocWorkspaceMembersRequest,
    ) -> Result<()> {
        let path = format!("/v1.0/doc/workspaces/{workspace_id}/docs/{node_id}/members");
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_update_workspace_doc_members(
        &self,
        access_token: &str,
        workspace_id: &str,
        node_id: &str,
        request: &DocWorkspaceMembersRequest,
    ) -> Result<()> {
        let path = format!("/v1.0/doc/workspaces/{workspace_id}/docs/{node_id}/members");
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::PUT, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_delete_workspace_doc_members(
        &self,
        access_token: &str,
        workspace_id: &str,
        node_id: &str,
        request: &DocWorkspaceMembersRequest,
    ) -> Result<()> {
        let path = format!("/v1.0/doc/workspaces/{workspace_id}/docs/{node_id}/members/remove");
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_create_workspace_openapi(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.post_openapi("/v1.0/doc/workspaces", access_token, Some(request))
            .await
            .map_err(|err| err.map_module("doc"))
    }

    /// Executes this API call.
    pub async fn doc_create_workspace_doc_openapi(
        &self,
        access_token: &str,
        workspace_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1.0/doc/workspaces/{workspace_id}/docs");
        self.post_openapi(&path, access_token, Some(request))
            .await
            .map_err(|err| err.map_module("doc"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocCreateRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocCreateResponse {
    pub doc_id: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocInfo {
    pub doc_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub doc_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocUpdateRequest {
    pub doc_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocListResponse {
    pub list: Vec<DocInfo>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocAddCommentRequest {
    pub operator_id: String,
    pub comment_content: String,
    pub comment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocAddCommentResponse {
    #[serde(rename = "commentId", skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocListCommentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl DocListCommentsRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = &self.operator_id {
            params.insert("operatorId".to_string(), value.clone());
        }
        if let Some(value) = self.max_results {
            params.insert("maxResults".to_string(), value.to_string());
        }
        if let Some(value) = &self.next_token {
            params.insert("nextToken".to_string(), value.clone());
        }
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocListCommentsResponse {
    #[serde(default)]
    pub comments: Vec<serde_json::Value>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DocWorkspaceMembersRequest {
    pub members: Vec<serde_json::Value>,
    #[serde(rename = "operatorId")]
    pub operator_id: String,
}
