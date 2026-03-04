use dingtalk_sdk::stream::{
    DataFrameResponse, DingTalkStreamClient, EVENT_HEADER_TYPE, TOPIC_BOT_MESSAGE_CALLBACK,
    event_success_response,
};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = std::env::var("DINGTALK_CLIENT_ID")?;
    let client_secret = std::env::var("DINGTALK_CLIENT_SECRET")?;

    let mut client = DingTalkStreamClient::new(client_id, client_secret)?;

    client.register_callback_handler(TOPIC_BOT_MESSAGE_CALLBACK, |frame| async move {
        let payload: Value = serde_json::from_str(&frame.data)?;
        println!("bot callback payload: {payload}");
        Ok(Some(DataFrameResponse::success()))
    });

    client.register_all_event_handler(|frame| async move {
        let payload: Value = serde_json::from_str(&frame.data)?;
        let event_type = frame.header(EVENT_HEADER_TYPE).unwrap_or("unknown");
        println!("event type: {event_type}, payload: {payload}");
        Ok(Some(event_success_response()?))
    });

    client.start().await?;
    Ok(())
}
