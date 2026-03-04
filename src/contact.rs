//! DingTalk API bindings for the contact module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::models::*;

impl DingTalkClient {
    pub async fn get_user(&self, access_token: &str, user_id: &str) -> Result<crate::models::User> {
        let body = serde_json::json!({ "userid": user_id });
        self.post("/topapi/v2/user/get", access_token, &body).await
    }

    pub async fn get_user_by_mobile(
        &self,
        access_token: &str,
        mobile: &str,
    ) -> Result<GetUserByMobileResponse> {
        let body = serde_json::json!({ "mobile": mobile });
        self.post("/topapi/v2/user/getbymobile", access_token, &body)
            .await
    }

    pub async fn create_user(
        &self,
        access_token: &str,
        user: &CreateUserRequest,
    ) -> Result<CreateUserResponse> {
        self.post("/topapi/v2/user/create", access_token, user)
            .await
    }

    pub async fn update_user(&self, access_token: &str, user: &UpdateUserRequest) -> Result<()> {
        self.post("/topapi/v2/user/update", access_token, user)
            .await
    }

    pub async fn delete_user(&self, access_token: &str, user_id: &str) -> Result<()> {
        let body = serde_json::json!({ "userid": user_id });
        self.post("/topapi/v2/user/delete", access_token, &body)
            .await
    }

    pub async fn get_department_users(
        &self,
        access_token: &str,
        dept_id: i64,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<GetDepartmentUsersResponse> {
        let mut body = serde_json::json!({ "dept_id": dept_id });
        if let Some(c) = cursor {
            body["cursor"] = c.into();
        }
        if let Some(s) = size {
            body["size"] = s.into();
        }
        self.post("/topapi/v2/user/list", access_token, &body).await
    }

    pub async fn get_department(&self, access_token: &str, dept_id: i64) -> Result<Department> {
        let body = serde_json::json!({ "dept_id": dept_id });
        self.post("/topapi/v2/department/get", access_token, &body)
            .await
    }

    pub async fn list_departments(&self, access_token: &str) -> Result<ListDepartmentsResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/v2/department/listsub", access_token, &body)
            .await
    }

    pub async fn create_department(
        &self,
        access_token: &str,
        dept: &CreateDepartmentRequest,
    ) -> Result<CreateDepartmentResponse> {
        self.post("/topapi/v2/department/create", access_token, dept)
            .await
    }

    pub async fn update_department(
        &self,
        access_token: &str,
        dept: &UpdateDepartmentRequest,
    ) -> Result<()> {
        self.post("/topapi/v2/department/update", access_token, dept)
            .await
    }

    pub async fn delete_department(&self, access_token: &str, dept_id: i64) -> Result<()> {
        let body = serde_json::json!({ "dept_id": dept_id });
        self.post("/topapi/v2/department/delete", access_token, &body)
            .await
    }

    pub async fn batch_create_users(
        &self,
        access_token: &str,
        request: &BatchCreateUsersRequest,
    ) -> Result<BatchCreateUsersResponse> {
        self.post("/topapi/v2/user/batch_create", access_token, request)
            .await
    }

    pub async fn batch_update_users(
        &self,
        access_token: &str,
        request: &BatchUpdateUsersRequest,
    ) -> Result<BatchUpdateUsersResponse> {
        self.post("/topapi/v2/user/batch_update", access_token, request)
            .await
    }

    pub async fn batch_delete_users(
        &self,
        access_token: &str,
        request: &BatchDeleteUsersRequest,
    ) -> Result<()> {
        self.post("/topapi/v2/user/batch_delete", access_token, request)
            .await
    }

    pub async fn batch_get_users(
        &self,
        access_token: &str,
        request: &BatchGetUsersRequest,
    ) -> Result<BatchGetUsersResponse> {
        self.post("/topapi/v2/user/batch_get", access_token, request)
            .await
    }

    pub async fn create_role(
        &self,
        access_token: &str,
        request: &CreateRoleRequest,
    ) -> Result<CreateRoleResponse> {
        self.post("/topapi/role/create", access_token, request)
            .await
    }

    pub async fn update_role(&self, access_token: &str, request: &UpdateRoleRequest) -> Result<()> {
        self.post("/topapi/role/update", access_token, request)
            .await
    }

    pub async fn delete_role(&self, access_token: &str, role_id: i64) -> Result<()> {
        let body = serde_json::json!({ "role_id": role_id });
        self.post("/topapi/role/delete", access_token, &body).await
    }

    pub async fn get_role(&self, access_token: &str, role_id: i64) -> Result<Role> {
        let body = serde_json::json!({ "role_id": role_id });
        self.post("/topapi/role/get", access_token, &body).await
    }

