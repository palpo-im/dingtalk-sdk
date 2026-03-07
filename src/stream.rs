//! DingTalk Stream Mode support.

use crate::error::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{MissedTickBehavior, interval, sleep};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

pub const DEFAULT_OPENAPI_HOST: &str = "https://api.dingtalk.com";
pub const GET_CONNECTION_ENDPOINT_PATH: &str = "/v1.0/gateway/connections/open";
pub const DEFAULT_USER_AGENT: &str = "dingtalk-sdk-rust/0.1.0";

pub const SUBSCRIPTION_TYPE_SYSTEM: &str = "SYSTEM";
pub const SUBSCRIPTION_TYPE_EVENT: &str = "EVENT";
pub const SUBSCRIPTION_TYPE_CALLBACK: &str = "CALLBACK";

pub const TOPIC_SYSTEM_PING: &str = "ping";
pub const TOPIC_SYSTEM_DISCONNECT: &str = "disconnect";
pub const TOPIC_BOT_MESSAGE_CALLBACK: &str = "/v1.0/im/bot/messages/get";
pub const TOPIC_PLUGIN_MESSAGE_CALLBACK: &str = "/v1.0/graph/api/invoke";
pub const TOPIC_CARD_INSTANCE_CALLBACK: &str = "/v1.0/card/instances/callback";

pub const HEADER_TOPIC: &str = "topic";
pub const HEADER_CONTENT_TYPE: &str = "contentType";
pub const HEADER_MESSAGE_ID: &str = "messageId";
pub const HEADER_TIME: &str = "time";

pub const CONTENT_TYPE_JSON: &str = "application/json";

pub const STATUS_OK: i32 = 200;
pub const STATUS_HANDLER_NOT_FOUND: i32 = 404;
pub const STATUS_INTERNAL_ERROR: i32 = 500;

pub const EVENT_HEADER_ID: &str = "eventId";
pub const EVENT_HEADER_BORN_TIME: &str = "eventBornTime";
pub const EVENT_HEADER_CORP_ID: &str = "eventCorpId";
pub const EVENT_HEADER_TYPE: &str = "eventType";
pub const EVENT_HEADER_UNIFIED_APP_ID: &str = "eventUnifiedAppId";

