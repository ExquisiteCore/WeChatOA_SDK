use quick_xml::de::from_str;

use crate::client::WeChatClient;
use crate::crypto::check_signature;
use crate::error::Result;
use crate::models::message::*;
use crate::models::event::*;

/// Incoming message/event from WeChat.
#[derive(Debug, Clone)]
pub enum IncomingMessage {
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    Video(VideoMessage),
    ShortVideo(ShortVideoMessage),
    Location(LocationMessage),
    Link(LinkMessage),
    SubscribeEvent(SubscribeEvent),
    UnsubscribeEvent(SubscribeEvent),
    ScanEvent(ScanEvent),
    LocationEvent(LocationEvent),
    MenuClickEvent(MenuClickEvent),
    MenuViewEvent(MenuViewEvent),
    TemplateSendJobFinishEvent(TemplateSendJobFinishEvent),
    Unknown(String),
}

impl WeChatClient {
    /// Verify the callback signature from WeChat server.
    ///
    /// This should be called when WeChat sends a verification request
    /// (GET request with signature, timestamp, nonce, echostr).
    ///
    /// Returns `true` if the signature is valid.
    pub fn verify_signature(&self, signature: &str, timestamp: &str, nonce: &str) -> bool {
        check_signature(&self.config.token, signature, timestamp, nonce)
    }

    /// Parse an incoming message or event from WeChat.
    ///
    /// The `xml_body` is the raw XML POST body from WeChat callback.
    pub fn parse_message(&self, xml_body: &str) -> Result<IncomingMessage> {
        // First, peek at the MsgType
        let peek: MsgTypePeek = from_str(xml_body)?;
        let msg_type = peek.msg_type.to_lowercase();

        let message = match msg_type.as_str() {
            "text" => IncomingMessage::Text(from_str(xml_body)?),
            "image" => IncomingMessage::Image(from_str(xml_body)?),
            "voice" => IncomingMessage::Voice(from_str(xml_body)?),
            "video" => IncomingMessage::Video(from_str(xml_body)?),
            "shortvideo" => IncomingMessage::ShortVideo(from_str(xml_body)?),
            "location" => IncomingMessage::Location(from_str(xml_body)?),
            "link" => IncomingMessage::Link(from_str(xml_body)?),
            "event" => self.parse_event(xml_body)?,
            _ => IncomingMessage::Unknown(xml_body.to_string()),
        };

        Ok(message)
    }

    fn parse_event(&self, xml_body: &str) -> Result<IncomingMessage> {
        let peek: EventTypePeek = from_str(xml_body)?;
        let event_type = peek.event.to_uppercase();

        let event = match event_type.as_str() {
            "SUBSCRIBE" => IncomingMessage::SubscribeEvent(from_str(xml_body)?),
            "UNSUBSCRIBE" => IncomingMessage::UnsubscribeEvent(from_str(xml_body)?),
            "SCAN" => IncomingMessage::ScanEvent(from_str(xml_body)?),
            "LOCATION" => IncomingMessage::LocationEvent(from_str(xml_body)?),
            "CLICK" => IncomingMessage::MenuClickEvent(from_str(xml_body)?),
            "VIEW" => IncomingMessage::MenuViewEvent(from_str(xml_body)?),
            "TEMPLATESENDJOBFINISH" => IncomingMessage::TemplateSendJobFinishEvent(from_str(xml_body)?),
            _ => IncomingMessage::Unknown(xml_body.to_string()),
        };

        Ok(event)
    }
}
