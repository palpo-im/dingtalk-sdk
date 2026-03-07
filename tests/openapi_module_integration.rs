use dingtalk_sdk::DingTalkClient;
use dingtalk_sdk::calendar::{
    AddCalendarAttendeesRequest, AddCalendarMeetingRoomsRequest, CalendarAttendeeRef,
    CalendarMeetingRoomRef, CancelCalendarEventRequest, DeleteCalendarEventRequest,
    GetCalendarEventRequest, ListCalendarAttendeesRequest, ListCalendarEventInstancesRequest,
    ListCalendarEventsRequest, MeetingRoomRespondRequest, QueryMeetingRoomSchedulesRequest,
    RemoveCalendarAttendeesRequest, RemoveCalendarMeetingRoomsRequest,
};
use dingtalk_sdk::check_in::GetCheckinRecordByUserRequest;
use dingtalk_sdk::crm::{DeleteCrmPersonalCustomerQuery, ListCrmPersonalCustomersQuery};
use dingtalk_sdk::doc::{DocAddCommentRequest, DocListCommentsRequest, DocWorkspaceMembersRequest};
use dingtalk_sdk::exclusive::{
    DeleteExclusiveTrustedDeviceRequest, SetExclusiveConversationCategoryRequest,
};
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn calendar_list_calendars_uses_openapi_header_auth() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1.0/calendar/users/u1/calendars"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "response": {
                "calendars": [
                    { "id": "c1", "summary": "Team Calendar" }
                ]
            }
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let response = client
        .calendar_list_calendars("token-123", "u1")
        .await
        .expect("calendar list should succeed");

    assert_eq!(response.response.calendars.len(), 1);
}

