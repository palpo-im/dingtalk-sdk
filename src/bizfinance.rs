//! DingTalk API bindings for the bizfinance module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn bizfinance_list(&self, access_token: &str) -> Result<BizfinanceListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/bizfinance/list", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn bizfinance_append_role_permission(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::PUT,
            "/v1.0/bizfinance/roles/permissions",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_batch_add_invoice(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/invoices/batch",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_batch_create_customer(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/auxiliaries/batch",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_begin_consume(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/consumedBenefits/prepare",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_cancel_consume(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/consumedBenefits/cancel",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_check_voucher_status(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/invoices/checkVoucherStatus/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_commit_consume(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/consumedBenefits/commit",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_create_receipt(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/receipts",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_delete_receipt(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/receipts/remove",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }

    /// Executes this API call.
    pub async fn bizfinance_query_permission_role_member(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/bizfinance/roles/members/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("bizfinance"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct BizfinanceListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
