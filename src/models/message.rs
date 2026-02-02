use serde::Deserialize;

/// Text message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextMessage {
    /// Developer's WeChat account (original ID)
    pub to_user_name: String,
    /// Sender's OpenID
    pub from_user_name: String,
    /// Message creation timestamp
    pub create_time: i64,
    /// Message ID
    pub msg_id: Option<i64>,
    /// Text content
    pub content: String,
}

/// Image message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Image URL
    pub pic_url: String,
    /// Media ID for downloading
    pub media_id: String,
}

/// Voice message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VoiceMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Media ID
    pub media_id: String,
    /// Voice format (e.g., "amr", "speex")
    pub format: String,
    /// Voice recognition result (if enabled)
    pub recognition: Option<String>,
}

/// Video message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VideoMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Media ID
    pub media_id: String,
    /// Thumbnail media ID
    pub thumb_media_id: String,
}

/// Short video message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShortVideoMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Media ID
    pub media_id: String,
    /// Thumbnail media ID
    pub thumb_media_id: String,
}

/// Location message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Latitude
    #[serde(rename = "Location_X")]
    pub location_x: f64,
    /// Longitude
    #[serde(rename = "Location_Y")]
    pub location_y: f64,
    /// Map scale
    pub scale: i32,
    /// Location description
    pub label: String,
}

/// Link message from user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: i64,
    pub msg_id: Option<i64>,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// URL
    pub url: String,
}

/// Incoming message type indicator (from MsgType field).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgType {
    Text,
    Image,
    Voice,
    Video,
    ShortVideo,
    Location,
    Link,
    Event,
}

/// Helper struct to peek at MsgType before full deserialization.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct MsgTypePeek {
    pub msg_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn test_parse_text_message() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[Hello World]]></Content>
<MsgId>1234567890123456</MsgId>
</xml>"#;

        let msg: TextMessage = from_str(xml).unwrap();
        assert_eq!(msg.to_user_name, "gh_test");
        assert_eq!(msg.from_user_name, "oUser123");
        assert_eq!(msg.create_time, 1348831860);
        assert_eq!(msg.content, "Hello World");
        assert_eq!(msg.msg_id, Some(1234567890123456));
    }

    #[test]
    fn test_parse_image_message() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[image]]></MsgType>
<PicUrl><![CDATA[http://example.com/pic.jpg]]></PicUrl>
<MediaId><![CDATA[media_id_123]]></MediaId>
<MsgId>1234567890123456</MsgId>
</xml>"#;

        let msg: ImageMessage = from_str(xml).unwrap();
        assert_eq!(msg.to_user_name, "gh_test");
        assert_eq!(msg.pic_url, "http://example.com/pic.jpg");
        assert_eq!(msg.media_id, "media_id_123");
    }

    #[test]
    fn test_parse_location_message() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[location]]></MsgType>
<Location_X>23.134521</Location_X>
<Location_Y>113.358803</Location_Y>
<Scale>20</Scale>
<Label><![CDATA[Some location]]></Label>
<MsgId>1234567890123456</MsgId>
</xml>"#;

        let msg: LocationMessage = from_str(xml).unwrap();
        assert!((msg.location_x - 23.134521).abs() < 0.0001);
        assert!((msg.location_y - 113.358803).abs() < 0.0001);
        assert_eq!(msg.scale, 20);
        assert_eq!(msg.label, "Some location");
    }

    #[test]
    fn test_peek_msg_type() {
        let xml = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[test]]></Content>
</xml>"#;

        let peek: MsgTypePeek = from_str(xml).unwrap();
        assert_eq!(peek.msg_type, "text");
    }
}
