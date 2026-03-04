//! DingTalk API bindings for the connector module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn connector_create(
        &self,
        access_token: &str,
        request: &ConnectorCreateRequest,
    ) -> Result<ConnectorCreateResponse> {
        self.post("/topapi/connector/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn connector_get(&self, access_token: &str, connector_id: &str) -> Result<Connector> {
        let body = serde_json::json!({ "connector_id": connector_id });
        self.post("/topapi/connector/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn connector_list(&self, access_token: &str) -> Result<ConnectorListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/connector/list", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConnectorCreateRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConnectorCreateResponse {
    pub connector_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Connector {
    pub connector_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ConnectorListResponse {
    pub connectors: Vec<Connector>,
}
