use dingtalk_sdk::DingTalkClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DingTalkClient::new()?;
    client.set_credentials("your-app-key".to_string(), "your-app-secret".to_string());

    let token = client.get_access_token().await?;
    println!("access token: {token}");

    Ok(())
}
