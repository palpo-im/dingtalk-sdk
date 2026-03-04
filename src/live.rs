//! DingTalk API bindings for the live module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn live_create_room(
        &self,
        access_token: &str,
        request: &LiveCreateRoomRequest,
    ) -> Result<LiveCreateRoomResponse> {
        self.post("/topapi/live/room/create", access_token, request)
            .await
    }

    /// Executes this API call.
    pub async fn live_get_room(&self, access_token: &str, room_id: &str) -> Result<LiveRoom> {
        let body = serde_json::json!({ "room_id": room_id});
        self.post("/topapi/live/room/get", access_token, &body)
            .await
    }

    /// Executes this API call.
    pub async fn live_list_rooms(
        &self,
        access_token: &str,
        request: &LiveListRoomsRequest,
    ) -> Result<LiveListRoomsResponse> {
        self.post("/topapi/live/room/list", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LiveCreateRoomRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LiveCreateRoomResponse {
    pub room_id: String,
    pub stream_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LiveRoom {
    pub room_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LiveListRoomsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LiveListRoomsResponse {
    pub rooms: Vec<LiveRoom>,
    pub has_more: bool,
    pub next_cursor: i64,
}