#[tokio::test]
async fn calendar_crud_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1.0/calendar/users/u1/calendars/c1/events"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "summary": "Demo" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "e1",
            "summary": "Demo"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1.0/calendar/users/u1/calendars/c1/events/e1"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("maxAttendees", "20"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "e1",
            "summary": "Demo"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1.0/calendar/users/u1/calendars/c1/events"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("maxResults", "50"))
        .and(query_param("showDeleted", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "events": [],
            "nextToken": "n1",
            "syncToken": "s1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1.0/calendar/users/u1/calendars/c1/events/e1"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "summary": "Updated" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "e1",
            "summary": "Updated"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/cancel",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("scope", "all"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/v1.0/calendar/users/u1/calendars/c1/events/e1"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("pushNotification", "true"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let created = client
        .calendar_create_event(
            "token-123",
            "u1",
            "c1",
            &serde_json::json!({ "summary": "Demo" }),
        )
        .await
        .expect("create should succeed");
    assert_eq!(created["id"], "e1");

    let fetched = client
        .calendar_get_event(
            "token-123",
            "u1",
            "c1",
            "e1",
            Some(&GetCalendarEventRequest {
                max_attendees: Some(20),
            }),
        )
        .await
        .expect("get should succeed");
    assert_eq!(fetched["id"], "e1");

    let listed = client
        .calendar_list_events(
            "token-123",
            "u1",
            "c1",
            Some(&ListCalendarEventsRequest {
                max_results: Some(50),
                show_deleted: Some(true),
                ..Default::default()
            }),
        )
        .await
        .expect("list should succeed");
    assert_eq!(listed.next_token.as_deref(), Some("n1"));

    let updated = client
        .calendar_update_event(
            "token-123",
            "u1",
            "c1",
            "e1",
            &serde_json::json!({ "summary": "Updated" }),
        )
        .await
        .expect("update should succeed");
    assert_eq!(updated["summary"], "Updated");

    let cancel_result = client
        .calendar_cancel_event(
            "token-123",
            "u1",
            "c1",
            "e1",
            Some(&CancelCalendarEventRequest {
                scope: Some("all".to_string()),
            }),
        )
        .await
        .expect("cancel should succeed");
    assert!(cancel_result.result);

    client
        .calendar_delete_event(
            "token-123",
            "u1",
            "c1",
            "e1",
            Some(&DeleteCalendarEventRequest {
                push_notification: Some(true),
            }),
        )
        .await
        .expect("delete should succeed");
}

#[tokio::test]
async fn check_in_query_uses_openapi_contract() {
    let server = MockServer::start().await;

    let request = GetCheckinRecordByUserRequest {
        end_time: 1700000100,
        max_results: 50,
        next_token: 0,
        operator_user_id: "manager1".to_string(),
        start_time: 1700000000,
        user_id_list: vec!["user1".to_string()],
    };

    Mock::given(method("POST"))
        .and(path("/v1.0/checkIn/records/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "endTime": 1700000100,
            "maxResults": 50,
            "nextToken": 0,
            "operatorUserId": "manager1",
            "startTime": 1700000000,
            "userIdList": ["user1"]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": {
                "nextToken": 1,
                "pageList": [
                    { "userId": "user1", "checkinTime": 1700000050 }
                ]
            }
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let response = client
        .check_in_get_checkin_record_by_user("token-123", &request)
        .await
        .expect("query should succeed");
    assert_eq!(response.result.next_token, Some(1));
    assert_eq!(response.result.page_list.len(), 1);
}

#[tokio::test]
async fn crm_core_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1.0/crm/personalCustomers"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "name": "Acme Corp" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "instanceId": "inst-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/crm/personalCustomers/batchQuery"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("currentOperatorUserId", "operator1"))
        .and(query_param("relationType", "customer"))
        .and(body_json(serde_json::json!({ "body": {} })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": []
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/crm/leads"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "name": "Lead A" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "outLeadsId": "lead-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/crm/leads/remove"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "outLeadsIds": ["lead-1"] })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/crm/customerInstances"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "maxResults": 10 })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "values": [] }
        })))
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/v1.0/crm/personalCustomers/data-1"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("currentOperatorUserId", "operator1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let added = client
        .crm_add_personal_customer("token-123", &serde_json::json!({ "name": "Acme Corp" }))
        .await
        .expect("add personal customer should succeed");
    assert_eq!(added.instance_id, "inst-1");

    let listed = client
        .crm_list_personal_customers(
            "token-123",
            Some(&ListCrmPersonalCustomersQuery {
                current_operator_user_id: Some("operator1".to_string()),
                relation_type: Some("customer".to_string()),
            }),
            Some(&serde_json::json!({ "body": {} })),
        )
        .await
        .expect("list personal customers should succeed");
    assert!(listed.get("result").is_some());

    let leads = client
        .crm_add_leads("token-123", &serde_json::json!({ "name": "Lead A" }))
        .await
        .expect("add leads should succeed");
    assert_eq!(leads["outLeadsId"], "lead-1");

    let delete_leads = client
        .crm_delete_leads(
            "token-123",
            &serde_json::json!({ "outLeadsIds": ["lead-1"] }),
        )
        .await
        .expect("delete leads should succeed");
    assert_eq!(delete_leads["success"], true);

    let queried = client
        .crm_query_all_customers("token-123", &serde_json::json!({ "maxResults": 10 }))
        .await
        .expect("query all customers should succeed");
    assert!(queried.get("result").is_some());

    client
        .crm_delete_personal_customer(
            "token-123",
            "data-1",
            Some(&DeleteCrmPersonalCustomerQuery {
                current_operator_user_id: Some("operator1".to_string()),
            }),
        )
        .await
        .expect("delete personal customer should succeed");
}

