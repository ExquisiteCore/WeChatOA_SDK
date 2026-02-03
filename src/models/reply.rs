use serde::Serialize;

use crate::client::WeChatClient;
use crate::error::Result;

/// Base fields for all reply messages.
#[derive(Debug, Clone)]
pub struct ReplyBase {
    /// Receiver's OpenID
    pub to_user_name: String,
    /// Developer's WeChat account
    pub from_user_name: String,
    /// Reply timestamp
    pub create_time: i64,
}

/// Text reply message.
#[derive(Debug, Clone)]
pub struct TextReply {
    pub base: ReplyBase,
    pub content: String,
}

impl TextReply {
    pub fn new(
        to_user: impl Into<String>,
        from_user: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            base: ReplyBase {
                to_user_name: to_user.into(),
                from_user_name: from_user.into(),
                create_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            },
            content: content.into(),
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[{}]]></Content>
</xml>"#,
            self.base.to_user_name, self.base.from_user_name, self.base.create_time, self.content
        )
    }

    /// Generate an encrypted XML reply for safe mode.
    ///
    /// Uses auto-generated timestamp and nonce.
    pub fn to_encrypted_xml(&self, client: &WeChatClient) -> Result<String> {
        client.encrypt_reply_auto(&self.to_xml())
    }

    /// Generate an encrypted XML reply with custom timestamp and nonce.
    pub fn to_encrypted_xml_with(
        &self,
        client: &WeChatClient,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        client.encrypt_reply(&self.to_xml(), timestamp, nonce)
    }
}

/// Image reply message.
#[derive(Debug, Clone)]
pub struct ImageReply {
    pub base: ReplyBase,
    pub media_id: String,
}

impl ImageReply {
    pub fn new(
        to_user: impl Into<String>,
        from_user: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Self {
        Self {
            base: ReplyBase {
                to_user_name: to_user.into(),
                from_user_name: from_user.into(),
                create_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            },
            media_id: media_id.into(),
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[image]]></MsgType>
<Image>
<MediaId><![CDATA[{}]]></MediaId>
</Image>
</xml>"#,
            self.base.to_user_name, self.base.from_user_name, self.base.create_time, self.media_id
        )
    }

    /// Generate an encrypted XML reply for safe mode.
    pub fn to_encrypted_xml(&self, client: &WeChatClient) -> Result<String> {
        client.encrypt_reply_auto(&self.to_xml())
    }

    /// Generate an encrypted XML reply with custom timestamp and nonce.
    pub fn to_encrypted_xml_with(
        &self,
        client: &WeChatClient,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        client.encrypt_reply(&self.to_xml(), timestamp, nonce)
    }
}

/// Voice reply message.
#[derive(Debug, Clone)]
pub struct VoiceReply {
    pub base: ReplyBase,
    pub media_id: String,
}

impl VoiceReply {
    pub fn new(
        to_user: impl Into<String>,
        from_user: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Self {
        Self {
            base: ReplyBase {
                to_user_name: to_user.into(),
                from_user_name: from_user.into(),
                create_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            },
            media_id: media_id.into(),
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[voice]]></MsgType>
<Voice>
<MediaId><![CDATA[{}]]></MediaId>
</Voice>
</xml>"#,
            self.base.to_user_name, self.base.from_user_name, self.base.create_time, self.media_id
        )
    }

    /// Generate an encrypted XML reply for safe mode.
    pub fn to_encrypted_xml(&self, client: &WeChatClient) -> Result<String> {
        client.encrypt_reply_auto(&self.to_xml())
    }

    /// Generate an encrypted XML reply with custom timestamp and nonce.
    pub fn to_encrypted_xml_with(
        &self,
        client: &WeChatClient,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        client.encrypt_reply(&self.to_xml(), timestamp, nonce)
    }
}

/// Video reply message.
#[derive(Debug, Clone)]
pub struct VideoReply {
    pub base: ReplyBase,
    pub media_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

impl VideoReply {
    pub fn new(
        to_user: impl Into<String>,
        from_user: impl Into<String>,
        media_id: impl Into<String>,
    ) -> Self {
        Self {
            base: ReplyBase {
                to_user_name: to_user.into(),
                from_user_name: from_user.into(),
                create_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            },
            media_id: media_id.into(),
            title: None,
            description: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn to_xml(&self) -> String {
        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[video]]></MsgType>
<Video>
<MediaId><![CDATA[{}]]></MediaId>
<Title><![CDATA[{}]]></Title>
<Description><![CDATA[{}]]></Description>
</Video>
</xml>"#,
            self.base.to_user_name,
            self.base.from_user_name,
            self.base.create_time,
            self.media_id,
            self.title.as_deref().unwrap_or(""),
            self.description.as_deref().unwrap_or("")
        )
    }

    /// Generate an encrypted XML reply for safe mode.
    pub fn to_encrypted_xml(&self, client: &WeChatClient) -> Result<String> {
        client.encrypt_reply_auto(&self.to_xml())
    }

    /// Generate an encrypted XML reply with custom timestamp and nonce.
    pub fn to_encrypted_xml_with(
        &self,
        client: &WeChatClient,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        client.encrypt_reply(&self.to_xml(), timestamp, nonce)
    }
}

/// News article item for news reply.
#[derive(Debug, Clone, Serialize)]
pub struct NewsArticle {
    pub title: String,
    pub description: String,
    pub pic_url: String,
    pub url: String,
}

/// News reply message (up to 8 articles).
#[derive(Debug, Clone)]
pub struct NewsReply {
    pub base: ReplyBase,
    pub articles: Vec<NewsArticle>,
}

impl NewsReply {
    pub fn new(
        to_user: impl Into<String>,
        from_user: impl Into<String>,
        articles: Vec<NewsArticle>,
    ) -> Self {
        Self {
            base: ReplyBase {
                to_user_name: to_user.into(),
                from_user_name: from_user.into(),
                create_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            },
            articles,
        }
    }

    pub fn to_xml(&self) -> String {
        let articles_xml: String = self
            .articles
            .iter()
            .map(|a| {
                format!(
                    r#"<item>
<Title><![CDATA[{}]]></Title>
<Description><![CDATA[{}]]></Description>
<PicUrl><![CDATA[{}]]></PicUrl>
<Url><![CDATA[{}]]></Url>
</item>"#,
                    a.title, a.description, a.pic_url, a.url
                )
            })
            .collect();

        format!(
            r#"<xml>
<ToUserName><![CDATA[{}]]></ToUserName>
<FromUserName><![CDATA[{}]]></FromUserName>
<CreateTime>{}</CreateTime>
<MsgType><![CDATA[news]]></MsgType>
<ArticleCount>{}</ArticleCount>
<Articles>
{}
</Articles>
</xml>"#,
            self.base.to_user_name,
            self.base.from_user_name,
            self.base.create_time,
            self.articles.len(),
            articles_xml
        )
    }

    /// Generate an encrypted XML reply for safe mode.
    pub fn to_encrypted_xml(&self, client: &WeChatClient) -> Result<String> {
        client.encrypt_reply_auto(&self.to_xml())
    }

    /// Generate an encrypted XML reply with custom timestamp and nonce.
    pub fn to_encrypted_xml_with(
        &self,
        client: &WeChatClient,
        timestamp: &str,
        nonce: &str,
    ) -> Result<String> {
        client.encrypt_reply(&self.to_xml(), timestamp, nonce)
    }
}

/// Return empty response to WeChat (no reply).
pub fn empty_reply() -> &'static str {
    "success"
}
