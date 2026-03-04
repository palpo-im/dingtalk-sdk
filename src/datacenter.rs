//! DingTalk API bindings for the datacenter module.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn datacenter_list(&self, access_token: &str) -> Result<DatacenterListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/datacenter/list", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn datacenter_query_active_user_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/activeUserData",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_attendance_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/attendanceData",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_report_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/reportData",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_chart_data(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/chartDatas/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_create_screen(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/screens",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_screen(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/screens",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_general_data_service(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/generalDataServices",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }

    /// Executes this API call.
    pub async fn datacenter_query_total_data_count(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.request_openapi(
            Method::POST,
            "/v1.0/datacenter/datas/totalCounts/query",
            access_token,
            None,
            Some(request),
        )
        .await
        .map_err(|err| err.map_module("datacenter"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DatacenterListResponse {
    pub list: Vec<serde_json::Value>,
    pub has_more: bool,
}
