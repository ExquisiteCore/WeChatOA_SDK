//! Basic usage example for WeChatOA SDK.
//!
//! Run with: cargo run --example basic
//!
//! Make sure to set environment variables:
//! - WECHAT_APP_ID
//! - WECHAT_APP_SECRET
//! - WECHAT_TOKEN

use std::env;
use wechat_oa_sdk::{Config, WeChatClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables
    let app_id = env::var("WECHAT_APP_ID").expect("WECHAT_APP_ID not set");
    let app_secret = env::var("WECHAT_APP_SECRET").expect("WECHAT_APP_SECRET not set");
    let token = env::var("WECHAT_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    // Create client
    let config = Config::new(app_id, app_secret, token);
    let client = WeChatClient::new(config);

    // Get access token (automatically cached and refreshed)
    let access_token = client.access_token().await?;
    println!("Access token: {}...", &access_token[..20]);

    // Get callback IP list
    let ips = client.get_callback_ip_list().await?;
    println!("WeChat callback IPs: {} addresses", ips.len());

    // Example: Get user list
    let users = client.get_user_list(None).await?;
    println!("Total followers: {}", users.total);

    // Example: Get tags
    let tags = client.get_tags().await?;
    println!("Tags: {:?}", tags.tags);

    // Example: Get draft count
    let draft_count = client.get_draft_count().await?;
    println!("Drafts in box: {}", draft_count);

    Ok(())
}
