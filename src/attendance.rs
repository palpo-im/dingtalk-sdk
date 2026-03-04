//! DingTalk API bindings for the attendance module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::models::*;

impl DingTalkClient {
    pub async fn get_attendance_list(
        &self,
        access_token: &str,
        work_date_from: &str,
        work_date_to: &str,
        user_ids: &[&str],
        offset: i64,
        limit: i64,
    ) -> Result<GetAttendanceListResponse> {
        let body = serde_json::json!({
            "workDateFrom": work_date_from,
            "workDateTo": work_date_to,
            "userIdList": user_ids,
            "offset": offset,
            "limit": limit
        });
        self.post("/attendance/list", access_token, &body).await
    }

    pub async fn get_attendance_result(
        &self,
        access_token: &str,
        work_date_from: &str,
        work_date_to: &str,
        user_ids: &[&str],
        offset: i64,
        limit: i64,
    ) -> Result<GetAttendanceResultResponse> {
        let body = serde_json::json!({
            "workDateFrom": work_date_from,
            "workDateTo": work_date_to,
            "userIdList": user_ids,
            "offset": offset,
            "limit": limit
        });
        self.post("/attendance/getattendancerecord", access_token, &body)
            .await
    }

    pub async fn get_attendance_shift(
        &self,
        access_token: &str,
        op_user_id: &str,
        user_id: &str,
        date: &str,
    ) -> Result<GetAttendanceShiftResponse> {
        let body = serde_json::json!({
            "opUserId": op_user_id,
            "userId": user_id,
            "dateTime": date
        });
        self.post("/topapi/attendance/shift/get", access_token, &body)
            .await
    }

    pub async fn create_shift(
        &self,
        access_token: &str,
        request: &CreateShiftRequest,
    ) -> Result<CreateShiftResponse> {
        self.post("/topapi/attendance/shift/add", access_token, request)
            .await
    }

    pub async fn update_shift(
        &self,
        access_token: &str,
        request: &UpdateShiftRequest,
    ) -> Result<()> {
        self.post("/topapi/attendance/shift/update", access_token, request)
            .await
    }

    pub async fn delete_shift(&self, access_token: &str, shift_id: &str) -> Result<()> {
        let body = serde_json::json!({ "shift_id": shift_id });
        self.post("/topapi/attendance/shift/delete", access_token, &body)
            .await
    }

    pub async fn list_shifts(
        &self,
        access_token: &str,
        op_user_id: &str,
    ) -> Result<ListShiftsResponse> {
        let body = serde_json::json!({ "op_user_id": op_user_id });
        self.post("/topapi/attendance/shift/list", access_token, &body)
            .await
    }

    pub async fn get_shift(&self, access_token: &str, shift_id: &str) -> Result<ShiftDetail> {
        let body = serde_json::json!({ "shift_id": shift_id });
        self.post("/topapi/attendance/shift/detail", access_token, &body)
            .await
    }

    pub async fn set_user_shift(
        &self,
        access_token: &str,
        request: &SetUserShiftRequest,
    ) -> Result<()> {
        self.post("/topapi/attendance/shift/setbyuser", access_token, request)
            .await
    }

    pub async fn get_user_shift_by_day(
        &self,
        access_token: &str,
        user_id: &str,
        date: &str,
    ) -> Result<ShiftDetail> {
        let body = serde_json::json!({
            "userid": user_id,
            "date_time": date
        });
        self.post(
            "/topapi/attendance/usershift/getbyuser",
            access_token,
            &body,
        )
        .await
    }

    pub async fn create_leave_type(
        &self,
        access_token: &str,
        request: &CreateLeaveTypeRequest,
    ) -> Result<CreateLeaveTypeResponse> {
        self.post(
            "/topapi/attendance/leaverecords/leavetype/add",
            access_token,
            request,
        )
        .await
    }

    pub async fn update_leave_type(
        &self,
        access_token: &str,
        request: &UpdateLeaveTypeRequest,
    ) -> Result<()> {
        self.post(
            "/topapi/attendance/leaverecords/leavetype/update",
            access_token,
            request,
        )
        .await
    }

    pub async fn delete_leave_type(&self, access_token: &str, leave_code: &str) -> Result<()> {
        let body = serde_json::json!({ "leave_code": leave_code });
        self.post(
            "/topapi/attendance/leaverecords/leavetype/delete",
            access_token,
            &body,
        )
        .await
    }

    pub async fn list_leave_types(&self, access_token: &str) -> Result<ListLeaveTypesResponse> {
        let body = serde_json::json!({});
        self.post(
            "/topapi/attendance/leaverecords/leavetype/list",
            access_token,
            &body,
        )
        .await
    }

    pub async fn get_leave_records(
        &self,
        access_token: &str,
        request: &GetLeaveRecordsRequest,
    ) -> Result<GetLeaveRecordsResponse> {
        self.post(
            "/topapi/attendance/leaverecords/list",
            access_token,
            request,
        )
        .await
    }

    pub async fn create_leave_record(
        &self,
        access_token: &str,
        request: &CreateLeaveRecordRequest,
    ) -> Result<CreateLeaveRecordResponse> {
        self.post(
            "/topapi/attendance/leaverecords/create",
            access_token,
            request,
        )
        .await
    }

