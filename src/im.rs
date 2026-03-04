//! DingTalk API bindings for the im module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::models::*;

impl DingTalkClient {
    pub async fn send_message(
        &self,
        access_token: &str,
        agent_id: i64,
        user_ids: &[&str],
        msg: &Message,
    ) -> Result<SendMessageResponse> {
        let mut body = serde_json::Map::new();
        body.insert("agent_id".to_string(), agent_id.into());
        body.insert(
            "userid_list".to_string(),
            user_ids
                .iter()
                .map(|s| (*s).to_string())
                .collect::<Vec<_>>()
                .into(),
        );
        body.insert("msg".to_string(), serde_json::to_value(msg)?);

        self.post(
            "/topapi/message/corpconversation/asyncsend_v2",
            access_token,
            &body,
        )
        .await
    }

    pub async fn send_single_message(
        &self,
        access_token: &str,
        agent_id: i64,
        user_id: &str,
        msg: &Message,
    ) -> Result<SendMessageResponse> {
        let mut body = serde_json::Map::new();
        body.insert("agent_id".to_string(), agent_id.into());
        body.insert("userid".to_string(), user_id.into());
        body.insert("msg".to_string(), serde_json::to_value(msg)?);

        self.post(
            "/topapi/message/corpconversation/asyncsend_v2",
            access_token,
            &body,
        )
        .await
    }

    pub async fn recall_message(
        &self,
        access_token: &str,
        agent_id: i64,
        task_id: i64,
    ) -> Result<()> {
        let body = serde_json::json!({
            "agent_id": agent_id,
            "task_id": task_id
        });
        self.post(
            "/topapi/message/corpconversation/recall",
            access_token,
            &body,
        )
        .await
    }

    pub async fn get_message_status(
        &self,
        access_token: &str,
        agent_id: i64,
        task_id: i64,
    ) -> Result<MessageStatusResponse> {
        let body = serde_json::json!({
            "agent_id": agent_id,
            "task_id": task_id
        });
        self.post(
            "/topapi/message/corpconversation/getprogress",
            access_token,
            &body,
        )
        .await
    }

    pub async fn create_conversation(
        &self,
        access_token: &str,
        request: &CreateConversationRequest,
    ) -> Result<CreateConversationResponse> {
        self.post_openapi(
            "/v1.0/im/conversations",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn get_conversation(
        &self,
        access_token: &str,
        open_conversation_id: &str,
    ) -> Result<Conversation> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        self.get_openapi("/v1.0/im/conversations", access_token, Some(&query))
            .await
    }

    pub async fn update_conversation(
        &self,
        access_token: &str,
        request: &UpdateConversationRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/im/conversations",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn send_group_message(
        &self,
        access_token: &str,
        request: &SendGroupMessageRequest,
    ) -> Result<SendGroupMessageResponse> {
        self.post_openapi(
            "/v1.0/im/group/messages",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn recall_group_message(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        message_task_id: &str,
    ) -> Result<()> {
        let body = serde_json::json!({
            "openConversationId": open_conversation_id,
            "messageTaskId": message_task_id
        });
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/im/group/messages/recall",
            access_token,
            None,
            Some(&body),
        )
        .await
    }

    pub async fn get_group_message_read_status(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        message_task_id: &str,
    ) -> Result<GroupMessageReadStatusResponse> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        query.insert("messageTaskId".to_string(), message_task_id.to_string());
        self.get_openapi(
            "/v1.0/im/group/messages/readStatus",
            access_token,
            Some(&query),
        )
        .await
    }

    pub async fn add_robot_to_conversation(
        &self,
        access_token: &str,
        request: &AddRobotToConversationRequest,
    ) -> Result<AddRobotToConversationResponse> {
        self.post_openapi(
            "/v1.0/im/robots/addToConversation",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn remove_robot_from_conversation(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        robot_code: &str,
    ) -> Result<()> {
        let body = serde_json::json!({
            "openConversationId": open_conversation_id,
            "robotCode": robot_code
        });
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/im/robots/removeFromConversation",
            access_token,
            None,
            Some(&body),
        )
        .await
    }

    pub async fn add_conv_nav_tab(
        &self,
        access_token: &str,
        request: &AddConvNavTabRequest,
    ) -> Result<AddConvNavTabResponse> {
        self.post_openapi(
            "/v1.0/im/conversations/navTabs",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn update_conv_nav_tab(
        &self,
        access_token: &str,
        request: &UpdateConvNavTabRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/im/conversations/navTabs",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn delete_conv_nav_tab(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        tab_id: &str,
    ) -> Result<()> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        query.insert("tabId".to_string(), tab_id.to_string());
        self.delete_openapi_no_content("/v1.0/im/conversations/navTabs", access_token, Some(&query))
            .await
    }

    pub async fn update_member_group_nick(
        &self,
        access_token: &str,
        request: &UpdateMemberGroupNickRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/im/sceneGroups/members/groupNicks",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn update_member_ban_words(
        &self,
        access_token: &str,
        request: &UpdateMemberBanWordsRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/im/sceneGroups/muteMembers/set",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn update_group_sub_admin(
        &self,
        access_token: &str,
        request: &UpdateGroupSubAdminRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/im/sceneGroups/subAdmins",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn send_interactive_card(
        &self,
        access_token: &str,
        request: &SendInteractiveCardRequest,
    ) -> Result<SendInteractiveCardResponse> {
        self.post_openapi(
            "/v1.0/im/interactiveCards/send",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn update_interactive_card(
        &self,
        access_token: &str,
        request: &UpdateInteractiveCardRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/im/interactiveCards",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    #[serde(rename = "task_id")]
    pub task_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_percent: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_media_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_all_authority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_history_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_media_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_union_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConversationRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_media_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_all_authority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_history_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGroupMessageRequest {
    pub msg_key: String,
    pub msg_param: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cool_robot_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendGroupMessageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_query_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMessageReadStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRobotToConversationRequest {
    pub open_conversation_id: String,
    pub robot_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRobotToConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_bot_user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddConvNavTabRequest {
    pub open_conversation_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddConvNavTabResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConvNavTabRequest {
    pub open_conversation_id: String,
    pub tab_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_editable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberGroupNickRequest {
    pub open_conversation_id: String,
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_nick: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberBanWordsRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupSubAdminRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInteractiveCardRequest {
    pub card_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_data: Option<CardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_options: Option<CardOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_track_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_data: Option<PrivateData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_param_map: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_media_id_param_map: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_param_map: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_media_id_param_map: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendInteractiveCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_query_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInteractiveCardRequest {
    pub out_track_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_data: Option<CardData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_options: Option<CardOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_data: Option<PrivateData>,
}
