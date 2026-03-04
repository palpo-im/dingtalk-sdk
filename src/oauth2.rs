//! DingTalk API bindings for the oauth2 module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn get_access_token_by_code(
        &self,
        access_token: &str,
        code: &str,
    ) -> Result<GetAccessTokenByCodeResponse> {
        let body = serde_json::json!({ "code": code });
        self.post("/sns/gettoken", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn get_user_info(
        &self,
        access_token: &str,
        code: &str,
    ) -> Result<GetUserInfoResponse> {
        let body = serde_json::json!({ "code": code });
        self.post("/sns/getuserinfo", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn get_permanent_code(
        &self,
        access_token: &str,
        tmp_auth_code: &str,
    ) -> Result<GetPermanentCodeResponse> {
        let body = serde_json::json!({ "tmp_auth_code": tmp_auth_code });
        self.post("/service/get_permanent_code", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn get_auth_info(&self, access_token: &str) -> Result<GetAuthInfoResponse> {
        let body = serde_json::json!({});
        self.post("/service/get_auth_info", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct GetAccessTokenByCodeResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct GetUserInfoResponse {
    pub openid: String,
    pub unionid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct GetPermanentCodeResponse {
    pub permanent_code: String,
    pub auth_corp_info: AuthCorpInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AuthCorpInfo {
    pub corpid: String,
    pub corp_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_logo_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct GetAuthInfoResponse {
    pub auth_corp_info: AuthCorpInfo,
    pub auth_info: AuthInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AuthInfo {
    pub agent: Vec<AgentInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct AgentInfo {
    pub agentid: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}
