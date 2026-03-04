//! DingTalk Calendar OpenAPI (v1.0) endpoints.

use crate::client::DingTalkClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCalendarEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<i64>,
}

impl GetCalendarEventRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = self.max_attendees {
            params.insert("maxAttendees".to_string(), value.to_string());
        }
        params
    }
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCalendarEventsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_master_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
}

impl ListCalendarEventsRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = self.max_attendees {
            params.insert("maxAttendees".to_string(), value.to_string());
        }
        if let Some(value) = self.max_results {
            params.insert("maxResults".to_string(), value.to_string());
        }
        if let Some(value) = &self.next_token {
            params.insert("nextToken".to_string(), value.clone());
        }
        if let Some(value) = &self.series_master_id {
            params.insert("seriesMasterId".to_string(), value.clone());
        }
        if let Some(value) = self.show_deleted {
            params.insert("showDeleted".to_string(), value.to_string());
        }
        if let Some(value) = &self.sync_token {
            params.insert("syncToken".to_string(), value.clone());
        }
        if let Some(value) = &self.time_max {
            params.insert("timeMax".to_string(), value.clone());
        }
        if let Some(value) = &self.time_min {
            params.insert("timeMin".to_string(), value.clone());
        }
        params
    }
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteCalendarEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notification: Option<bool>,
}

impl DeleteCalendarEventRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = self.push_notification {
            params.insert("pushNotification".to_string(), value.to_string());
        }
        params
    }
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelCalendarEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl CancelCalendarEventRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = &self.scope {
            params.insert("scope".to_string(), value.clone());
        }
        params
    }
}

