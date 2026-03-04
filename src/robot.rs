//! DingTalk API bindings for the robot module.

use crate::client::DingTalkClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub use crate::models::*;

impl DingTalkClient {
    /// Executes this API call.
    pub async fn send_robot_text(&self, webhook: &str, content: &str) -> Result<()> {
        let body = serde_json::json!({
            "msgtype": "text",
            "text": {
                "content": content
            }
        });
        self.post_raw(webhook, &body).await
    }

    /// Executes this API call.
    pub async fn send_robot_markdown(&self, webhook: &str, title: &str, text: &str) -> Result<()> {
        let body = serde_json::json!({
            "msgtype": "markdown",
            "markdown": {
                "title": title,
                "text": text
            }
        });
        self.post_raw(webhook, &body).await
    }

    /// Executes this API call.
    pub async fn send_robot_link(
        &self,
        webhook: &str,
        title: &str,
        text: &str,
        message_url: &str,
        pic_url: Option<&str>,
    ) -> Result<()> {
        let mut link = serde_json::Map::new();
        link.insert("title".to_string(), title.into());
        link.insert("text".to_string(), text.into());
        link.insert("messageUrl".to_string(), message_url.into());
        if let Some(pic) = pic_url {
            link.insert("picUrl".to_string(), pic.into());
        }

        let body = serde_json::json!({
            "msgtype": "link",
            "link": link
        });
        self.post_raw(webhook, &body).await
    }

    /// Executes this API call.
    pub async fn send_robot_action_card(
        &self,
        webhook: &str,
        action_card: &RobotActionCard,
    ) -> Result<()> {
        let body = serde_json::json!({
            "msgtype": "actionCard",
            "actionCard": action_card
        });
        self.post_raw(webhook, &body).await
    }

    /// Executes this API call.
    pub async fn send_robot_feed_card(
        &self,
        webhook: &str,
        links: &[RobotFeedCardLink],
    ) -> Result<()> {
        let body = serde_json::json!({
            "msgtype": "feedCard",
            "feedCard": {
                "links": links
            }
        });
        self.post_raw(webhook, &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct RobotActionCard {
    pub title: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_json_list: Option<Vec<RobotButton>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct RobotButton {
    pub title: String,
    pub action_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload model used by this API.
pub struct RobotFeedCardLink {
    pub title: String,
    pub message_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<String>,
}
