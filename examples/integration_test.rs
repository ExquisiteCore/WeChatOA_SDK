//! Integration test for WeChat OA SDK.
//!
//! Run with:
//!   WECHAT_APP_ID=xxx WECHAT_APP_SECRET=xxx WECHAT_TOKEN=xxx cargo run --example integration_test

use std::env;
use wechat_oa_sdk::{Config, WeChatClient, WeChatError};

#[tokio::main]
async fn main() {
    println!("=== WeChat OA SDK Integration Test ===\n");

    // Load config
    let app_id = env::var("WECHAT_APP_ID").unwrap_or_else(|_| {
        eprintln!("Error: WECHAT_APP_ID not set");
        eprintln!("Usage: WECHAT_APP_ID=xxx WECHAT_APP_SECRET=xxx WECHAT_TOKEN=xxx cargo run --example integration_test");
        std::process::exit(1);
    });
    let app_secret = env::var("WECHAT_APP_SECRET").expect("WECHAT_APP_SECRET not set");
    let token = env::var("WECHAT_TOKEN").unwrap_or_else(|_| "test_token".to_string());

    let config = Config::new(&app_id, &app_secret, &token);
    let client = WeChatClient::new(config);

    let mut passed = 0;
    let mut failed = 0;

    // Test 1: Access Token
    print!("[1/8] Access Token... ");
    match client.access_token().await {
        Ok(token) => {
            println!("OK ({}...)", &token[..20.min(token.len())]);
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
            println!("\nCannot continue without valid access token.");
            println!("Please check your APP_ID and APP_SECRET.");
            return;
        }
    }

    // Test 2: Get Callback IPs
    print!("[2/8] Get Callback IPs... ");
    match client.get_callback_ip_list().await {
        Ok(ips) => {
            println!("OK ({} IPs)", ips.len());
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Test 3: Get User List
    print!("[3/8] Get User List... ");
    match client.get_user_list(None).await {
        Ok(users) => {
            println!("OK (total: {} followers)", users.total);
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Test 4: Get Tags
    print!("[4/8] Get Tags... ");
    match client.get_tags().await {
        Ok(tags) => {
            println!("OK ({} tags)", tags.tags.len());
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Test 5: Get Draft Count
    print!("[5/8] Get Draft Count... ");
    match client.get_draft_count().await {
        Ok(count) => {
            println!("OK ({} drafts)", count);
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Test 6: Get Material Count
    print!("[6/8] Get Material Count... ");
    match client.get_material_count().await {
        Ok(count) => {
            println!(
                "OK (image: {}, voice: {}, video: {}, news: {})",
                count.image_count, count.voice_count, count.video_count, count.news_count
            );
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Test 7: Get Menu
    print!("[7/8] Get Menu... ");
    match client.get_menu().await {
        Ok(menu) => {
            let has_menu = menu.menu.is_some();
            println!("OK (menu configured: {})", has_menu);
            passed += 1;
        }
        Err(e) => {
            if let WeChatError::Api { errcode, .. } = &e {
                if *errcode == 46003 {
                    println!("OK (no menu configured)");
                    passed += 1;
                } else {
                    println!("FAILED: {}", e);
                    failed += 1;
                }
            } else {
                println!("FAILED: {}", e);
                failed += 1;
            }
        }
    }

    // Test 8: Get Published Articles
    print!("[8/8] Get Published Articles... ");
    match client.get_publish_list(0, 5, true).await {
        Ok(list) => {
            println!("OK ({} articles)", list.total_count);
            passed += 1;
        }
        Err(e) => {
            println!("FAILED: {}", e);
            failed += 1;
        }
    }

    // Summary
    println!("\n=== Test Summary ===");
    println!("Passed: {}", passed);
    println!("Failed: {}", failed);

    if failed == 0 {
        println!("\nAll tests passed! SDK is working correctly.");
    } else {
        println!("\nSome tests failed. Check your account permissions.");
    }
}
