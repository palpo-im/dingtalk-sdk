//! DingTalk API bindings for the drive module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn drive_get_space_info(
        &self,
        access_token: &str,
        user_id: &str,
    ) -> Result<DriveSpaceInfo> {
        let body = serde_json::json!({ "user_id": user_id });
        self.post("/topapi/drive/space/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn drive_list_files(
        &self,
        access_token: &str,
        request: &DriveListFilesRequest,
    ) -> Result<DriveListFilesResponse> {
        self.post("/topapi/drive/file/list", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn drive_create_folder(
        &self,
        access_token: &str,
        request: &DriveCreateFolderRequest,
    ) -> Result<DriveCreateFolderResponse> {
        self.post("/topapi/drive/folder/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn drive_delete_file(
        &self,
        access_token: &str,
        file_id: &str,
        user_id: &str,
    ) -> Result<()> {
        let body = serde_json::json!({ "file_id": file_id, "user_id": user_id });
        self.post("/topapi/drive/file/delete", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn drive_move_file(
        &self,
        access_token: &str,
        request: &DriveMoveFileRequest,
    ) -> Result<()> {
        self.post("/topapi/drive/file/move", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn drive_copy_file(
        &self,
        access_token: &str,
        request: &DriveCopyFileRequest,
    ) -> Result<DriveCopyFileResponse> {
        self.post("/topapi/drive/file/copy", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveSpaceInfo {
    pub used_size: i64,
    pub total_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveListFilesRequest {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveListFilesResponse {
    pub files: Vec<DriveFile>,
    pub has_more: bool,
    pub next_cursor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveFile {
    pub file_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveCreateFolderRequest {
    pub user_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveCreateFolderResponse {
    pub file_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveMoveFileRequest {
    pub user_id: String,
    pub file_id: String,
    pub target_parent_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveCopyFileRequest {
    pub user_id: String,
    pub file_id: String,
    pub target_parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct DriveCopyFileResponse {
    pub file_id: String,
}
