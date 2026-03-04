//! DingTalk API bindings for the client module.

use crate::error::{Error, Result};
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};

const DEFAULT_BASE_URL: &str = "https://oapi.dingtalk.com";
const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_RETRY_INITIAL_BACKOFF_MS: u64 = 200;
const DEFAULT_RETRY_MAX_BACKOFF_MS: u64 = 2_000;

#[derive(Debug, Clone)]
struct RetryConfig {
    max_retries: usize,
    initial_backoff: Duration,
    max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 0,
            initial_backoff: Duration::from_millis(DEFAULT_RETRY_INITIAL_BACKOFF_MS),
            max_backoff: Duration::from_millis(DEFAULT_RETRY_MAX_BACKOFF_MS),
        }
    }
}

#[derive(Debug)]
struct RateLimiterState {
    interval: Duration,
    next_allowed: Instant,
}

#[derive(Debug, Clone)]
/// Payload model used by this API.
pub struct DingTalkClient {
    client: Client,
    base_url: String,
    app_key: Option<String>,
    app_secret: Option<String>,
    retry_config: RetryConfig,
    rate_limiter: Option<Arc<Mutex<RateLimiterState>>>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct ApiResponse<T> {
    pub errcode: i64,
    pub errmsg: String,
    #[serde(flatten)]
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct EmptyResponse {}

impl DingTalkClient {
    /// Executes this helper method.
    pub fn new() -> Result<Self> {
        Self::builder().build()
    }

    /// Executes this helper method.
    pub fn builder() -> DingTalkClientBuilder {
        DingTalkClientBuilder::default()
    }

    /// Executes this helper method.
    pub fn set_credentials(&mut self, app_key: String, app_secret: String) {
        self.app_key = Some(app_key);
        self.app_secret = Some(app_secret);
    }

    async fn wait_for_rate_limit(&self) {
        let Some(rate_limiter) = &self.rate_limiter else {
            return;
        };

        let mut state = rate_limiter.lock().await;
        let now = Instant::now();
        if now < state.next_allowed {
            sleep(state.next_allowed - now).await;
        }
        state.next_allowed = Instant::now() + state.interval;
    }

    fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<std::time::Duration> {
        let value = headers.get(reqwest::header::RETRY_AFTER)?;
        let value = value.to_str().ok()?;
        let seconds = value.parse::<u64>().ok()?;
        Some(Duration::from_secs(seconds))
    }

    fn retry_backoff(&self, attempt: usize, retry_after: Option<Duration>) -> Duration {
        if let Some(retry_after) = retry_after {
            return retry_after.min(self.retry_config.max_backoff);
        }

        let pow = (attempt.saturating_sub(1)).min(16) as u32;
        let backoff = self
            .retry_config
            .initial_backoff
            .saturating_mul(2u32.pow(pow));
        backoff.min(self.retry_config.max_backoff)
    }

    fn should_retry_http_error(error: &reqwest::Error) -> bool {
        error.is_timeout() || error.is_connect() || error.is_request()
    }

    async fn send_with_retry<F>(&self, mut build_request: F) -> Result<reqwest::Response>
    where
        F: FnMut() -> RequestBuilder,
    {
        let max_attempts = self.retry_config.max_retries.saturating_add(1);
        let mut attempt = 0usize;

        loop {
            attempt += 1;
            self.wait_for_rate_limit().await;

            match build_request().send().await {
                Ok(response) => {
                    let status = response.status();

                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        let retry_after = Self::parse_retry_after(response.headers());
                        if attempt < max_attempts {
                            sleep(self.retry_backoff(attempt, retry_after)).await;
                            continue;
                        }
                        return Err(Error::rate_limited(
                            "DingTalk API returned HTTP 429",
                            retry_after,
                        ));
                    }

                    if status.is_server_error() {
                        if attempt < max_attempts {
                            sleep(self.retry_backoff(attempt, None)).await;
                            continue;
                        }
                        return Err(Error::retry_exhausted(
                            attempt,
                            format!("last response status was {status}"),
                        ));
                    }

                    return Ok(response);
                }
                Err(error) => {
                    if Self::should_retry_http_error(&error) {
                        if attempt < max_attempts {
                            sleep(self.retry_backoff(attempt, None)).await;
                            continue;
                        }
                        return Err(Error::retry_exhausted(attempt, error.to_string()));
                    }
                    return Err(Error::HttpError(error));
                }
            }
        }
    }

    /// Executes this API call.
    pub async fn get_access_token(&self) -> Result<String> {
        let app_key = self
            .app_key
            .as_ref()
            .ok_or_else(|| Error::auth_error("App key not configured"))?;
        let app_secret = self
            .app_secret
            .as_ref()
            .ok_or_else(|| Error::auth_error("App secret not configured"))?;

        let url = format!("{}/gettoken", self.base_url);
        let response: ApiResponse<TokenResponse> = self
            .send_with_retry(|| {
                self.client
                    .get(&url)
                    .query(&[("appkey", app_key), ("appsecret", app_secret)])
            })
            .await?
            .json()
            .await?;

        if response.errcode != 0 {
            return Err(Error::api_error(response.errcode, response.errmsg));
        }

        Ok(response.data.access_token)
    }

    /// Executes an HTTP request and decodes DingTalk's standard response envelope.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<&str, &str>>,
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        if !matches!(
            method,
            Method::GET | Method::POST | Method::PUT | Method::DELETE
        ) {
            return Err(Error::invalid_param("Unsupported HTTP method"));
        }

        let url = format!("{}{}", self.base_url, path);
        let body = body.map(serde_json::to_value).transpose()?;

        let response = self
            .send_with_retry(|| {
                let mut request: RequestBuilder = if method == Method::GET {
                    self.client.get(&url)
                } else if method == Method::POST {
                    self.client.post(&url)
                } else if method == Method::PUT {
                    self.client.put(&url)
                } else {
                    self.client.delete(&url)
                };

                request = request.query(&[("access_token", access_token)]);

                if let Some(params) = query {
                    request = request.query(params);
                }

                if let Some(data) = &body {
                    request = request.json(data);
                }

                request
            })
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().await.unwrap_or_default();
            let message = if body_text.trim().is_empty() {
                "Request failed".to_string()
            } else {
                body_text
            };
            return Err(Error::api_error(status.as_u16() as i64, message));
        }

