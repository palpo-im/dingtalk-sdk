use dingtalk_sdk::DingTalkClient;
use dingtalk_sdk::models::{Message, TextMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_token = std::env::var("DINGTALK_ACCESS_TOKEN")
        .expect("set DINGTALK_ACCESS_TOKEN before running this example");
    let agent_id: i64 = std::env::var("DINGTALK_AGENT_ID")
        .expect("set DINGTALK_AGENT_ID before running this example")
        .parse()?;
    let user_id = std::env::var("DINGTALK_USER_ID")
        .expect("set DINGTALK_USER_ID before running this example");

    let client = DingTalkClient::new()?;
    let message = Message {
        msg_type: "text".to_string(),
        text: Some(TextMessage {
            content: "Hello from dingtalk-sdk example".to_string(),
        }),
        image: None,
        voice: None,
        file: None,
        link: None,
        oa: None,
        markdown: None,
        action_card: None,
    };

    let response = client
        .send_message(&access_token, agent_id, &[user_id.as_str()], &message)
        .await?;

    println!("message task id: {}", response.task_id);
    Ok(())
}
