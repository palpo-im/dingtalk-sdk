//! DingTalk API bindings for the yida module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn yida_list(&self, access_token: &str) -> Result<YidaListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/yida/list", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn yida_app_login_code_gen(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/authorizations/appLoginCodes",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_batch_get_form_data_by_id_list(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/forms/instances/ids/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_batch_removal_by_form_instance_id_list(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/forms/instances/batchRemove",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_batch_save_form_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/forms/instances/batchSave",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_batch_update_form_data_by_instance_id(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::PUT,
            "/v1.0/yida/forms/instances/components",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_batch_update_form_data_by_instance_map(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::PUT,
            "/v1.0/yida/forms/instances/datas",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_create_or_update_form_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/forms/instances/insertOrUpdate",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_execute_batch_task(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/tasks/batches/execute",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_execute_platform_task(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/tasks/platformTasks/execute",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }

    /// Executes this API call.
    pub async fn yida_execute_task(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/yida/tasks/execute",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("yida"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct YidaListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