        let response: ApiResponse<T> = response.json().await?;

        if response.errcode != 0 {
            return Err(Error::api_error(response.errcode, response.errmsg));
        }

        Ok(response.data)
    }

    /// Executes an OpenAPI v1.0 request where the token is sent in request headers.
    pub async fn request_openapi<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<String, String>>,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        if !matches!(
            method,
            Method::GET | Method::POST | Method::PUT | Method::DELETE
        ) {
            return Err(Error::invalid_param("Unsupported HTTP method"));
        }

        let url = format!("{}{}", self.base_url, path);
        let body = body.cloned();
        let response = self
            .send_with_retry(|| {
                let mut request: RequestBuilder = if method == Method::GET {
                    self.client.get(&url)
                } else if method == Method::POST {
                    self.client.post(&url)
                } else if method == Method::PUT {
                    self.client.put(&url)
                } else {
                    self.client.delete(&url)
                };

                request = request.header("x-acs-dingtalk-access-token", access_token);

                if let Some(params) = query {
                    request = request.query(params);
                }

                if let Some(data) = &body {
                    request = request.json(data);
                }

                request
            })
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().await.unwrap_or_default();
            let message = if body_text.trim().is_empty() {
                "Request failed".to_string()
            } else {
                body_text
            };
            return Err(Error::api_error(status.as_u16() as i64, message));
        }

        Ok(response.json().await?)
    }

    /// Executes an OpenAPI v1.0 request that does not return a response body.
    pub async fn request_openapi_no_content(
        &self,
        method: Method,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<String, String>>,
        body: Option<&serde_json::Value>,
    ) -> Result<()> {
        if !matches!(
            method,
            Method::GET | Method::POST | Method::PUT | Method::DELETE
        ) {
            return Err(Error::invalid_param("Unsupported HTTP method"));
        }

        let url = format!("{}{}", self.base_url, path);
        let body = body.cloned();
        let response = self
            .send_with_retry(|| {
                let mut request: RequestBuilder = if method == Method::GET {
                    self.client.get(&url)
                } else if method == Method::POST {
                    self.client.post(&url)
                } else if method == Method::PUT {
                    self.client.put(&url)
                } else {
                    self.client.delete(&url)
                };

                request = request.header("x-acs-dingtalk-access-token", access_token);

                if let Some(params) = query {
                    request = request.query(params);
                }

                if let Some(data) = &body {
                    request = request.json(data);
                }

                request
            })
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().await.unwrap_or_default();
            let message = if body_text.trim().is_empty() {
                "Request failed".to_string()
            } else {
                body_text
            };
            return Err(Error::api_error(status.as_u16() as i64, message));
        }

        Ok(())
    }

    /// Executes an OpenAPI v1.0 GET request.
    pub async fn get_openapi<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<T> {
        self.request_openapi(Method::GET, path, access_token, query, None)
            .await
    }

    /// Executes an OpenAPI v1.0 POST request.
    pub async fn post_openapi<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        self.request_openapi(Method::POST, path, access_token, None, body)
            .await
    }

    /// Executes an OpenAPI v1.0 PUT request.
    pub async fn put_openapi<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        self.request_openapi(Method::PUT, path, access_token, None, body)
            .await
    }

    /// Executes an OpenAPI v1.0 DELETE request.
    pub async fn delete_openapi<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<T> {
        self.request_openapi(Method::DELETE, path, access_token, query, None)
            .await
    }

    /// Executes an OpenAPI v1.0 DELETE request that returns no response body.
    pub async fn delete_openapi_no_content(
        &self,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<()> {
        self.request_openapi_no_content(Method::DELETE, path, access_token, query, None)
            .await
    }

    /// Executes a GET request against a DingTalk endpoint.
    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        access_token: &str,
        query: Option<&HashMap<&str, &str>>,
    ) -> Result<T> {
        self.request(
            Method::GET,
            path,
            access_token,
            query,
            None::<&EmptyResponse>,
        )
        .await
    }

    /// Executes a POST request against a DingTalk endpoint.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        access_token: &str,
        body: &B,
    ) -> Result<T> {
        self.request(Method::POST, path, access_token, None, Some(body))
            .await
    }

    /// Executes a raw POST request to an absolute URL.
    pub async fn post_raw<B: Serialize>(&self, url: &str, body: &B) -> Result<()> {
        let body = serde_json::to_value(body)?;
        let response = self
            .send_with_retry(|| self.client.post(url).json(&body))
            .await?;
        let status = response.status();
        if !status.is_success() {
            let body_text = response.text().await.unwrap_or_default();
            let message = if body_text.trim().is_empty() {
                "Request failed".to_string()
            } else {
                body_text
            };
            return Err(Error::api_error(status.as_u16() as i64, message));
        }
        Ok(())
    }
}

