//! DingTalk API bindings for the industry module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn industry_list(&self, access_token: &str) -> Result<IndustryListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/industry/list", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn industry_ai_retail_product_add(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/retail/product/add",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_ai_retail_product_query(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/retail/product/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_ai_retail_product_update(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/retail/product/update",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_ai_retail_product_delete(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/retail/product/delete",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_ai_retail_product_image_upload(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/retail/product/image/upload",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_chatai_sentiment_query(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/chatai/abilities/sentiments/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_chatmemo_faq_add(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/chatmemo/faq",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_chatmemo_faq_list(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/chatmemo/faq/lists",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_chatmemo_faq_remove(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/chatmemo/faq/remove",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }

    /// Executes this API call.
    pub async fn industry_task_queue_query(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/industry/ai/taskQueue/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("industry"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct IndustryListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
