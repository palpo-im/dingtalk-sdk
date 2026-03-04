//! DingTalk CRM OpenAPI (v1.0) endpoints.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCrmPersonalCustomersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_operator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
}

impl ListCrmPersonalCustomersQuery {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = &self.current_operator_user_id {
            params.insert("currentOperatorUserId".to_string(), value.clone());
        }
        if let Some(value) = &self.relation_type {
            params.insert("relationType".to_string(), value.clone());
        }
        params
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteCrmPersonalCustomerQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_operator_user_id: Option<String>,
}

impl DeleteCrmPersonalCustomerQuery {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = &self.current_operator_user_id {
            params.insert("currentOperatorUserId".to_string(), value.clone());
        }
        params
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCrmPersonalCustomerResponse {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
}

impl DingTalkClient {
    pub async fn crm_add_personal_customer(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<AddCrmPersonalCustomerResponse> {
        self.post_openapi("/v1.0/crm/personalCustomers", access_token, Some(request))
            .await
    }

    pub async fn crm_delete_personal_customer(
        &self,
        access_token: &str,
        data_id: &str,
        query: Option<&DeleteCrmPersonalCustomerQuery>,
    ) -> Result<()> {
        let path = format!("/v1.0/crm/personalCustomers/{data_id}");
        let query_map = query
            .map(DeleteCrmPersonalCustomerQuery::to_query_params)
            .unwrap_or_default();
        let query_ref = if query_map.is_empty() {
            None
        } else {
            Some(&query_map)
        };
        self.delete_openapi_no_content(&path, access_token, query_ref)
            .await
    }

    pub async fn crm_list_personal_customers(
        &self,
        access_token: &str,
        query: Option<&ListCrmPersonalCustomersQuery>,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let path = "/v1.0/crm/personalCustomers/batchQuery";
        let query_map = query
            .map(ListCrmPersonalCustomersQuery::to_query_params)
            .unwrap_or_default();
        let query_ref = if query_map.is_empty() {
            None
        } else {
            Some(&query_map)
        };
        self.request_openapi(Method::POST, path, access_token, query_ref, body)
            .await
    }

    pub async fn crm_add_leads(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.post_openapi("/v1.0/crm/leads", access_token, Some(request))
            .await
    }

    pub async fn crm_delete_leads(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.post_openapi("/v1.0/crm/leads/remove", access_token, Some(request))
            .await
    }

    pub async fn crm_query_all_customers(
        &self,
        access_token: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.post_openapi("/v1.0/crm/customerInstances", access_token, Some(request))
            .await
    }

    pub async fn crm_create_custom_data_field(
        &self,
        access_token: &str,
        request: &CreateCustomDataFieldRequest,
    ) -> Result<CreateCustomDataFieldResponse> {
        self.post_openapi(
            "/v1.0/crm/customData/fields",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_update_custom_data_field(
        &self,
        access_token: &str,
        request: &UpdateCustomDataFieldRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::PUT,
            "/v1.0/crm/customData/fields",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_delete_custom_data_field(
        &self,
        access_token: &str,
        field_id: &str,
    ) -> Result<()> {
        let path = format!("/v1.0/crm/customData/fields/{field_id}");
        self.delete_openapi_no_content(&path, access_token, None)
            .await
    }

    pub async fn crm_list_custom_data_fields(
        &self,
        access_token: &str,
        request: &ListCustomDataFieldsRequest,
    ) -> Result<ListCustomDataFieldsResponse> {
        self.post_openapi(
            "/v1.0/crm/customData/fields/query",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_create_customer_tag(
        &self,
        access_token: &str,
        request: &CreateCustomerTagRequest,
    ) -> Result<CreateCustomerTagResponse> {
        self.post_openapi(
            "/v1.0/crm/customerTags",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_update_customer_tag(
        &self,
        access_token: &str,
        request: &UpdateCustomerTagRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::PUT,
            "/v1.0/crm/customerTags",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_delete_customer_tag(&self, access_token: &str, tag_id: &str) -> Result<()> {
        let path = format!("/v1.0/crm/customerTags/{tag_id}");
        self.delete_openapi_no_content(&path, access_token, None)
            .await
    }

    pub async fn crm_list_customer_tags(
        &self,
        access_token: &str,
        request: &ListCustomerTagsRequest,
    ) -> Result<ListCustomerTagsResponse> {
        self.post_openapi(
            "/v1.0/crm/customerTags/query",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_add_tags_to_customer(
        &self,
        access_token: &str,
        request: &AddTagsToCustomerRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/crm/customerTags/bind",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_remove_tags_from_customer(
        &self,
        access_token: &str,
        request: &RemoveTagsFromCustomerRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/crm/customerTags/unbind",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_create_relation_type(
        &self,
        access_token: &str,
        request: &CreateRelationTypeRequest,
    ) -> Result<CreateRelationTypeResponse> {
        self.post_openapi(
            "/v1.0/crm/relationTypes",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_update_relation_type(
        &self,
        access_token: &str,
        request: &UpdateRelationTypeRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::PUT,
            "/v1.0/crm/relationTypes",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_delete_relation_type(&self, access_token: &str, type_id: &str) -> Result<()> {
        let path = format!("/v1.0/crm/relationTypes/{type_id}");
        self.delete_openapi_no_content(&path, access_token, None)
            .await
    }

    pub async fn crm_list_relation_types(
        &self,
        access_token: &str,
        request: &ListRelationTypesRequest,
    ) -> Result<ListRelationTypesResponse> {
        self.post_openapi(
            "/v1.0/crm/relationTypes/query",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_create_stage(
        &self,
        access_token: &str,
        request: &CreateStageRequest,
    ) -> Result<CreateStageResponse> {
        self.post_openapi(
            "/v1.0/crm/stages",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_update_stage(
        &self,
        access_token: &str,
        request: &UpdateStageRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::PUT,
            "/v1.0/crm/stages",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_delete_stage(&self, access_token: &str, stage_id: &str) -> Result<()> {
        let path = format!("/v1.0/crm/stages/{stage_id}");
        self.delete_openapi_no_content(&path, access_token, None)
            .await
    }

    pub async fn crm_list_stages(
        &self,
        access_token: &str,
        request: &ListStagesRequest,
    ) -> Result<ListStagesResponse> {
        self.post_openapi(
            "/v1.0/crm/stages/query",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_update_customer_stage(
        &self,
        access_token: &str,
        request: &UpdateCustomerStageRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/crm/customerStages",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_get_customer(
        &self,
        access_token: &str,
        instance_id: &str,
    ) -> Result<CustomerInstance> {
        let path = format!("/v1.0/crm/customerInstances/{instance_id}");
        self.get_openapi(&path, access_token, None).await
    }

    pub async fn crm_update_customer(
        &self,
        access_token: &str,
        request: &UpdateCustomerRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::PUT,
            "/v1.0/crm/customerInstances",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_transfer_customer(
        &self,
        access_token: &str,
        request: &TransferCustomerRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            Method::POST,
            "/v1.0/crm/customers/transfer",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn crm_get_customer_by_external_id(
        &self,
        access_token: &str,
        external_id: &str,
    ) -> Result<CustomerInstance> {
        let mut query = HashMap::new();
        query.insert("externalId".to_string(), external_id.to_string());
        self.get_openapi(
            "/v1.0/crm/customerInstances/byExternalId",
            access_token,
            Some(&query),
        )
        .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomDataFieldRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomDataFieldResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomDataFieldRequest {
    pub field_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomDataFieldsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomDataFieldsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<CustomDataField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDataField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerTagRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerTagResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerTagRequest {
    pub tag_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomerTagsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCustomerTagsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CustomerTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTagsToCustomerRequest {
    pub instance_id: String,
    pub tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTagsFromCustomerRequest {
    pub instance_id: String,
    pub tag_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRelationTypeRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRelationTypeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRelationTypeRequest {
    pub type_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRelationTypesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRelationTypesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<RelationType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStageRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStageResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStageRequest {
    pub stage_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStagesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<Stage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probability: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerStageRequest {
    pub instance_id: String,
    pub stage_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInstance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CustomerTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub instance_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCustomerRequest {
    pub instance_ids: Vec<String>,
    pub new_owner_user_id: String,
}