/// Payload model used by attendee-related APIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarAttendeeRef {
    pub id: String,
    #[serde(rename = "isOptional", skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCalendarAttendeesRequest {
    #[serde(rename = "attendeesToAdd")]
    pub attendees_to_add: Vec<CalendarAttendeeRef>,
    #[serde(rename = "chatNotification", skip_serializing_if = "Option::is_none")]
    pub chat_notification: Option<bool>,
    #[serde(rename = "pushNotification", skip_serializing_if = "Option::is_none")]
    pub push_notification: Option<bool>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveCalendarAttendeesRequest {
    #[serde(rename = "attendeesToRemove")]
    pub attendees_to_remove: Vec<CalendarAttendeeRef>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCalendarAttendeesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

impl ListCalendarAttendeesRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = self.max_results {
            params.insert("maxResults".to_string(), value.to_string());
        }
        if let Some(value) = &self.next_token {
            params.insert("nextToken".to_string(), value.clone());
        }
        params
    }
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarAttendeesResponse {
    #[serde(default)]
    pub attendees: Vec<serde_json::Value>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Payload model used by meeting-room-related APIs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarMeetingRoomRef {
    #[serde(rename = "roomId")]
    pub room_id: String,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCalendarMeetingRoomsRequest {
    #[serde(rename = "meetingRoomsToAdd")]
    pub meeting_rooms_to_add: Vec<CalendarMeetingRoomRef>,
    #[serde(rename = "pushNotification", skip_serializing_if = "Option::is_none")]
    pub push_notification: Option<bool>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveCalendarMeetingRoomsRequest {
    #[serde(rename = "meetingRoomsToRemove")]
    pub meeting_rooms_to_remove: Vec<CalendarMeetingRoomRef>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryMeetingRoomSchedulesRequest {
    #[serde(rename = "roomIds")]
    pub room_ids: Vec<String>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "timeMax", skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    #[serde(rename = "timeMin", skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMeetingRoomSchedulesResponse {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoomRespondRequest {
    #[serde(rename = "responseStatus")]
    pub response_status: String,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListCalendarEventInstancesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_min: Option<String>,
}

impl ListCalendarEventInstancesRequest {
    fn to_query_params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        if let Some(value) = self.max_results {
            params.insert("maxResults".to_string(), value.to_string());
        }
        if let Some(value) = &self.next_token {
            params.insert("nextToken".to_string(), value.clone());
        }
        if let Some(value) = &self.time_max {
            params.insert("timeMax".to_string(), value.clone());
        }
        if let Some(value) = &self.time_min {
            params.insert("timeMin".to_string(), value.clone());
        }
        params
    }
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarEventInstancesResponse {
    #[serde(default)]
    pub events: Vec<serde_json::Value>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarListResponse {
    pub response: CalendarListData,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarListData {
    pub calendars: Vec<serde_json::Value>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCalendarEventsResponse {
    #[serde(default)]
    pub events: Vec<serde_json::Value>,
    #[serde(rename = "nextToken", skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "syncToken", skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}

/// Payload model used by this API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelCalendarEventResponse {
    pub result: bool,
}

impl DingTalkClient {
    /// Executes this API call.
    pub async fn calendar_list_calendars(
        &self,
        access_token: &str,
        user_id: &str,
    ) -> Result<CalendarListResponse> {
        let path = format!("/v1.0/calendar/users/{user_id}/calendars");
        self.get_openapi(&path, access_token, None).await
    }

    /// Executes this API call.
    pub async fn calendar_create_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path = format!("/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events");
        self.post_openapi(&path, access_token, Some(request)).await
    }

    /// Executes this API call.
    pub async fn calendar_get_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: Option<&GetCalendarEventRequest>,
    ) -> Result<serde_json::Value> {
        let path =
            format!("/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}");
        let query = request
            .map(GetCalendarEventRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.get_openapi(&path, access_token, query_ref).await
    }

    /// Executes this API call.
    pub async fn calendar_list_events(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        request: Option<&ListCalendarEventsRequest>,
    ) -> Result<ListCalendarEventsResponse> {
        let path = format!("/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events");
        let query = request
            .map(ListCalendarEventsRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.get_openapi(&path, access_token, query_ref).await
    }

    /// Executes this API call.
    pub async fn calendar_update_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        let path =
            format!("/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}");
        self.put_openapi(&path, access_token, Some(request)).await
    }

    /// Executes this API call.
    pub async fn calendar_delete_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: Option<&DeleteCalendarEventRequest>,
    ) -> Result<()> {
        let path =
            format!("/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}");
        let query = request
            .map(DeleteCalendarEventRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.delete_openapi_no_content(&path, access_token, query_ref)
            .await
    }

    /// Executes this API call.
    pub async fn calendar_cancel_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: Option<&CancelCalendarEventRequest>,
    ) -> Result<CancelCalendarEventResponse> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/cancel"
        );
        let query = request
            .map(CancelCalendarEventRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.request_openapi(
            Method::POST,
            &path,
            access_token,
            query_ref,
            None::<&serde_json::Value>,
        )
        .await
    }

    /// Executes this API call.
    pub async fn calendar_create_recurring_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.calendar_create_event(access_token, user_id, calendar_id, request)
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_update_recurring_event(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.calendar_update_event(access_token, user_id, calendar_id, event_id, request)
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_list_event_instances(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: Option<&ListCalendarEventInstancesRequest>,
    ) -> Result<ListCalendarEventInstancesResponse> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/instances"
        );
        let query = request
            .map(ListCalendarEventInstancesRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.get_openapi(&path, access_token, query_ref)
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_add_attendees(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &AddCalendarAttendeesRequest,
    ) -> Result<()> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/attendees"
        );
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_remove_attendees(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &RemoveCalendarAttendeesRequest,
    ) -> Result<()> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/attendees/batchRemove"
        );
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_list_attendees(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: Option<&ListCalendarAttendeesRequest>,
    ) -> Result<ListCalendarAttendeesResponse> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/attendees"
        );
        let query = request
            .map(ListCalendarAttendeesRequest::to_query_params)
            .unwrap_or_default();
        let query_ref = if query.is_empty() { None } else { Some(&query) };
        self.get_openapi(&path, access_token, query_ref)
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_add_meeting_rooms(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &AddCalendarMeetingRoomsRequest,
    ) -> Result<()> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/meetingRooms"
        );
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_remove_meeting_rooms(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        request: &RemoveCalendarMeetingRoomsRequest,
    ) -> Result<()> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/meetingRooms/batchRemove"
        );
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_query_meeting_room_schedules(
        &self,
        access_token: &str,
        user_id: &str,
        request: &QueryMeetingRoomSchedulesRequest,
    ) -> Result<QueryMeetingRoomSchedulesResponse> {
        let path = format!("/v1.0/calendar/users/{user_id}/meetingRooms/schedules/query");
        let body = serde_json::to_value(request)?;
        self.request_openapi(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }

    /// Executes this API call.
    pub async fn calendar_meeting_room_respond(
        &self,
        access_token: &str,
        user_id: &str,
        calendar_id: &str,
        event_id: &str,
        room_id: &str,
        request: &MeetingRoomRespondRequest,
    ) -> Result<()> {
        let path = format!(
            "/v1.0/calendar/users/{user_id}/calendars/{calendar_id}/events/{event_id}/meetingRooms/{room_id}/respond"
        );
        let body = serde_json::to_value(request)?;
        self.request_openapi_no_content(Method::POST, &path, access_token, None, Some(&body))
            .await
            .map_err(|err| err.map_module("calendar"))
    }
}
