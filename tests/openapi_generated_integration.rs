use dingtalk_sdk::{DingTalkClient, Error};
use std::collections::HashMap;
use wiremock::matchers::{body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn generated_static_endpoint_uses_openapi_contract() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1.0/trip/processes/details"))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(query_param("processInstanceId", "pi-1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let mut query = HashMap::new();
    query.insert("processInstanceId".to_string(), "pi-1".to_string());

    let value = client
        .go_trip_1_0_get_travel_process_detail("token-123", None, Some(&query), None)
        .await
        .expect("generated method should succeed");

    assert_eq!(value, serde_json::Value::Null);
}

#[tokio::test]
async fn generated_dynamic_endpoint_renders_path_params() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/v1.0/calendar/users/u1/calendars/c1/events/e1/attendees",
        ))
        .and(header("x-acs-dingtalk-access-token", "token-123"))
        .and(body_json(serde_json::json!({
            "attendeesToAdd": [{"id": "u2"}]
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let mut path_params = HashMap::new();
    path_params.insert("user_id".to_string(), "u1".to_string());
    path_params.insert("calendar_id".to_string(), "c1".to_string());
    path_params.insert("event_id".to_string(), "e1".to_string());

    let body = serde_json::json!({
        "attendeesToAdd": [{"id": "u2"}]
    });

    let value = client
        .go_calendar_1_0_add_attendee("token-123", Some(&path_params), None, Some(&body))
        .await
        .expect("generated method should succeed");

    assert_eq!(value, serde_json::Value::Null);
}

#[tokio::test]
async fn generated_endpoint_returns_error_when_path_param_missing() {
    let client = DingTalkClient::builder()
        .build()
        .expect("client should build");

    let mut path_params = HashMap::new();
    path_params.insert("user_id".to_string(), "u1".to_string());
    path_params.insert("calendar_id".to_string(), "c1".to_string());

    let result = client
        .go_calendar_1_0_add_attendee("token-123", Some(&path_params), None, None)
        .await;

    match result {
        Err(Error::InvalidParam(message)) => {
            assert!(
                message.contains("event_id"),
                "unexpected message: {message}"
            );
        }
        other => panic!("expected InvalidParam, got {other:?}"),
    }
}
