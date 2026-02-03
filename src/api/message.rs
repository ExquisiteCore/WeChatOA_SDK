use quick_xml::de::from_str;
use serde::Deserialize;

use crate::client::WeChatClient;
use crate::crypto::{
    check_msg_signature, check_signature, compute_msg_signature, decode_aes_key, decrypt_message,
    encrypt_message, generate_encrypted_xml, generate_nonce,
};
use crate::error::{Result, WeChatError};
use crate::models::event::*;
use crate::models::message::*;

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
            "TEMPLATESENDJOBFINISH" => {
                IncomingMessage::TemplateSendJobFinishEvent(from_str(xml_body)?)
            }
            _ => IncomingMessage::Unknown(xml_body.to_string()),
        };

        Ok(event)
    }

    // ==================== Encrypted Message Handling ====================

    /// Get the decoded AES key from config.
    ///
    /// Returns an error if `encoding_aes_key` is not configured.
    fn get_aes_key(&self) -> Result<[u8; 32]> {
        let encoding_aes_key = self
            .config
            .encoding_aes_key
            .as_ref()
            .ok_or(WeChatError::AesKeyNotConfigured)?;
        decode_aes_key(encoding_aes_key)
    }

    /// Verify the encrypted message signature.
    ///
    /// Used to verify incoming encrypted messages from WeChat.
    pub fn verify_msg_signature(
        &self,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
        encrypt_msg: &str,
    ) -> bool {
        check_msg_signature(&self.config.token, msg_signature, timestamp, nonce, encrypt_msg)
    }

    /// Parse an encrypted incoming message or event from WeChat.
    ///
    /// This should be used when your server is configured in "safe mode" or "compatible mode".
    ///
    /// # Arguments
    /// - `xml_body`: The raw XML POST body containing the encrypted message
    /// - `msg_signature`: The signature from query parameter
    /// - `timestamp`: The timestamp from query parameter
    /// - `nonce`: The nonce from query parameter
    ///
    /// # Example
    /// ```ignore
    /// let msg = client.parse_encrypted_message(
    ///     &xml_body,
    ///     &query.msg_signature,
    ///     &query.timestamp,
    ///     &query.nonce,
    /// )?;
    /// ```
    pub fn parse_encrypted_message(
        &self,
        xml_body: &str,
        msg_signature: &str,
        timestamp: &str,
        nonce: &str,
    ) -> Result<IncomingMessage> {
        // Parse the encrypted XML to get the Encrypt field
        let encrypted_xml: EncryptedXml = from_str(xml_body)?;

        // Verify the signature
        if !self.verify_msg_signature(msg_signature, timestamp, nonce, &encrypted_xml.encrypt) {
            return Err(WeChatError::InvalidSignature);
        }

        // Decrypt the message
        let aes_key = self.get_aes_key()?;
        let (decrypted_xml, app_id) = decrypt_message(&aes_key, &encrypted_xml.encrypt)?;

        // Verify AppID matches
        if app_id != self.config.app_id {
            return Err(WeChatError::AppIdMismatch);
        }

        // Parse the decrypted XML as a regular message
        self.parse_message(&decrypted_xml)
    }

    /// Decrypt the echostr for server verification in encrypted mode.
    ///
    /// When WeChat verifies your server URL in encrypted mode, it sends an encrypted echostr.
    /// You need to decrypt it and return the decrypted content.
    ///
    /// # Arguments
    /// - `encrypted_echostr`: The encrypted echostr from query parameter
    ///
    /// # Returns
    /// The decrypted echostr that should be returned to WeChat.
    pub fn decrypt_echostr(&self, encrypted_echostr: &str) -> Result<String> {
        let aes_key = self.get_aes_key()?;
        let (decrypted, _app_id) = decrypt_message(&aes_key, encrypted_echostr)?;
        Ok(decrypted)
    }

    /// Encrypt a reply XML for sending back to WeChat.
    ///
    /// This should be used when your server is configured in "safe mode".
    ///
    /// # Arguments
    /// - `reply_xml`: The plain reply XML (e.g., from `TextReply::to_xml()`)
    /// - `timestamp`: The timestamp to use (can be current time or from the original request)
    /// - `nonce`: The nonce to use (can generate a new one or use from the original request)
    ///
    /// # Returns
    /// The encrypted XML that should be returned to WeChat.
    ///
    /// # Example
    /// ```ignore
    /// let reply = TextReply::new(&to_user, &from_user, "Hello!");
    /// let encrypted_xml = client.encrypt_reply(&reply.to_xml(), &timestamp, &nonce)?;
    /// ```
    pub fn encrypt_reply(&self, reply_xml: &str, timestamp: &str, nonce: &str) -> Result<String> {
        let aes_key = self.get_aes_key()?;

        // Encrypt the reply XML
        let encrypted = encrypt_message(&aes_key, &self.config.app_id, reply_xml)?;

        // Generate the signature
        let signature = compute_msg_signature(&self.config.token, timestamp, nonce, &encrypted);

        // Generate the final encrypted XML
        Ok(generate_encrypted_xml(&encrypted, &signature, timestamp, nonce))
    }

    /// Encrypt a reply XML with auto-generated timestamp and nonce.
    ///
    /// Convenience method that generates timestamp and nonce automatically.
    pub fn encrypt_reply_auto(&self, reply_xml: &str) -> Result<String> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let nonce = generate_nonce();
        self.encrypt_reply(reply_xml, &timestamp, &nonce)
    }
}

/// Encrypted message XML structure.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EncryptedXml {
    encrypt: String,
}