type DynHandlerFuture = Pin<Box<dyn Future<Output = Result<Option<DataFrameResponse>>> + Send>>;
pub type FrameHandler = Arc<dyn Fn(DataFrame) -> DynHandlerFuture + Send + Sync>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionModel {
    #[serde(rename = "type")]
    pub subscription_type: String,
    pub topic: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEndpointRequest {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    pub subscriptions: Vec<SubscriptionModel>,
    #[serde(rename = "ua")]
    pub user_agent: String,
    #[serde(rename = "localIp", skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub extras: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionEndpointResponse {
    pub endpoint: String,
    pub ticket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrame {
    #[serde(rename = "specVersion")]
    pub spec_version: String,
    #[serde(rename = "type")]
    pub frame_type: String,
    #[serde(default, alias = "timestamp")]
    pub time: i64,
    pub headers: HashMap<String, String>,
    pub data: String,
}

impl DataFrame {
    pub fn topic(&self) -> Option<&str> {
        self.headers.get(HEADER_TOPIC).map(String::as_str)
    }

    pub fn message_id(&self) -> Option<&str> {
        self.headers.get(HEADER_MESSAGE_ID).map(String::as_str)
    }

    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFrameResponse {
    pub code: i32,
    pub headers: HashMap<String, String>,
    pub message: String,
    pub data: String,
}

impl DataFrameResponse {
    pub fn new(code: i32) -> Self {
        Self {
            code,
            headers: HashMap::new(),
            message: String::new(),
            data: String::new(),
        }
    }

    pub fn success() -> Self {
        Self::new(STATUS_OK)
    }

    pub fn handler_not_found() -> Self {
        Self::new(STATUS_HANDLER_NOT_FOUND)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        let mut response = Self::new(STATUS_INTERNAL_ERROR);
        response.message = message.into();
        response
    }

    pub fn with_json(mut self, value: &impl Serialize) -> Result<Self> {
        self.data = serde_json::to_string(value)?;
        Ok(self)
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }

    pub fn ack_pong(message_id: impl Into<String>, data: impl Into<String>) -> Self {
        let mut response = Self::success();
        response.message = "ok".to_string();
        response.headers.insert(
            HEADER_CONTENT_TYPE.to_string(),
            CONTENT_TYPE_JSON.to_string(),
        );
        response
            .headers
            .insert(HEADER_MESSAGE_ID.to_string(), message_id.into());
        response.data = data.into();
        response
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventBornTime")]
    pub event_born_time: i64,
    #[serde(rename = "eventCorpId")]
    pub event_corp_id: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "eventUnifiedAppId")]
    pub event_unified_app_id: String,
}

impl EventHeader {
    pub fn from_frame(frame: &DataFrame) -> Self {
        Self {
            event_id: frame
                .header(EVENT_HEADER_ID)
                .unwrap_or_default()
                .to_string(),
            event_born_time: frame
                .header(EVENT_HEADER_BORN_TIME)
                .and_then(|ts| ts.parse::<i64>().ok())
                .unwrap_or_default(),
            event_corp_id: frame
                .header(EVENT_HEADER_CORP_ID)
                .unwrap_or_default()
                .to_string(),
            event_type: frame
                .header(EVENT_HEADER_TYPE)
                .unwrap_or_default()
                .to_string(),
            event_unified_app_id: frame
                .header(EVENT_HEADER_UNIFIED_APP_ID)
                .unwrap_or_default()
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventProcessStatus {
    Success,
    Later,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProcessResult {
    pub status: EventProcessStatus,
    pub message: String,
}

impl EventProcessResult {
    pub fn success() -> Self {
        Self {
            status: EventProcessStatus::Success,
            message: "success".to_string(),
        }
    }

    pub fn later() -> Self {
        Self {
            status: EventProcessStatus::Later,
            message: "later".to_string(),
        }
    }
}

pub fn event_success_response() -> Result<DataFrameResponse> {
    DataFrameResponse::success().with_json(&EventProcessResult::success())
}

pub fn event_later_response() -> Result<DataFrameResponse> {
    DataFrameResponse::success().with_json(&EventProcessResult::later())
}

#[derive(Debug, Clone)]
pub struct StreamClientConfig {
    pub openapi_host: String,
    pub user_agent: String,
    pub keep_alive_idle: Duration,
    pub auto_reconnect: bool,
    pub reconnect_interval: Duration,
    pub extras: HashMap<String, String>,
    pub local_ip: Option<String>,
}

impl Default for StreamClientConfig {
    fn default() -> Self {
        Self {
            openapi_host: DEFAULT_OPENAPI_HOST.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            keep_alive_idle: Duration::from_secs(120),
            auto_reconnect: true,
            reconnect_interval: Duration::from_secs(3),
            extras: HashMap::new(),
            local_ip: None,
        }
    }
}

pub struct DingTalkStreamClient {
    client_id: String,
    client_secret: String,
    http_client: reqwest::Client,
    config: StreamClientConfig,
    handlers: HashMap<String, HashMap<String, FrameHandler>>,
}

enum FrameDisposition {
    Continue,
    Disconnect,
}

impl DingTalkStreamClient {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Result<Self> {
        let client_id = client_id.into();
        let client_secret = client_secret.into();

        if client_id.trim().is_empty() {
            return Err(Error::missing_field("client_id"));
        }
        if client_secret.trim().is_empty() {
            return Err(Error::missing_field("client_secret"));
        }

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;

        let mut client = Self {
            client_id,
            client_secret,
            http_client,
            config: StreamClientConfig::default(),
            handlers: HashMap::new(),
        };
        client.register_default_system_handlers();
        Ok(client)
    }

    pub fn config_mut(&mut self) -> &mut StreamClientConfig {
        &mut self.config
    }

    pub fn register_handler<F, Fut>(&mut self, subscription_type: &str, topic: &str, handler: F)
    where
        F: Fn(DataFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Option<DataFrameResponse>>> + Send + 'static,
    {
        let boxed: FrameHandler = Arc::new(move |frame| Box::pin(handler(frame)));
        self.handlers
            .entry(subscription_type.to_string())
            .or_default()
            .insert(topic.to_string(), boxed);
    }

    pub fn register_callback_handler<F, Fut>(&mut self, topic: &str, handler: F)
    where
        F: Fn(DataFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Option<DataFrameResponse>>> + Send + 'static,
    {
        self.register_handler(SUBSCRIPTION_TYPE_CALLBACK, topic, handler);
    }

    pub fn register_event_handler<F, Fut>(&mut self, topic: &str, handler: F)
    where
        F: Fn(DataFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Option<DataFrameResponse>>> + Send + 'static,
    {
        self.register_handler(SUBSCRIPTION_TYPE_EVENT, topic, handler);
    }

    pub fn register_all_event_handler<F, Fut>(&mut self, handler: F)
    where
        F: Fn(DataFrame) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Option<DataFrameResponse>>> + Send + 'static,
    {
        self.register_handler(SUBSCRIPTION_TYPE_EVENT, "*", handler);
    }

    pub async fn get_connection_endpoint(&self) -> Result<ConnectionEndpointResponse> {
        self.validate()?;

        let mut subscriptions = Vec::new();
        for (subscription_type, topics) in &self.handlers {
            for topic in topics.keys() {
                subscriptions.push(SubscriptionModel {
                    subscription_type: subscription_type.clone(),
                    topic: topic.clone(),
                });
            }
        }

        let request = ConnectionEndpointRequest {
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            subscriptions,
            user_agent: self.config.user_agent.clone(),
            local_ip: self.config.local_ip.clone(),
            extras: self.config.extras.clone(),
        };

        let mut base = self.config.openapi_host.trim_end_matches('/').to_string();
        if base.is_empty() {
            base = DEFAULT_OPENAPI_HOST.to_string();
        }
        let url = format!("{base}{GET_CONNECTION_ENDPOINT_PATH}");

        let response = self
            .http_client
            .post(url)
            .header("Content-Type", CONTENT_TYPE_JSON)
            .header("Accept", CONTENT_TYPE_JSON)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::api_error(
                status.as_u16() as i64,
                if body.is_empty() {
                    "failed to get stream connection endpoint"
                } else {
                    body.as_str()
                },
            ));
        }

        let endpoint: ConnectionEndpointResponse = response.json().await?;
        if endpoint.endpoint.trim().is_empty() || endpoint.ticket.trim().is_empty() {
            return Err(Error::invalid_param(
                "stream connection endpoint response missing endpoint/ticket",
            ));
        }

        Ok(endpoint)
    }

    pub async fn start(&self) -> Result<()> {
        self.validate()?;

        loop {
            let run_result = self.start_once().await;
            if !self.config.auto_reconnect {
                return run_result;
            }

            if let Err(err) = run_result {
                eprintln!("[dingtalk-stream] stream loop exited with error: {err}");
            }
            sleep(self.config.reconnect_interval).await;
        }
    }

    pub async fn start_once(&self) -> Result<()> {
        let endpoint = self.get_connection_endpoint().await?;
        let ws_url = Self::build_ws_url(&endpoint.endpoint, &endpoint.ticket)?;
        let (mut socket, _) = connect_async(ws_url.as_str()).await?;

        let mut keep_alive = interval(self.config.keep_alive_idle.max(Duration::from_secs(3)));
        keep_alive.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = keep_alive.tick() => {
                    socket.send(Message::Ping(Vec::new().into())).await?;
                }
                incoming = socket.next() => {
                    match incoming {
                        Some(Ok(Message::Text(text))) => {
                            let frame: DataFrame = serde_json::from_str(text.as_ref())?;
                            let disposition = self.process_frame(frame, &mut socket).await?;
                            if matches!(disposition, FrameDisposition::Disconnect) {
                                return Ok(());
                            }
                        }
                        Some(Ok(Message::Binary(binary))) => {
                            if let Ok(text) = String::from_utf8(binary.to_vec()) {
                                let frame: DataFrame = serde_json::from_str(&text)?;
                                let disposition = self.process_frame(frame, &mut socket).await?;
                                if matches!(disposition, FrameDisposition::Disconnect) {
                                    return Ok(());
                                }
                            }
                        }
                        Some(Ok(Message::Ping(payload))) => {
                            socket.send(Message::Pong(payload)).await?;
                        }
                        Some(Ok(Message::Pong(_))) => {}
                        Some(Ok(Message::Frame(_))) => {}
                        Some(Ok(Message::Close(_))) => return Ok(()),
                        Some(Err(err)) => return Err(err.into()),
                        None => return Ok(()),
                    }
                }
            }
        }
    }

    fn build_ws_url(endpoint: &str, ticket: &str) -> Result<Url> {
        let mut ws_url = Url::parse(endpoint)?;
        ws_url.query_pairs_mut().append_pair("ticket", ticket);
        Ok(ws_url)
    }

    fn register_default_system_handlers(&mut self) {
        self.register_handler(
            SUBSCRIPTION_TYPE_SYSTEM,
            TOPIC_SYSTEM_PING,
            |frame| async move {
                let message_id = frame.message_id().unwrap_or_default().to_string();
                Ok(Some(DataFrameResponse::ack_pong(message_id, frame.data)))
            },
        );
        self.register_handler(
            SUBSCRIPTION_TYPE_SYSTEM,
            TOPIC_SYSTEM_DISCONNECT,
            |_frame| async { Ok(None) },
        );
    }

    fn validate(&self) -> Result<()> {
        if self.client_id.trim().is_empty() {
            return Err(Error::missing_field("client_id"));
        }
        if self.client_secret.trim().is_empty() {
            return Err(Error::missing_field("client_secret"));
        }

        for (subscription_type, topics) in &self.handlers {
            match subscription_type.as_str() {
                SUBSCRIPTION_TYPE_SYSTEM | SUBSCRIPTION_TYPE_EVENT | SUBSCRIPTION_TYPE_CALLBACK => {
                }
                other => {
                    return Err(Error::invalid_param(format!(
                        "unknown subscription type: {other}"
                    )));
                }
            }

            if topics.is_empty() {
                return Err(Error::invalid_param(format!(
                    "no topic handler for subscription type: {subscription_type}"
                )));
            }
        }

        Ok(())
    }

    fn resolve_handler(&self, frame_type: &str, topic: &str) -> Option<FrameHandler> {
        let topics = self.handlers.get(frame_type)?;
        topics
            .get(topic)
            .or_else(|| topics.get("*"))
            .map(Arc::clone)
    }

    fn is_disconnect_frame(frame: &DataFrame) -> bool {
        frame.frame_type == SUBSCRIPTION_TYPE_SYSTEM
            && frame.topic() == Some(TOPIC_SYSTEM_DISCONNECT)
    }

    async fn process_frame(
        &self,
        frame: DataFrame,
        socket: &mut WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    ) -> Result<FrameDisposition> {
        let topic = frame.topic().unwrap_or_default().to_string();
        let maybe_handler = self.resolve_handler(&frame.frame_type, &topic);

        let mut response = if let Some(handler) = maybe_handler {
            match handler(frame.clone()).await {
                Ok(Some(resp)) => resp,
                Ok(None) => {
                    if Self::is_disconnect_frame(&frame) {
                        return Ok(FrameDisposition::Disconnect);
                    }
                    DataFrameResponse::success()
                }
                Err(err) => DataFrameResponse::internal_error(err.to_string()),
            }
        } else {
            DataFrameResponse::handler_not_found()
        };

        if !response.headers.contains_key(HEADER_MESSAGE_ID) {
            if let Some(message_id) = frame.message_id() {
                response
                    .headers
                    .insert(HEADER_MESSAGE_ID.to_string(), message_id.to_string());
            }
        }
        if !response.headers.contains_key(HEADER_CONTENT_TYPE) {
            response.headers.insert(
                HEADER_CONTENT_TYPE.to_string(),
                CONTENT_TYPE_JSON.to_string(),
            );
        }

        socket
            .send(Message::Text(serde_json::to_string(&response)?.into()))
            .await?;
        Ok(FrameDisposition::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_event_header_from_data_frame() {
        let mut headers = HashMap::new();
        headers.insert(EVENT_HEADER_ID.to_string(), "evt_1".to_string());
        headers.insert(
            EVENT_HEADER_BORN_TIME.to_string(),
            "1700000000000".to_string(),
        );
        headers.insert(EVENT_HEADER_CORP_ID.to_string(), "dingcorp".to_string());
        headers.insert(EVENT_HEADER_TYPE.to_string(), "user_add_org".to_string());
        headers.insert(
            EVENT_HEADER_UNIFIED_APP_ID.to_string(),
            "unified_app".to_string(),
        );

        let frame = DataFrame {
            spec_version: "1.0".to_string(),
            frame_type: SUBSCRIPTION_TYPE_EVENT.to_string(),
            time: 1700000000001,
            headers,
            data: "{}".to_string(),
        };

        let header = EventHeader::from_frame(&frame);
        assert_eq!(header.event_id, "evt_1");
        assert_eq!(header.event_born_time, 1700000000000);
        assert_eq!(header.event_corp_id, "dingcorp");
        assert_eq!(header.event_type, "user_add_org");
        assert_eq!(header.event_unified_app_id, "unified_app");
    }

    #[test]
    fn build_ws_url_with_ticket() {
        let url = DingTalkStreamClient::build_ws_url(
            "wss://open-connection.dingtalk.com/connect",
            "ticket_123",
        )
        .expect("url should be valid");

        assert_eq!(
            url.as_str(),
            "wss://open-connection.dingtalk.com/connect?ticket=ticket_123"
        );
    }

    #[test]
    fn parse_data_frame_without_time() {
        let frame = serde_json::json!({
            "specVersion": "1.0",
            "type": "CALLBACK",
            "headers": {
                "topic": TOPIC_BOT_MESSAGE_CALLBACK,
                "messageId": "msg_1"
            },
            "data": "{}"
        });

        let parsed: DataFrame = serde_json::from_value(frame).expect("frame should parse");
        assert_eq!(parsed.time, 0);
    }

    #[test]
    fn parse_data_frame_with_timestamp_alias() {
        let frame = serde_json::json!({
            "specVersion": "1.0",
            "type": "CALLBACK",
            "timestamp": 1700000000001_i64,
            "headers": {
                "topic": TOPIC_BOT_MESSAGE_CALLBACK,
                "messageId": "msg_2"
            },
            "data": "{}"
        });

        let parsed: DataFrame = serde_json::from_value(frame).expect("frame should parse");
        assert_eq!(parsed.time, 1700000000001_i64);
    }
}
