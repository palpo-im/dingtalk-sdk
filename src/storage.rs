//! DingTalk API bindings for the storage module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn upload_file(
        &self,
        access_token: &str,
        request: &StorageUploadRequest,
    ) -> Result<StorageUploadResponse> {
        self.post("/storage/file/upload", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn download_file(
        &self,
        access_token: &str,
        file_id: &str,
    ) -> Result<StorageDownloadResponse> {
        let body = serde_json::json!({ "file_id": file_id });
        self.post("/storage/file/download", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn get_file_info(
        &self,
        access_token: &str,
        file_id: &str,
    ) -> Result<StorageFileInfo> {
        let body = serde_json::json!({ "file_id": file_id });
        self.post("/storage/file/get", access_token, &body).await
    }

    /// Executes this API call.
    pub async fn delete_file(&self, access_token: &str, file_id: &str) -> Result<()> {
        let body = serde_json::json!({ "file_id": file_id });
        self.post("/storage/file/delete", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct StorageUploadRequest {
    pub file_name: String,
    pub file_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct StorageUploadResponse {
    pub file_id: String,
    pub upload_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct StorageDownloadResponse {
    pub download_url: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct StorageFileInfo {
    pub file_id: String,
    pub file_name: String,
    pub file_size: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}
