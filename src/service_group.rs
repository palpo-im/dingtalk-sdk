//! DingTalk API bindings for the service_group module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::models::*;

impl DingTalkClient {
    pub async fn service_group_list(&self, access_token: &str) -> Result<ServiceGroupListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/service_group/list", access_token, &body)
            .await
    }

    pub async fn service_group_create(
        &self,
        access_token: &str,
        request: &CreateServiceGroupRequest,
    ) -> Result<CreateServiceGroupResponse> {
        self.post("/v1.0/serviceGroups", access_token, request)
            .await
    }

    pub async fn service_group_get(
        &self,
        access_token: &str,
        open_conversation_id: &str,
    ) -> Result<ServiceGroup> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        self.get_openapi("/v1.0/serviceGroups", access_token, Some(&query))
            .await
    }

    pub async fn service_group_update(
        &self,
        access_token: &str,
        request: &UpdateServiceGroupRequest,
    ) -> Result<()> {
        self.put_openapi::<serde_json::Value>(
            "/v1.0/serviceGroups",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await?;
        Ok(())
    }

    pub async fn service_group_delete(
        &self,
        access_token: &str,
        open_conversation_id: &str,
    ) -> Result<()> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        self.delete_openapi_no_content("/v1.0/serviceGroups", access_token, Some(&query))
            .await
    }

    pub async fn service_group_add_members(
        &self,
        access_token: &str,
        request: &AddServiceGroupMembersRequest,
    ) -> Result<AddServiceGroupMembersResponse> {
        self.post_openapi(
            "/v1.0/serviceGroups/members/add",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_remove_members(
        &self,
        access_token: &str,
        request: &RemoveServiceGroupMembersRequest,
    ) -> Result<RemoveServiceGroupMembersResponse> {
        self.post_openapi(
            "/v1.0/serviceGroups/members/remove",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_list_members(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<ListServiceGroupMembersResponse> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        if let Some(c) = cursor {
            query.insert("cursor".to_string(), c.to_string());
        }
        if let Some(s) = size {
            query.insert("size".to_string(), s.to_string());
        }
        self.get_openapi("/v1.0/serviceGroups/members", access_token, Some(&query))
            .await
    }

    pub async fn service_group_add_admins(
        &self,
        access_token: &str,
        request: &AddServiceGroupAdminsRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/serviceGroups/admins/add",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_remove_admins(
        &self,
        access_token: &str,
        request: &RemoveServiceGroupAdminsRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/serviceGroups/admins/remove",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_list_admins(
        &self,
        access_token: &str,
        open_conversation_id: &str,
    ) -> Result<ListServiceGroupAdminsResponse> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        self.get_openapi("/v1.0/serviceGroups/admins", access_token, Some(&query))
            .await
    }

    pub async fn service_group_send_message(
        &self,
        access_token: &str,
        request: &SendServiceGroupMessageRequest,
    ) -> Result<SendServiceGroupMessageResponse> {
        self.post_openapi(
            "/v1.0/serviceGroups/messages/send",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_recall_message(
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
            "/v1.0/serviceGroups/messages/recall",
            access_token,
            None,
            Some(&body),
        )
        .await
    }

    pub async fn service_group_get_message_read_status(
        &self,
        access_token: &str,
        open_conversation_id: &str,
        message_task_id: &str,
    ) -> Result<ServiceGroupMessageReadStatusResponse> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        query.insert("messageTaskId".to_string(), message_task_id.to_string());
        self.get_openapi(
            "/v1.0/serviceGroups/messages/readStatus",
            access_token,
            Some(&query),
        )
        .await
    }

    pub async fn service_group_list_user_groups(
        &self,
        access_token: &str,
        user_id: &str,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<ListUserGroupsResponse> {
        let mut query = HashMap::new();
        query.insert("userId".to_string(), user_id.to_string());
        if let Some(c) = cursor {
            query.insert("cursor".to_string(), c.to_string());
        }
        if let Some(s) = size {
            query.insert("size".to_string(), s.to_string());
        }
        self.get_openapi("/v1.0/serviceGroups/userGroups", access_token, Some(&query))
            .await
    }

    pub async fn service_group_add_managers(
        &self,
        access_token: &str,
        request: &AddServiceGroupManagersRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/serviceGroups/managers/add",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_remove_managers(
        &self,
        access_token: &str,
        request: &RemoveServiceGroupManagersRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::POST,
            "/v1.0/serviceGroups/managers/remove",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn service_group_list_managers(
        &self,
        access_token: &str,
        open_conversation_id: &str,
    ) -> Result<ListServiceGroupManagersResponse> {
        let mut query = HashMap::new();
        query.insert(
            "openConversationId".to_string(),
            open_conversation_id.to_string(),
        );
        self.get_openapi("/v1.0/serviceGroups/managers", access_token, Some(&query))
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ServiceGroupInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateServiceGroupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateServiceGroupRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddServiceGroupMembersRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddServiceGroupMembersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveServiceGroupMembersRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveServiceGroupMembersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListServiceGroupMembersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_list: Option<Vec<ServiceGroupMember>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddServiceGroupAdminsRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveServiceGroupAdminsRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListServiceGroupAdminsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_list: Option<Vec<ServiceGroupAdmin>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupAdmin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendServiceGroupMessageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_conversation_id: Option<String>,
    pub msg_key: String,
    pub msg_param: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robot_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendServiceGroupMessageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_query_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupMessageReadStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUserGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<ServiceGroupInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddServiceGroupManagersRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveServiceGroupManagersRequest {
    pub open_conversation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListServiceGroupManagersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_list: Option<Vec<ServiceGroupManager>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceGroupManager {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}