    pub async fn list_roles(&self, access_token: &str) -> Result<ListRolesResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/role/list", access_token, &body).await
    }

    pub async fn add_users_to_role(
        &self,
        access_token: &str,
        request: &AddUsersToRoleRequest,
    ) -> Result<()> {
        self.post("/topapi/role/addrolesforemps", access_token, request)
            .await
    }

    pub async fn remove_users_from_role(
        &self,
        access_token: &str,
        request: &RemoveUsersFromRoleRequest,
    ) -> Result<()> {
        self.post("/topapi/role/removerolesforemps", access_token, request)
            .await
    }

    pub async fn get_role_users(
        &self,
        access_token: &str,
        role_id: i64,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<GetRoleUsersResponse> {
        let mut body = serde_json::json!({ "role_id": role_id });
        if let Some(c) = cursor {
            body["cursor"] = c.into();
        }
        if let Some(s) = size {
            body["size"] = s.into();
        }
        self.post("/topapi/role/simplelist", access_token, &body)
            .await
    }

    pub async fn create_role_group(
        &self,
        access_token: &str,
        request: &CreateRoleGroupRequest,
    ) -> Result<CreateRoleGroupResponse> {
        self.post("/topapi/role/add_role_group", access_token, request)
            .await
    }

    pub async fn create_label(
        &self,
        access_token: &str,
        request: &CreateLabelRequest,
    ) -> Result<CreateLabelResponse> {
        self.post("/topapi/label/create", access_token, request)
            .await
    }

    pub async fn update_label(
        &self,
        access_token: &str,
        request: &UpdateLabelRequest,
    ) -> Result<()> {
        self.post("/topapi/label/update", access_token, request)
            .await
    }

    pub async fn delete_label(&self, access_token: &str, label_id: i64) -> Result<()> {
        let body = serde_json::json!({ "label_id": label_id });
        self.post("/topapi/label/delete", access_token, &body).await
    }

    pub async fn list_labels(&self, access_token: &str) -> Result<ListLabelsResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/label/list", access_token, &body).await
    }

    pub async fn add_label_to_user(
        &self,
        access_token: &str,
        request: &AddLabelToUserRequest,
    ) -> Result<()> {
        self.post("/topapi/label/adduserlabel", access_token, request)
            .await
    }

    pub async fn remove_label_from_user(
        &self,
        access_token: &str,
        request: &RemoveLabelFromUserRequest,
    ) -> Result<()> {
        self.post("/topapi/label/deluserlabel", access_token, request)
            .await
    }

    pub async fn get_label_users(
        &self,
        access_token: &str,
        label_id: i64,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<GetLabelUsersResponse> {
        let mut body = serde_json::json!({ "label_id": label_id });
        if let Some(c) = cursor {
            body["cursor"] = c.into();
        }
        if let Some(s) = size {
            body["size"] = s.into();
        }
        self.post("/topapi/label/getuserlistbylabel", access_token, &body)
            .await
    }

    pub async fn create_tag(
        &self,
        access_token: &str,
        request: &CreateTagRequest,
    ) -> Result<CreateTagResponse> {
        self.post("/topapi/v2/tag/create", access_token, request)
            .await
    }

    pub async fn update_tag(&self, access_token: &str, request: &UpdateTagRequest) -> Result<()> {
        self.post("/topapi/v2/tag/update", access_token, request)
            .await
    }

    pub async fn delete_tag(&self, access_token: &str, tag_id: i64) -> Result<()> {
        let body = serde_json::json!({ "tag_id": tag_id });
        self.post("/topapi/v2/tag/delete", access_token, &body)
            .await
    }

    pub async fn get_tag(&self, access_token: &str, tag_id: i64) -> Result<Tag> {
        let body = serde_json::json!({ "tag_id": tag_id });
        self.post("/topapi/v2/tag/get", access_token, &body).await
    }

    pub async fn list_tags(&self, access_token: &str) -> Result<ListTagsResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/v2/tag/list", access_token, &body).await
    }

    pub async fn add_users_to_tag(
        &self,
        access_token: &str,
        request: &AddUsersToTagRequest,
    ) -> Result<AddUsersToTagResponse> {
        self.post("/topapi/v2/tag/adduser", access_token, request)
            .await
    }

    pub async fn remove_users_from_tag(
        &self,
        access_token: &str,
        request: &RemoveUsersFromTagRequest,
    ) -> Result<RemoveUsersFromTagResponse> {
        self.post("/topapi/v2/tag/removeuser", access_token, request)
            .await
    }

    pub async fn add_departments_to_tag(
        &self,
        access_token: &str,
        request: &AddDepartmentsToTagRequest,
    ) -> Result<()> {
        self.post("/topapi/v2/tag/adddept", access_token, request)
            .await
    }

    pub async fn remove_departments_from_tag(
        &self,
        access_token: &str,
        request: &RemoveDepartmentsFromTagRequest,
    ) -> Result<()> {
        self.post("/topapi/v2/tag/removedept", access_token, request)
            .await
    }

    pub async fn get_tag_users(
        &self,
        access_token: &str,
        tag_id: i64,
        cursor: Option<i64>,
        size: Option<i64>,
    ) -> Result<GetTagUsersResponse> {
        let mut body = serde_json::json!({ "tag_id": tag_id });
        if let Some(c) = cursor {
            body["cursor"] = c.into();
        }
        if let Some(s) = size {
            body["size"] = s.into();
        }
        self.post("/topapi/v2/tag/getuserlist", access_token, &body)
            .await
    }

    pub async fn get_tag_departments(
        &self,
        access_token: &str,
        tag_id: i64,
    ) -> Result<GetTagDepartmentsResponse> {
        let body = serde_json::json!({ "tag_id": tag_id });
        self.post("/topapi/v2/tag/getdeptlist", access_token, &body)
            .await
    }

    pub async fn add_account_mapping(
        &self,
        access_token: &str,
        request: &AddAccountMappingRequest,
    ) -> Result<bool> {
        self.post_openapi(
            "/v1.0/contact/accountMapping",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn get_account_mapping(
        &self,
        access_token: &str,
        domain: &str,
        out_id: &str,
        out_tenant_id: &str,
    ) -> Result<GetAccountMappingResponse> {
        let mut query = HashMap::new();
        query.insert("domain".to_string(), domain.to_string());
        query.insert("outId".to_string(), out_id.to_string());
        query.insert("outTenantId".to_string(), out_tenant_id.to_string());
        self.get_openapi("/v1.0/contact/accountMapping", access_token, Some(&query))
            .await
    }

    pub async fn delete_account_mapping(
        &self,
        access_token: &str,
        domain: &str,
        out_id: &str,
        out_tenant_id: &str,
    ) -> Result<bool> {
        let mut query = HashMap::new();
        query.insert("domain".to_string(), domain.to_string());
        query.insert("outId".to_string(), out_id.to_string());
        query.insert("outTenantId".to_string(), out_tenant_id.to_string());
        self.delete_openapi("/v1.0/contact/accountMapping", access_token, Some(&query))
            .await
    }

    pub async fn get_org_user_count(&self, access_token: &str, only_active: bool) -> Result<i64> {
        let body = serde_json::json!({ "only_active": only_active });
        let resp: OrgUserCountResponse =
            self.post("/topapi/user/count", access_token, &body).await?;
        Ok(resp.count)
    }

    pub async fn get_admin_list(&self, access_token: &str) -> Result<GetAdminListResponse> {
        let body = serde_json::json!({});
        self.post("/topapi/user/get_admin", access_token, &body)
            .await
    }

    pub async fn get_org_info(&self, access_token: &str) -> Result<OrgInfo> {
        let body = serde_json::json!({});
        self.post("/topapi/v2/org/get", access_token, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByMobileResponse {
    #[serde(rename = "userid")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    #[serde(rename = "userid")]
    pub user_id: String,
    pub name: String,
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
    pub dept_id_list: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_dept_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    #[serde(rename = "userid")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "userid")]
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    pub dept_id_list: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepartmentUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<crate::models::User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    #[serde(rename = "dept_id")]
    pub dept_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dept_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDepartmentsResponse {
    pub result: Vec<Department>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dept_group: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentResponse {
    #[serde(rename = "dept_id")]
    pub dept_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDepartmentRequest {
    #[serde(rename = "dept_id")]
    pub dept_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_add_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateUsersRequest {
    pub user_list: Vec<CreateUserRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_userid_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_userid_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateUsersRequest {
    pub user_list: Vec<UpdateUserRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_userid_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_userid_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteUsersRequest {
    #[serde(rename = "userid_list")]
    pub user_id_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersRequest {
    #[serde(rename = "userid_list")]
    pub user_id_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<crate::models::User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub role_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    #[serde(rename = "roleId")]
    pub role_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    #[serde(rename = "roleId")]
    pub role_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<RoleGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUsersToRoleRequest {
    #[serde(rename = "roleId")]
    pub role_id: i64,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUsersFromRoleRequest {
    #[serde(rename = "roleId")]
    pub role_id: i64,
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<RoleUserInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleGroupRequest {
    pub group_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleGroupResponse {
    #[serde(rename = "groupId")]
    pub group_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLabelRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLabelResponse {
    #[serde(rename = "id")]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLabelRequest {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLabelsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<Label>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddLabelToUserRequest {
    #[serde(rename = "userid")]
    pub user_id: String,
    pub labels: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveLabelFromUserRequest {
    #[serde(rename = "userid")]
    pub user_id: String,
    pub labels: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLabelUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<LabelUserInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelUserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagResponse {
    #[serde(rename = "tag_id")]
    pub tag_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "tag_id")]
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTagsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUsersToTagRequest {
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUsersToTagResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub over_user_limit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUsersFromTagRequest {
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUsersFromTagResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddDepartmentsToTagRequest {
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveDepartmentsFromTagRequest {
    pub tag_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<TagUserInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagUserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTagDepartmentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<TagDepartmentInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDepartmentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAccountMappingRequest {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_id: Option<String>,
    pub out_tenant_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountMappingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUserCountResponse {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAdminListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_list: Option<Vec<AdminInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
}
