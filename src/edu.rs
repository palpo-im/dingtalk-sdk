//! DingTalk API bindings for the edu module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn edu_list(&self, access_token: &str) -> Result<EduListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/edu/list", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn edu_activate_device(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/vpaas/devices/activate",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_add_device(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(Method::POST, "/v1.0/edu/devices", access_token, None, Some(request))
            .await
            .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_batch_create_cards(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(Method::POST, "/v1.0/edu/cards", access_token, None, Some(request))
            .await
            .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_card_batch_query_cards(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/cards/tasks/batch",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_card_end_card(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/cards/finish",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_check_restriction(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/restrictions/check",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_create_app_order(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/appOrders",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_create_order(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(Method::POST, "/v1.0/edu/orders", access_token, None, Some(request))
            .await
            .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_cancel_order(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/orders/cancel",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }

    /// Executes this API call.
    pub async fn edu_cancel_user_order(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/edu/userOrders/cancel",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("edu"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct EduListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