    pub async fn update_leave_record(
        &self,
        access_token: &str,
        request: &UpdateLeaveRecordRequest,
    ) -> Result<()> {
        self.post(
            "/topapi/attendance/leaverecords/update",
            access_token,
            request,
        )
        .await
    }

    pub async fn get_overtime_duration(
        &self,
        access_token: &str,
        request: &GetOvertimeDurationRequest,
    ) -> Result<GetOvertimeDurationResponse> {
        self.post(
            "/topapi/attendance/getovertimeduration",
            access_token,
            request,
        )
        .await
    }

    pub async fn get_overtime_records(
        &self,
        access_token: &str,
        request: &GetOvertimeRecordsRequest,
    ) -> Result<GetOvertimeRecordsResponse> {
        self.post(
            "/topapi/attendance/overtimeRecord/list",
            access_token,
            request,
        )
        .await
    }

    pub async fn create_field_report(
        &self,
        access_token: &str,
        request: &CreateFieldReportRequest,
    ) -> Result<CreateFieldReportResponse> {
        self.post_openapi(
            "/v1.0/attendance/fieldReports",
            access_token,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn get_field_report(
        &self,
        access_token: &str,
        report_id: &str,
    ) -> Result<FieldReport> {
        let path = format!("/v1.0/attendance/fieldReports/{report_id}");
        self.get_openapi(&path, access_token, None).await
    }

    pub async fn list_field_reports(
        &self,
        access_token: &str,
        request: &ListFieldReportsRequest,
    ) -> Result<ListFieldReportsResponse> {
        let mut query = HashMap::new();
        if let Some(v) = &request.user_id {
            query.insert("userId".to_string(), v.clone());
        }
        if let Some(v) = &request.start_time {
            query.insert("startTime".to_string(), v.clone());
        }
        if let Some(v) = &request.end_time {
            query.insert("endTime".to_string(), v.clone());
        }
        if let Some(v) = request.cursor {
            query.insert("cursor".to_string(), v.to_string());
        }
        if let Some(v) = request.size {
            query.insert("size".to_string(), v.to_string());
        }
        self.get_openapi("/v1.0/attendance/fieldReports", access_token, Some(&query))
            .await
    }

    pub async fn update_field_report(
        &self,
        access_token: &str,
        request: &UpdateFieldReportRequest,
    ) -> Result<()> {
        self.request_openapi_no_content(
            reqwest::Method::PUT,
            "/v1.0/attendance/fieldReports",
            access_token,
            None,
            Some(&serde_json::to_value(request)?),
        )
        .await
    }

    pub async fn delete_field_report(&self, access_token: &str, report_id: &str) -> Result<()> {
        let path = format!("/v1.0/attendance/fieldReports/{report_id}");
        self.delete_openapi_no_content(&path, access_token, None)
            .await
    }

    pub async fn get_attendance_update_result(
        &self,
        access_token: &str,
        request: &GetAttendanceUpdateResultRequest,
    ) -> Result<GetAttendanceUpdateResultResponse> {
        self.post(
            "/topapi/attendance/getattupdaterecord",
            access_token,
            request,
        )
        .await
    }

    pub async fn upload_attendance_file(
        &self,
        access_token: &str,
        request: &UploadAttendanceFileRequest,
    ) -> Result<UploadAttendanceFileResponse> {
        self.post("/topapi/attendance/upload", access_token, request)
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceListResponse {
    pub recordresult: Vec<AttendanceRecord>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_check_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_check_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceResultResponse {
    pub result: Vec<AttendanceResult>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceShiftResponse {
    pub result: AttendanceShift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceShift {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShiftRequest {
    pub shift_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_time_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_sections: Option<Vec<WorkSection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub across: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_duty_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_duty_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShiftResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShiftRequest {
    pub shift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_time_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_sections: Option<Vec<WorkSection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListShiftsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ShiftInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_time_minutes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_sections: Option<Vec<WorkSection>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserShiftRequest {
    pub user_id: String,
    pub shift_id: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeaveTypeRequest {
    pub leave_code: String,
    pub leave_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_view: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_unit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_reason_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeaveTypeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeaveTypeRequest {
    pub leave_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_view: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_unit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_reason_required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLeaveTypesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<LeaveType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_view: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_unit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaveRecordsRequest {
    pub user_id: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLeaveRecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<LeaveRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeaveRecordRequest {
    pub user_id: String,
    pub leave_code: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeaveRecordResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLeaveRecordRequest {
    pub leave_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOvertimeDurationRequest {
    pub user_id: String,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOvertimeDurationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<OvertimeDurationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvertimeDurationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday_duration: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOvertimeRecordsRequest {
    pub user_id: String,
    pub start_time: i64,
    pub end_time: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOvertimeRecordsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<OvertimeRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OvertimeRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldReportRequest {
    pub user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<FieldReportAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldReportAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFieldReportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<FieldReportAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFieldReportsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFieldReportsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<FieldReport>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFieldReportRequest {
    pub report_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceUpdateResultRequest {
    pub user_id: String,
    pub work_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttendanceUpdateResultResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<AttendanceUpdateRecord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceUpdateRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_check_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAttendanceFileRequest {
    pub file_name: String,
    pub file_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAttendanceFileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
}