impl Default for DingTalkClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default DingTalk client")
    }
}

#[derive(Debug, Default)]
/// Payload model used by this API.
pub struct DingTalkClientBuilder {
    base_url: Option<String>,
    timeout: Option<Duration>,
    app_key: Option<String>,
    app_secret: Option<String>,
    max_retries: Option<usize>,
    retry_initial_backoff: Option<Duration>,
    retry_max_backoff: Option<Duration>,
    rate_limit_per_second: Option<u32>,
}

impl DingTalkClientBuilder {
    /// Executes this helper method.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Executes this helper method.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Executes this helper method.
    pub fn app_key(mut self, key: impl Into<String>) -> Self {
        self.app_key = Some(key.into());
        self
    }

    /// Executes this helper method.
    pub fn app_secret(mut self, secret: impl Into<String>) -> Self {
        self.app_secret = Some(secret.into());
        self
    }

    /// Executes this helper method.
    pub fn credentials(mut self, key: impl Into<String>, secret: impl Into<String>) -> Self {
        self.app_key = Some(key.into());
        self.app_secret = Some(secret.into());
        self
    }

    /// Executes this helper method.
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Executes this helper method.
    pub fn retry_initial_backoff(mut self, backoff: Duration) -> Self {
        self.retry_initial_backoff = Some(backoff);
        self
    }

    /// Executes this helper method.
    pub fn retry_max_backoff(mut self, backoff: Duration) -> Self {
        self.retry_max_backoff = Some(backoff);
        self
    }

    /// Executes this helper method.
    pub fn rate_limit_per_second(mut self, requests_per_second: u32) -> Self {
        self.rate_limit_per_second = Some(requests_per_second);
        self
    }

    /// Executes this helper method.
    pub fn build(self) -> Result<DingTalkClient> {
        let client = Client::builder()
            .timeout(
                self.timeout
                    .unwrap_or_else(|| Duration::from_secs(DEFAULT_TIMEOUT)),
            )
            .user_agent("dingtalk-sdk-rust/0.1.0")
            .build()?;

        if matches!(self.rate_limit_per_second, Some(0)) {
            return Err(Error::invalid_param(
                "rate_limit_per_second must be greater than zero",
            ));
        }

        let mut retry_config = RetryConfig::default();
        if let Some(value) = self.max_retries {
            retry_config.max_retries = value;
        }
        if let Some(value) = self.retry_initial_backoff {
            retry_config.initial_backoff = value;
        }
        if let Some(value) = self.retry_max_backoff {
            retry_config.max_backoff = value;
        }
        if retry_config.max_backoff < retry_config.initial_backoff {
            retry_config.max_backoff = retry_config.initial_backoff;
        }

        let rate_limiter = self.rate_limit_per_second.map(|requests_per_second| {
            let interval = Duration::from_secs_f64(1.0 / requests_per_second as f64);
            Arc::new(Mutex::new(RateLimiterState {
                interval,
                next_allowed: Instant::now(),
            }))
        });

        Ok(DingTalkClient {
            client,
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            app_key: self.app_key,
            app_secret: self.app_secret,
            retry_config,
            rate_limiter,
        })
    }
}
