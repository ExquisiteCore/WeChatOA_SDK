//! Example: Message handling for WeChat callback server.
//!
//! This example shows how to parse incoming messages and generate replies.
//! In a real application, you would integrate this with a web framework
//! like axum, actix-web, or rocket.

use wechat_oa_sdk::Config;
use wechat_oa_sdk::WeChatClient;
use wechat_oa_sdk::api::message::IncomingMessage;
use wechat_oa_sdk::models::reply::{TextReply, NewsReply, NewsArticle, empty_reply};

fn main() {
    // Create client with token for signature verification
    let config = Config::new("your_app_id", "your_app_secret", "your_callback_token");
    let client = WeChatClient::new(config);

    // Example: Verify signature (called on GET request from WeChat)
    let signature = "abc123";
    let timestamp = "1234567890";
    let nonce = "random";

    if client.verify_signature(signature, timestamp, nonce) {
        println!("Signature valid!");
        // Return echostr to WeChat
    }

    // Example: Parse incoming message (called on POST request from WeChat)
    let xml_body = r#"<xml>
<ToUserName><![CDATA[gh_test]]></ToUserName>
<FromUserName><![CDATA[oUser123]]></FromUserName>
<CreateTime>1348831860</CreateTime>
<MsgType><![CDATA[text]]></MsgType>
<Content><![CDATA[Hello]]></Content>
<MsgId>1234567890123456</MsgId>
</xml>"#;

    match client.parse_message(xml_body) {
        Ok(msg) => {
            let reply = handle_message(msg);
            println!("Reply XML:\n{}", reply);
        }
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
        }
    }
}

fn handle_message(msg: IncomingMessage) -> String {
    match msg {
        IncomingMessage::Text(text) => {
            // Echo the message back
            let reply = TextReply::new(
                &text.from_user_name,
                &text.to_user_name,
                format!("You said: {}", text.content),
            );
            reply.to_xml()
        }
        IncomingMessage::SubscribeEvent(event) => {
            // Welcome new followers
            let reply = TextReply::new(
                &event.from_user_name,
                &event.to_user_name,
                "Welcome! Thanks for subscribing.",
            );
            reply.to_xml()
        }
        IncomingMessage::MenuClickEvent(event) => {
            // Handle menu click
            let reply = TextReply::new(
                &event.from_user_name,
                &event.to_user_name,
                format!("You clicked menu: {}", event.event_key),
            );
            reply.to_xml()
        }
        IncomingMessage::Image(img) => {
            // Reply with news article
            let articles = vec![
                NewsArticle {
                    title: "Thanks for the image!".to_string(),
                    description: "We received your image.".to_string(),
                    pic_url: img.pic_url.clone(),
                    url: "https://example.com".to_string(),
                }
            ];
            let reply = NewsReply::new(&img.from_user_name, &img.to_user_name, articles);
            reply.to_xml()
        }
        _ => {
            // Return empty response for unhandled messages
            empty_reply().to_string()
        }
    }
}