#[tokio::test]
async fn calendar_attendee_and_room_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/attendees",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "attendeesToAdd": [{ "id": "u2", "isOptional": true }],
            "chatNotification": true,
            "pushNotification": true
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/attendees/batchRemove",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "attendeesToRemove": [{ "id": "u2" }]
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/attendees",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("maxResults", "100"))
        .and(query_param("nextToken", "t1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "attendees": [{ "id": "u2" }],
            "nextToken": "t2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/meetingRooms",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "meetingRoomsToAdd": [{ "roomId": "r1" }],
            "pushNotification": true
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/meetingRooms/batchRemove",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "meetingRoomsToRemove": [{ "roomId": "r1" }]
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/calendar/users/u1/meetingRooms/schedules/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "roomIds": ["r1"],
            "maxResults": 10,
            "timeMin": "2026-03-01T00:00:00Z",
            "timeMax": "2026-03-02T00:00:00Z"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "schedules": [{ "roomId": "r1", "busy": [] }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/meetingRooms/r1/respond",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "responseStatus": "accepted"
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/instances",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("maxResults", "10"))
        .and(query_param("timeMin", "2026-03-01T00:00:00Z"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "events": [{ "id": "e1-1" }],
            "nextToken": "n2"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    client
        .calendar_add_attendees(
            "token-123",
            "u1",
            "c1",
            "e1",
            &AddCalendarAttendeesRequest {
                attendees_to_add: vec![CalendarAttendeeRef {
                    id: "u2".to_string(),
                    is_optional: Some(true),
                }],
                chat_notification: Some(true),
                push_notification: Some(true),
            },
        )
        .await
        .expect("add attendees should succeed");

    client
        .calendar_remove_attendees(
            "token-123",
            "u1",
            "c1",
            "e1",
            &RemoveCalendarAttendeesRequest {
                attendees_to_remove: vec![CalendarAttendeeRef {
                    id: "u2".to_string(),
                    is_optional: None,
                }],
            },
        )
        .await
        .expect("remove attendees should succeed");

    let attendees = client
        .calendar_list_attendees(
            "token-123",
            "u1",
            "c1",
            "e1",
            Some(&ListCalendarAttendeesRequest {
                max_results: Some(100),
                next_token: Some("t1".to_string()),
            }),
        )
        .await
        .expect("list attendees should succeed");
    assert_eq!(attendees.next_token.as_deref(), Some("t2"));

    client
        .calendar_add_meeting_rooms(
            "token-123",
            "u1",
            "c1",
            "e1",
            &AddCalendarMeetingRoomsRequest {
                meeting_rooms_to_add: vec![CalendarMeetingRoomRef {
                    room_id: "r1".to_string(),
                }],
                push_notification: Some(true),
            },
        )
        .await
        .expect("add meeting rooms should succeed");

    client
        .calendar_remove_meeting_rooms(
            "token-123",
            "u1",
            "c1",
            "e1",
            &RemoveCalendarMeetingRoomsRequest {
                meeting_rooms_to_remove: vec![CalendarMeetingRoomRef {
                    room_id: "r1".to_string(),
                }],
            },
        )
        .await
        .expect("remove meeting rooms should succeed");

    let schedules = client
        .calendar_query_meeting_room_schedules(
            "token-123",
            "u1",
            &QueryMeetingRoomSchedulesRequest {
                room_ids: vec!["r1".to_string()],
                max_results: Some(10),
                next_token: None,
                time_max: Some("2026-03-02T00:00:00Z".to_string()),
                time_min: Some("2026-03-01T00:00:00Z".to_string()),
            },
        )
        .await
        .expect("query meeting room schedules should succeed");
    assert!(schedules.data.get("schedules").is_some());

    client
        .calendar_meeting_room_respond(
            "token-123",
            "u1",
            "c1",
            "e1",
            "r1",
            &MeetingRoomRespondRequest {
                response_status: "accepted".to_string(),
            },
        )
        .await
        .expect("meeting room respond should succeed");

    let instances = client
        .calendar_list_event_instances(
            "token-123",
            "u1",
            "c1",
            "e1",
            Some(&ListCalendarEventInstancesRequest {
                max_results: Some(10),
                next_token: None,
                time_max: None,
                time_min: Some("2026-03-01T00:00:00Z".to_string()),
            }),
        )
        .await
        .expect("list event instances should succeed");
    assert_eq!(instances.next_token.as_deref(), Some("n2"));
}

#[tokio::test]
async fn doc_collaboration_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1.0/doc/docs/d1/comments"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("operatorId", "u1"))
        .and(body_json(serde_json::json!({
            "commentContent": "LGTM",
            "commentType": "TEXT",
            "option": { "replyTo": "root" }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "commentId": "c1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1.0/doc/docs/d1/comments"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("operatorId", "u1"))
        .and(query_param("maxResults", "20"))
        .and(query_param("nextToken", "n1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "comments": [{ "id": "c1" }],
            "nextToken": "n2"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/doc/workspaces/w1/docs/n1/members"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "members": [{ "id": "u1", "role": "editor" }],
            "operatorId": "admin"
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1.0/doc/workspaces/w1/docs/n1/members"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "members": [{ "id": "u1", "role": "owner" }],
            "operatorId": "admin"
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/doc/workspaces/w1/docs/n1/members/remove"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "members": [{ "id": "u1" }],
            "operatorId": "admin"
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/doc/workspaces"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "name": "Engineering"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "workspaceId": "w1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/doc/workspaces/w1/docs"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "name": "Architecture",
            "type": "doc"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "docId": "d2"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let added_comment = client
        .doc_add_comment_openapi(
            "token-123",
            "d1",
            &DocAddCommentRequest {
                operator_id: "u1".to_string(),
                comment_content: "LGTM".to_string(),
                comment_type: "TEXT".to_string(),
                option: Some(serde_json::json!({ "replyTo": "root" })),
            },
        )
        .await
        .expect("add comment should succeed");
    assert_eq!(added_comment.comment_id.as_deref(), Some("c1"));

    let comments = client
        .doc_list_comments_openapi(
            "token-123",
            "d1",
            Some(&DocListCommentsRequest {
                operator_id: Some("u1".to_string()),
                max_results: Some(20),
                next_token: Some("n1".to_string()),
            }),
        )
        .await
        .expect("list comments should succeed");
    assert_eq!(comments.next_token.as_deref(), Some("n2"));

    client
        .doc_add_workspace_doc_members(
            "token-123",
            "w1",
            "n1",
            &DocWorkspaceMembersRequest {
                members: vec![serde_json::json!({ "id": "u1", "role": "editor" })],
                operator_id: "admin".to_string(),
            },
        )
        .await
        .expect("add doc members should succeed");

    client
        .doc_update_workspace_doc_members(
            "token-123",
            "w1",
            "n1",
            &DocWorkspaceMembersRequest {
                members: vec![serde_json::json!({ "id": "u1", "role": "owner" })],
                operator_id: "admin".to_string(),
            },
        )
        .await
        .expect("update doc members should succeed");

    client
        .doc_delete_workspace_doc_members(
            "token-123",
            "w1",
            "n1",
            &DocWorkspaceMembersRequest {
                members: vec![serde_json::json!({ "id": "u1" })],
                operator_id: "admin".to_string(),
            },
        )
        .await
        .expect("delete doc members should succeed");

    let workspace = client
        .doc_create_workspace_openapi("token-123", &serde_json::json!({ "name": "Engineering" }))
        .await
        .expect("create workspace should succeed");
    assert_eq!(workspace["workspaceId"], "w1");

    let doc = client
        .doc_create_workspace_doc_openapi(
            "token-123",
            "w1",
            &serde_json::json!({ "name": "Architecture", "type": "doc" }),
        )
        .await
        .expect("create workspace doc should succeed");
    assert_eq!(doc["docId"], "d2");
}

#[tokio::test]
async fn exclusive_datacenter_and_industry_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1.0/exclusive/benefits"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "benefits": [{ "id": "b1" }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1.0/exclusive/conversationCategories"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "categories": [{ "categoryId": 1, "name": "Important" }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/exclusive/conversationCategories/set"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "openConversationId": "cid-1",
            "categoryId": 1
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/exclusive/follow/message/send"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "content": "hello"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "taskId": "task-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/exclusive/trustedDevices/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "cursor": 0
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "devices": [{ "deviceId": "dev-1" }],
            "nextToken": "n1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/exclusive/trustedDevices"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "deviceName": "Laptop"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "deviceId": "dev-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1.0/exclusive/trustedDevices/dev-1"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "deviceName": "Laptop-2"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "updated": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/exclusive/trustedDevices/remove"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "deviceIds": ["dev-1"]
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/activeUserData"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "date": "2026-03-01" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "count": 12 }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/attendanceData"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "date": "2026-03-01" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "count": 9 }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/reportData"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "date": "2026-03-01" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "count": 5 }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/chartDatas/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "chartId": "c1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": [{ "x": 1, "y": 2 }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/screens"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "name": "Overview" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "screenId": "s1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/screens"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "screenId": "s1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "screenId": "s1", "name": "Overview" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/generalDataServices"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "serviceCode": "svc-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": { "serviceCode": "svc-1" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/datacenter/datas/totalCounts/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "datasetCode": "d1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "total": 100
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/retail/product/add"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "productCode": "p1",
            "productName": "Desk"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/retail/product/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "productCode": "p1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "product": { "productCode": "p1" }
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/retail/product/update"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "productCode": "p1", "productName": "Desk v2" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/retail/product/delete"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "productCode": "p1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/retail/product/image/upload"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "fileId": "f1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "imageUrl": "https://example.com/f1.png"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/chatai/abilities/sentiments/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "content": "great service" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "sentiment": "positive"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/chatmemo/faq"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "question": "Q1", "answer": "A1" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "faqId": "faq-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/chatmemo/faq/lists"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "cursor": 0 })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "list": [{ "faqId": "faq-1" }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/chatmemo/faq/remove"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "faqId": "faq-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/industry/ai/taskQueue/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "taskId": "task-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "running"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let benefits = client
        .exclusive_query_benefits("token-123")
        .await
        .expect("exclusive benefits should succeed");
    assert!(benefits.get("benefits").is_some());

    let categories = client
        .exclusive_get_conversation_categories("token-123")
        .await
        .expect("conversation categories should succeed");
    assert_eq!(categories.categories.len(), 1);

    client
        .exclusive_set_conversation_category(
            "token-123",
            &SetExclusiveConversationCategoryRequest {
                open_conversation_id: "cid-1".to_string(),
                category_id: 1,
            },
        )
        .await
        .expect("set conversation category should succeed");

    let send_message = client
        .exclusive_send_message("token-123", &serde_json::json!({ "content": "hello" }))
        .await
        .expect("exclusive send message should succeed");
    assert_eq!(send_message["taskId"], "task-1");

    let trusted_devices = client
        .exclusive_query_trusted_devices("token-123", &serde_json::json!({ "cursor": 0 }))
        .await
        .expect("query trusted devices should succeed");
    assert_eq!(trusted_devices.next_token.as_deref(), Some("n1"));

    let created_device = client
        .exclusive_create_trusted_device(
            "token-123",
            &serde_json::json!({ "deviceName": "Laptop" }),
        )
        .await
        .expect("create trusted device should succeed");
    assert_eq!(created_device["deviceId"], "dev-1");

    let updated_device = client
        .exclusive_update_trusted_device(
            "token-123",
            "dev-1",
            &serde_json::json!({ "deviceName": "Laptop-2" }),
        )
        .await
        .expect("update trusted device should succeed");
    assert_eq!(updated_device["updated"], true);

    client
        .exclusive_delete_trusted_device(
            "token-123",
            &DeleteExclusiveTrustedDeviceRequest {
                device_ids: vec!["dev-1".to_string()],
            },
        )
        .await
        .expect("delete trusted device should succeed");

    let active_users = client
        .datacenter_query_active_user_data(
            "token-123",
            &serde_json::json!({ "date": "2026-03-01" }),
        )
        .await
        .expect("active user data should succeed");
    assert_eq!(active_users["result"]["count"], 12);

    let attendance = client
        .datacenter_query_attendance_data("token-123", &serde_json::json!({ "date": "2026-03-01" }))
        .await
        .expect("attendance data should succeed");
    assert_eq!(attendance["result"]["count"], 9);

    let report = client
        .datacenter_query_report_data("token-123", &serde_json::json!({ "date": "2026-03-01" }))
        .await
        .expect("report data should succeed");
    assert_eq!(report["result"]["count"], 5);

    let chart = client
        .datacenter_query_chart_data("token-123", &serde_json::json!({ "chartId": "c1" }))
        .await
        .expect("chart data should succeed");
    assert!(chart.get("result").is_some());

    let screen = client
        .datacenter_create_screen("token-123", &serde_json::json!({ "name": "Overview" }))
        .await
        .expect("create screen should succeed");
    assert_eq!(screen["screenId"], "s1");

    let queried_screen = client
        .datacenter_query_screen("token-123", &serde_json::json!({ "screenId": "s1" }))
        .await
        .expect("query screen should succeed");
    assert_eq!(queried_screen["result"]["screenId"], "s1");

    let general_service = client
        .datacenter_query_general_data_service(
            "token-123",
            &serde_json::json!({ "serviceCode": "svc-1" }),
        )
        .await
        .expect("general data service should succeed");
    assert_eq!(general_service["result"]["serviceCode"], "svc-1");

    let total_count = client
        .datacenter_query_total_data_count("token-123", &serde_json::json!({ "datasetCode": "d1" }))
        .await
        .expect("total data count should succeed");
    assert_eq!(total_count["total"], 100);

    let add_product = client
        .industry_ai_retail_product_add(
            "token-123",
            &serde_json::json!({ "productCode": "p1", "productName": "Desk" }),
        )
        .await
        .expect("retail add should succeed");
    assert_eq!(add_product["success"], true);

    let query_product = client
        .industry_ai_retail_product_query("token-123", &serde_json::json!({ "productCode": "p1" }))
        .await
        .expect("retail query should succeed");
    assert_eq!(query_product["product"]["productCode"], "p1");

    let update_product = client
        .industry_ai_retail_product_update(
            "token-123",
            &serde_json::json!({ "productCode": "p1", "productName": "Desk v2" }),
        )
        .await
        .expect("retail update should succeed");
    assert_eq!(update_product["success"], true);

    let delete_product = client
        .industry_ai_retail_product_delete("token-123", &serde_json::json!({ "productCode": "p1" }))
        .await
        .expect("retail delete should succeed");
    assert_eq!(delete_product["success"], true);

    let upload_image = client
        .industry_ai_retail_product_image_upload(
            "token-123",
            &serde_json::json!({ "fileId": "f1" }),
        )
        .await
        .expect("retail image upload should succeed");
    assert!(upload_image.get("imageUrl").is_some());

    let sentiment = client
        .industry_chatai_sentiment_query(
            "token-123",
            &serde_json::json!({ "content": "great service" }),
        )
        .await
        .expect("sentiment query should succeed");
    assert_eq!(sentiment["sentiment"], "positive");

    let faq_add = client
        .industry_chatmemo_faq_add(
            "token-123",
            &serde_json::json!({ "question": "Q1", "answer": "A1" }),
        )
        .await
        .expect("faq add should succeed");
    assert_eq!(faq_add["faqId"], "faq-1");

    let faq_list = client
        .industry_chatmemo_faq_list("token-123", &serde_json::json!({ "cursor": 0 }))
        .await
        .expect("faq list should succeed");
    assert_eq!(faq_list["list"][0]["faqId"], "faq-1");

    let faq_remove = client
        .industry_chatmemo_faq_remove("token-123", &serde_json::json!({ "faqId": "faq-1" }))
        .await
        .expect("faq remove should succeed");
    assert_eq!(faq_remove["success"], true);

    let task_queue = client
        .industry_task_queue_query("token-123", &serde_json::json!({ "taskId": "task-1" }))
        .await
        .expect("task queue query should succeed");
    assert_eq!(task_queue["status"], "running");
}

#[tokio::test]
async fn yida_openapi_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1.0/yida/authorizations/appLoginCodes"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "userId": "u1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "loginCode": "lc-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/yida/forms/instances/ids/query"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "formUuid": "f1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": [{ "id": "inst-1" }]
        })))
        .mount(&server)
        .await;

    Mock::given(method("PUT"))
        .and(path("/v1.0/yida/forms/instances/components"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "instanceId": "inst-1", "componentValue": "v1" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/yida/tasks/execute"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "taskId": "task-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "taskStatus": "done"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let login_code = client
        .yida_app_login_code_gen("token-123", &serde_json::json!({ "userId": "u1" }))
        .await
        .expect("app login code should succeed");
    assert_eq!(login_code["loginCode"], "lc-1");

    let ids = client
        .yida_batch_get_form_data_by_id_list("token-123", &serde_json::json!({ "formUuid": "f1" }))
        .await
        .expect("batch get ids should succeed");
    assert_eq!(ids["result"][0]["id"], "inst-1");

    let update = client
        .yida_batch_update_form_data_by_instance_id(
            "token-123",
            &serde_json::json!({ "instanceId": "inst-1", "componentValue": "v1" }),
        )
        .await
        .expect("batch update by instance id should succeed");
    assert_eq!(update["success"], true);

    let task = client
        .yida_execute_task("token-123", &serde_json::json!({ "taskId": "task-1" }))
        .await
        .expect("execute task should succeed");
    assert_eq!(task["taskStatus"], "done");
}

#[tokio::test]
async fn bizfinance_openapi_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/v1.0/bizfinance/roles/permissions"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "roleCode": "financeManager" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/bizfinance/invoices/batch"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "invoices": [{ "invoiceNo": "INV-1" }] }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "successCount": 1
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/bizfinance/consumedBenefits/prepare"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "benefitId": "benefit-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "consumeId": "consume-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/bizfinance/receipts/remove"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "receiptId": "receipt-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": true
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let permission = client
        .bizfinance_append_role_permission(
            "token-123",
            &serde_json::json!({ "roleCode": "financeManager" }),
        )
        .await
        .expect("append role permission should succeed");
    assert_eq!(permission["result"], true);

    let invoices = client
        .bizfinance_batch_add_invoice(
            "token-123",
            &serde_json::json!({ "invoices": [{ "invoiceNo": "INV-1" }] }),
        )
        .await
        .expect("batch add invoice should succeed");
    assert_eq!(invoices["successCount"], 1);

    let consume = client
        .bizfinance_begin_consume(
            "token-123",
            &serde_json::json!({ "benefitId": "benefit-1" }),
        )
        .await
        .expect("begin consume should succeed");
    assert_eq!(consume["consumeId"], "consume-1");

    let deleted = client
        .bizfinance_delete_receipt(
            "token-123",
            &serde_json::json!({ "receiptId": "receipt-1" }),
        )
        .await
        .expect("delete receipt should succeed");
    assert_eq!(deleted["result"], true);
}

#[tokio::test]
async fn edu_openapi_methods_match_reference_paths() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1.0/edu/vpaas/devices/activate"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "sn": "SN-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": true
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/edu/cards"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "cards": [{ "name": "c1" }] }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "taskId": "task-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/edu/orders"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(
            serde_json::json!({ "userId": "u1", "skuId": "sku-1" }),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "orderId": "order-1"
        })))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1.0/edu/orders/cancel"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({ "orderId": "order-1" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let activated = client
        .edu_activate_device("token-123", &serde_json::json!({ "sn": "SN-1" }))
        .await
        .expect("activate device should succeed");
    assert_eq!(activated["result"], true);

    let cards = client
        .edu_batch_create_cards(
            "token-123",
            &serde_json::json!({ "cards": [{ "name": "c1" }] }),
        )
        .await
        .expect("batch create cards should succeed");
    assert_eq!(cards["taskId"], "task-1");

    let order = client
        .edu_create_order(
            "token-123",
            &serde_json::json!({ "userId": "u1", "skuId": "sku-1" }),
        )
        .await
        .expect("create order should succeed");
    assert_eq!(order["orderId"], "order-1");

    let cancel = client
        .edu_cancel_order("token-123", &serde_json::json!({ "orderId": "order-1" }))
        .await
        .expect("cancel order should succeed");
    assert_eq!(cancel["success"], true);
}
