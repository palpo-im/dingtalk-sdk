//! DingTalk API bindings for the models module.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct User {
    pub userid: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id_list: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_dept_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_boss: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leader_in_depts: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hide: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leader: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hired_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_join_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Department {
    pub dept_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dept_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_hiding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_permitted_userids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_manager_useridlist: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_dept_owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Message {
    #[serde(rename = "msgtype")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<VoiceMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oa: Option<OaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<MarkdownMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_card: Option<ActionCardMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct TextMessage {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ImageMessage {
    pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct VoiceMessage {
    pub media_id: String,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct FileMessage {
    pub media_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct LinkMessage {
    pub message_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OaMessage {
    pub message_url: String,
    pub pc_message_url: String,
    pub head: OaMessageHead,
    pub body: OaMessageBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OaMessageHead {
    pub bgcolor: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OaMessageBody {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<OaForm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich: Option<OaRich>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OaForm {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct OaRich {
    pub num: i64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct MarkdownMessage {
    pub title: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ActionCardMessage {
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_json_list: Option<Vec<ActionCardButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ActionCardButton {
    pub title: String,
    pub action_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct Response<T> {
    pub errcode: i64,
    pub errmsg: String,
    #[serde(flatten)]
    pub data: T,
}

impl<T> Response<T> {
    /// Executes this helper method.
    pub fn is_success(&self) -> bool {
        self.errcode == 0
    }
}
