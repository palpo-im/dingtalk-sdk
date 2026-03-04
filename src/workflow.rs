//! DingTalk API bindings for the workflow module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    pub async fn workflow_create(
        &self,
        access_token: &str,
        request: &WorkflowCreateRequest,
    ) -> Result<WorkflowCreateResponse> {
        self.post("/topapi/process/create", access_token, request)
            .await
    }

    pub async fn workflow_get(
        &self,
        access_token: &str,
        process_instance_id: &str,
    ) -> Result<WorkflowGetResponse> {
        let body = serde_json::json!({ "process_instance_id": process_instance_id });
        self.post("/topapi/process/get", access_token, &body).await
    }

    pub async fn workflow_list_by_user(
        &self,
        access_token: &str,
        user_id: &str,
        start_time: i64,
        end_time: i64,
        size: i64,
        cursor: i64,
    ) -> Result<WorkflowListResponse> {
        let body = serde_json::json!({
            "userid": user_id,
            "start_time": start_time,
            "end_time": end_time,
            "size": size,
            "cursor": cursor
        });
        self.post("/topapi/process/listbyuserid", access_token, &body)
            .await
    }

    pub async fn workflow_copy(
        &self,
        access_token: &str,
        process_instance_id: &str,
    ) -> Result<WorkflowCopyResponse> {
        let body = serde_json::json!({ "process_instance_id": process_instance_id });
        self.post("/topapi/process/copy", access_token, &body).await
    }

    pub async fn workflow_terminate(
        &self,
        access_token: &str,
        request: &WorkflowTerminateRequest,
    ) -> Result<()> {
        self.post("/topapi/process/terminate", access_token, request)
            .await
    }

    pub async fn workflow_list_by_creator(
        &self,
        access_token: &str,
        request: &WorkflowListByCreatorRequest,
    ) -> Result<WorkflowListByCreatorResponse> {
        self.post("/topapi/process/listbycreator", access_token, request)
            .await
    }

    pub async fn workflow_list_by_staff(
        &self,
        access_token: &str,
        request: &WorkflowListByStaffRequest,
    ) -> Result<WorkflowListByStaffResponse> {
        self.post("/topapi/process/listbystaff", access_token, request)
            .await
    }

    pub async fn workflow_update(
        &self,
        access_token: &str,
        request: &WorkflowUpdateRequest,
    ) -> Result<()> {
        self.post("/topapi/process/update", access_token, request)
            .await
    }

    pub async fn workflow_get_task(
        &self,
        access_token: &str,
        task_id: &str,
    ) -> Result<WorkflowTaskResponse> {
        let body = serde_json::json!({ "task_id": task_id });
        self.post("/topapi/process/gettask", access_token, &body)
            .await
    }

    pub async fn workflow_execute_task(
        &self,
        access_token: &str,
        request: &WorkflowExecuteTaskRequest,
    ) -> Result<WorkflowExecuteTaskResponse> {
        self.post("/topapi/process/execute", access_token, request)
            .await
    }

    pub async fn workflow_batch_execute_task(
        &self,
        access_token: &str,
        request: &WorkflowBatchExecuteTaskRequest,
    ) -> Result<WorkflowBatchExecuteTaskResponse> {
        self.post("/topapi/process/batchexecute", access_token, request)
            .await
    }

    pub async fn workflow_get_process_instance(
        &self,
        access_token: &str,
        process_instance_id: &str,
    ) -> Result<ProcessInstance> {
        let body = serde_json::json!({ "process_instance_id": process_instance_id });
        self.post("/topapi/process/instance/get", access_token, &body)
            .await
    }

    pub async fn workflow_create_process_instance(
        &self,
        access_token: &str,
        request: &CreateProcessInstanceRequest,
    ) -> Result<CreateProcessInstanceResponse> {
        self.post("/topapi/process/instance/create", access_token, request)
            .await
    }

    pub async fn workflow_register_callback(
        &self,
        access_token: &str,
        request: &RegisterCallbackRequest,
    ) -> Result<RegisterCallbackResponse> {
        self.post("/topapi/process/callback/register", access_token, request)
            .await
    }

    pub async fn workflow_update_callback(
        &self,
        access_token: &str,
        request: &UpdateCallbackRequest,
    ) -> Result<()> {
        self.post("/topapi/process/callback/update", access_token, request)
            .await
    }

    pub async fn workflow_delete_callback(
        &self,
        access_token: &str,
        request: &DeleteCallbackRequest,
    ) -> Result<()> {
        self.post("/topapi/process/callback/delete", access_token, request)
            .await
    }

    pub async fn workflow_get_callback(
        &self,
        access_token: &str,
        request: &GetCallbackRequest,
    ) -> Result<GetCallbackResponse> {
        self.post("/topapi/process/callback/get", access_token, request)
            .await
    }

    pub async fn workflow_get_form_data(
        &self,
        access_token: &str,
        process_instance_id: &str,
    ) -> Result<FormDataResponse> {
        let body = serde_json::json!({ "process_instance_id": process_instance_id });
        self.post("/topapi/process/instance/formdata/get", access_token, &body)
            .await
    }

    pub async fn workflow_get_code_list(
        &self,
        access_token: &str,
        request: &GetCodeListRequest,
    ) -> Result<GetCodeListResponse> {
        self.post("/topapi/process/listbycreator", access_token, request)
            .await
    }

    pub async fn workflow_get_todo_list(
        &self,
        access_token: &str,
        request: &GetTodoListRequest,
    ) -> Result<GetTodoListResponse> {
        self.post("/topapi/process/gettodolist", access_token, request)
            .await
    }

    pub async fn workflow_get_todo_num(
        &self,
        access_token: &str,
        user_id: &str,
    ) -> Result<GetTodoNumResponse> {
        let body = serde_json::json!({ "userid": user_id });
        self.post("/topapi/process/gettodonum", access_token, &body)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCreateRequest {
    pub process_code: String,
    pub originator_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCreateResponse {
    pub request_id: String,
    pub business_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGetResponse {
    pub process_instance: ProcessInstance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInstance {
    pub business_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_records: Option<Vec<OperationRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_dept_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_finish_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListResponse {
    pub result: WorkflowListResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ProcessInstance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCopyResponse {
    pub request_id: String,
    pub business_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTerminateRequest {
    pub process_instance_id: String,
    pub is_system: bool,
    pub remark: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListByCreatorRequest {
    pub userid: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_code_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListByCreatorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ProcessInstance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListByStaffRequest {
    pub userid: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_code_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowListByStaffResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ProcessInstance>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowUpdateRequest {
    pub process_instance_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecuteTaskRequest {
    pub process_instance_id: String,
    pub tasks: Vec<TaskExecuteInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskExecuteInfo {
    pub task_id: String,
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecuteTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowBatchExecuteTaskRequest {
    pub process_instance_ids: Vec<String>,
    pub tasks: Vec<TaskExecuteInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowBatchExecuteTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_process_instance_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProcessInstanceRequest {
    pub process_code: String,
    pub originator_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<ApproverInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_select_action: Option<Vec<TargetSelectAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproverInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetSelectAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<ActionInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProcessInstanceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterCallbackRequest {
    pub call_back_tag: Vec<String>,
    pub token: String,
    pub aes_key: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterCallbackResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_back_list: Option<Vec<CallbackInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_back_tag: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aes_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCallbackRequest {
    pub call_back_tag: Vec<String>,
    pub token: String,
    pub aes_key: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCallbackRequest {
    pub call_back_tag: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCallbackRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_back_tag: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCallbackResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_back_list: Option<Vec<CallbackInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDataResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_component_values: Option<Vec<FormField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ProcessInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCodeListRequest {
    pub userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCodeListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ProcessCodeInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessCodeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_uuid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTodoListRequest {
    pub userid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_code_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTodoListResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_list: Option<Vec<TodoTaskInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoTaskInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTodoNumResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todo_num: Option<i64>,
}
