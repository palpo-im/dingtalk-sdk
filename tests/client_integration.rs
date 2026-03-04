use dingtalk_sdk::{DingTalkClient, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[derive(Debug, Serialize)]
struct EchoRequest {
    name: String,
}

#[derive(Debug, Deserialize)]
struct EchoResponse {
    echoed: String,
}

#[tokio::test]
async fn get_access_token_returns_token_on_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/gettoken"))
        .and(query_param("appkey", "test-key"))
        .and(query_param("appsecret", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "access_token": "token-123",
            "expires_in": 7200
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .credentials("test-key", "test-secret")
        .build()
        .expect("client should build");

    let token = client
        .get_access_token()
        .await
        .expect("token request should succeed");

    assert_eq!(token, "token-123");
}

#[tokio::test]
async fn post_includes_access_token_query_and_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/topapi/echo"))
        .and(query_param("access_token", "token-123"))
        .and(query_param("cursor", "10"))
        .and(body_json(serde_json::json!({ "name": "hello" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "echoed": "hello"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let body = EchoRequest {
        name: "hello".to_string(),
    };
    let mut query = HashMap::new();
    query.insert("cursor", "10");

    let response: EchoResponse = client
        .request(
            reqwest::Method::POST,
            "/topapi/echo",
            "token-123",
            Some(&query),
            Some(&body),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.echoed, "hello");
}

#[tokio::test]
async fn request_returns_api_error_when_errcode_is_non_zero() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/topapi/fail"))
        .and(query_param("access_token", "token-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 4001,
            "errmsg": "invalid request",
            "echoed": ""
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let body = serde_json::json!({});
    let result: Result<EchoResponse, Error> = client.post("/topapi/fail", "token-123", &body).await;

    match result {
        Err(Error::ApiError { code, message }) => {
            assert_eq!(code, 4001);
            assert_eq!(message, "invalid request");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

#[tokio::test]
async fn request_retries_server_errors_then_returns_retry_exhausted() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/topapi/retry-fail"))
        .and(query_param("access_token", "token-123"))
        .respond_with(ResponseTemplate::new(500).set_body_string("temporary error"))
        .expect(3)
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .max_retries(2)
        .retry_initial_backoff(Duration::from_millis(1))
        .retry_max_backoff(Duration::from_millis(1))
        .build()
        .expect("client should build");

    let body = serde_json::json!({});
    let result: Result<EchoResponse, Error> =
        client.post("/topapi/retry-fail", "token-123", &body).await;

    match result {
        Err(Error::RetryExhausted { attempts, .. }) => assert_eq!(attempts, 3),
        other => panic!("expected RetryExhausted error, got {other:?}"),
    }
}

#[tokio::test]
async fn request_returns_rate_limited_error_on_http_429() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/topapi/rate-limited"))
        .and(query_param("access_token", "token-123"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "1")
                .set_body_string("too many requests"),
        )
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build");

    let body = serde_json::json!({});
    let result: Result<EchoResponse, Error> = client
        .post("/topapi/rate-limited", "token-123", &body)
        .await;

    match result {
        Err(Error::RateLimited {
            retry_after: Some(retry_after),
            ..
        }) => assert_eq!(retry_after, Duration::from_secs(1)),
        other => panic!("expected RateLimited error, got {other:?}"),
    }
}

#[tokio::test]
async fn rate_limit_per_second_throttles_consecutive_requests() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/topapi/echo-rate"))
        .and(query_param("access_token", "token-123"))
        .and(body_json(serde_json::json!({ "name": "hello" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "errcode": 0,
            "errmsg": "ok",
            "echoed": "hello"
        })))
        .mount(&server)
        .await;

    let client = DingTalkClient::builder()
        .base_url(server.uri())
        .rate_limit_per_second(5)
        .build()
        .expect("client should build");

    let body = EchoRequest {
        name: "hello".to_string(),
    };

    let start = Instant::now();
    let _: EchoResponse = client
        .post("/topapi/echo-rate", "token-123", &body)
        .await
        .expect("first request should succeed");
    let _: EchoResponse = client
        .post("/topapi/echo-rate", "token-123", &body)
        .await
        .expect("second request should succeed");

    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_millis(150),
        "expected throttling delay, got {:?}",
        elapsed
    );
}
