use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::WeChatClient;
use crate::error::Result;

/// Customer service message types.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "msgtype", rename_all = "lowercase")]
pub enum CustomerServiceMessage {
    Text {
        touser: String,
        text: TextContent,
    },
    Image {
        touser: String,
        image: MediaContent,
    },
    Voice {
        touser: String,
        voice: MediaContent,
    },
    Video {
        touser: String,
        video: VideoContent,
    },
    Music {
        touser: String,
        music: MusicContent,
    },
    News {
        touser: String,
        news: NewsContent,
    },
    #[serde(rename = "mpnews")]
    MpNews {
        touser: String,
        mpnews: MediaContent,
    },
    #[serde(rename = "msgmenu")]
    MsgMenu {
        touser: String,
        msgmenu: MsgMenuContent,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct TextContent {
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MediaContent {
    pub media_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoContent {
    pub media_id: String,
    pub thumb_media_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MusicContent {
    pub title: String,
    pub description: String,
    pub musicurl: String,
    pub hqmusicurl: String,
    pub thumb_media_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewsContent {
    pub articles: Vec<NewsArticle>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewsArticle {
    pub title: String,
    pub description: String,
    pub url: String,
    pub picurl: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MsgMenuContent {
    pub head_content: String,
    pub list: Vec<MsgMenuItem>,
    pub tail_content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MsgMenuItem {
    pub id: String,
    pub content: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct SendResponse {
    errcode: i64,
    errmsg: String,
}

impl WeChatClient {
    /// Send a customer service message.
    pub async fn send_customer_service_message(
        &self,
        message: &CustomerServiceMessage,
    ) -> Result<()> {
        let _: SendResponse = self.post_json("/message/custom/send", message).await?;
        Ok(())
    }

    /// Send a text message via customer service.
    pub async fn send_text(&self, to_user: &str, content: &str) -> Result<()> {
        let message = CustomerServiceMessage::Text {
            touser: to_user.to_string(),
            text: TextContent {
                content: content.to_string(),
            },
        };
        self.send_customer_service_message(&message).await
    }

    /// Send an image via customer service.
    pub async fn send_image(&self, to_user: &str, media_id: &str) -> Result<()> {
        let message = CustomerServiceMessage::Image {
            touser: to_user.to_string(),
            image: MediaContent {
                media_id: media_id.to_string(),
            },
        };
        self.send_customer_service_message(&message).await
    }

    /// Send a voice message via customer service.
    pub async fn send_voice(&self, to_user: &str, media_id: &str) -> Result<()> {
        let message = CustomerServiceMessage::Voice {
            touser: to_user.to_string(),
            voice: MediaContent {
                media_id: media_id.to_string(),
            },
        };
        self.send_customer_service_message(&message).await
    }

    /// Send a video via customer service.
    pub async fn send_video(
        &self,
        to_user: &str,
        media_id: &str,
        thumb_media_id: &str,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<()> {
        let message = CustomerServiceMessage::Video {
            touser: to_user.to_string(),
            video: VideoContent {
                media_id: media_id.to_string(),
                thumb_media_id: thumb_media_id.to_string(),
                title: title.map(|s| s.to_string()),
                description: description.map(|s| s.to_string()),
            },
        };
        self.send_customer_service_message(&message).await
    }

    /// Send news articles via customer service.
    pub async fn send_news(&self, to_user: &str, articles: Vec<NewsArticle>) -> Result<()> {
        let message = CustomerServiceMessage::News {
            touser: to_user.to_string(),
            news: NewsContent { articles },
        };
        self.send_customer_service_message(&message).await
    }

    /// Set typing status for a user.
    pub async fn set_typing(&self, to_user: &str, typing: bool) -> Result<()> {
        let command = if typing { "Typing" } else { "CancelTyping" };
        let body = json!({
            "touser": to_user,
            "command": command
        });
        let _: SendResponse = self.post_json("/message/custom/typing", &body).await?;
        Ok(())
    }
}
