//! DingTalk API bindings for the contract module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn contract_create(
        &self,
        access_token: &str,
        request: &ContractCreateRequest,
    ) -> Result<ContractCreateResponse> {
        self.post("/topapi/contract/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn contract_get(&self, access_token: &str, contract_id: &str) -> Result<Contract> {
        let body = serde_json::json!({ "contract_id": contract_id });
        self.post("/topapi/contract/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn contract_list(
        &self,
        access_token: &str,
        request: &ContractListRequest,
    ) -> Result<ContractListResponse> {
        self.post("/topapi/contract/list", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContractCreateRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContractCreateResponse {
    pub contract_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Contract {
    pub contract_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContractListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ContractListResponse {
    pub contracts: Vec<Contract>,
    pub has_more: bool,
}
