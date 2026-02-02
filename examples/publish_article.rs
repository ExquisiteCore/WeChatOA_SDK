//! Example: Publishing articles.
//!
//! This example demonstrates the full flow of publishing an article:
//! 1. Upload cover image
//! 2. Upload content images
//! 3. Create draft
//! 4. Publish or mass send

use std::env;
use std::fs;
use wechat_oa_sdk::models::material::MaterialType;
use wechat_oa_sdk::models::publish::Article;
use wechat_oa_sdk::{Config, WeChatClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = env::var("WECHAT_APP_ID")?;
    let app_secret = env::var("WECHAT_APP_SECRET")?;
    let token = env::var("WECHAT_TOKEN").unwrap_or_else(|_| "your_token".to_string());

    let config = Config::new(app_id, app_secret, token);
    let client = WeChatClient::new(config);

    // Step 1: Upload cover image (permanent material)
    println!("Uploading cover image...");
    let cover_data = fs::read("cover.jpg").expect("cover.jpg not found");
    let cover = client
        .upload_permanent_media(MaterialType::Image, "cover.jpg", cover_data)
        .await?;
    println!("Cover media_id: {}", cover.media_id);

    // Step 2: Upload content image (for embedding in article)
    println!("Uploading content image...");
    let content_img_data = fs::read("content.jpg").expect("content.jpg not found");
    let content_img_url = client
        .upload_article_image("content.jpg", content_img_data)
        .await?;
    println!("Content image URL: {}", content_img_url);

    // Step 3: Create article content (HTML)
    let content = format!(
        r#"
        <p>This is the article content.</p>
        <p><img src="{}" /></p>
        <p>More content here...</p>
        "#,
        content_img_url
    );

    // Step 4: Create article and save as draft
    let article = Article::new("Article Title", content, &cover.media_id)
        .with_author("Author Name")
        .with_digest("This is the article summary shown in the list.")
        .with_content_source_url("https://example.com/original");

    println!("Creating draft...");
    let draft_id = client.add_draft(vec![article]).await?;
    println!("Draft created: {}", draft_id);

    // Step 5a: Publish (creates a URL, doesn't push to followers)
    println!("Publishing...");
    let publish_id = client.submit_publish(&draft_id).await?;
    println!("Publish submitted: {}", publish_id);

    // Check publish status
    loop {
        let status = client.get_publish_status(&publish_id).await?;
        println!("Publish status: {}", status.publish_status);

        match status.publish_status {
            0 => {
                // Success
                if let Some(article_id) = status.article_id {
                    println!("Published! Article ID: {}", article_id);

                    // Get article URL
                    if let Some(detail) = status.article_detail {
                        for item in detail.item {
                            println!("Article URL: {}", item.article_url);
                        }
                    }
                }
                break;
            }
            1 => {
                // Still publishing
                println!("Still publishing, waiting...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
            _ => {
                // Failed
                println!("Publish failed with status: {}", status.publish_status);
                break;
            }
        }
    }

    // Step 5b: Or mass send to all followers (commented out)
    // println!("Mass sending...");
    // let mass_result = client.mass_send_article(&draft_id, None, true).await?;
    // println!("Mass send msg_id: {}", mass_result.msg_id);

    Ok(())
}
