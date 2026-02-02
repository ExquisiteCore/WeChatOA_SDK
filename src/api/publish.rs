use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::{Result, WeChatError};
use crate::models::publish::*;

const WECHAT_API_BASE: &str = "https://api.weixin.qq.com/cgi-bin";

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiResponse {
    errcode: Option<i64>,
    errmsg: Option<String>,
}

impl WeChatClient {
    // ========== Draft (草稿箱) APIs ==========

    /// Add a new draft with one or more articles.
    ///
    /// Returns the media_id of the created draft.
    pub async fn add_draft(&self, articles: Vec<Article>) -> Result<String> {
        let body = DraftAddRequest { articles };
        let resp: DraftAddResponse = self.post_json("/draft/add", &body).await?;
        Ok(resp.media_id)
    }

    /// Get a draft by media_id.
    pub async fn get_draft(&self, media_id: &str) -> Result<DraftContent> {
        let body = serde_json::json!({ "media_id": media_id });

        #[derive(Deserialize)]
        struct Response {
            news_item: Vec<DraftArticle>,
        }

        let resp: Response = self.post_json("/draft/get", &body).await?;
        Ok(DraftContent {
            news_item: resp.news_item,
        })
    }

    /// Delete a draft by media_id.
    pub async fn delete_draft(&self, media_id: &str) -> Result<()> {
        let body = serde_json::json!({ "media_id": media_id });
        let _: ApiResponse = self.post_json("/draft/delete", &body).await?;
        Ok(())
    }

    /// Update an article in a draft.
    ///
    /// - `media_id`: The draft media_id
    /// - `index`: The article index (0-based)
    /// - `article`: The updated article content
    pub async fn update_draft(&self, media_id: &str, index: i32, article: Article) -> Result<()> {
        let body = serde_json::json!({
            "media_id": media_id,
            "index": index,
            "articles": article
        });
        let _: ApiResponse = self.post_json("/draft/update", &body).await?;
        Ok(())
    }

    /// Get the total count of drafts.
    pub async fn get_draft_count(&self) -> Result<i32> {
        let resp: DraftCountResponse = self.get("/draft/count", &[]).await?;
        Ok(resp.total_count)
    }

    /// Get draft list.
    ///
    /// - `offset`: Starting position (0-based)
    /// - `count`: Number of items to return (1-20)
    /// - `no_content`: Whether to exclude article content (true = faster)
    pub async fn get_draft_list(
        &self,
        offset: i32,
        count: i32,
        no_content: bool,
    ) -> Result<DraftListResponse> {
        let body = serde_json::json!({
            "offset": offset,
            "count": count,
            "no_content": if no_content { 1 } else { 0 }
        });
        self.post_json("/draft/batchget", &body).await
    }

    // ========== Publish (发布) APIs ==========

    /// Submit a draft for publishing.
    ///
    /// This is for "发布" (not 群发). Published articles won't appear in
    /// followers' feeds but can be accessed via URL.
    ///
    /// Returns the publish_id to track publishing status.
    pub async fn submit_publish(&self, media_id: &str) -> Result<String> {
        let body = serde_json::json!({ "media_id": media_id });
        let resp: PublishSubmitResponse = self.post_json("/freepublish/submit", &body).await?;
        Ok(resp.publish_id)
    }

    /// Get the status of a publish job.
    pub async fn get_publish_status(&self, publish_id: &str) -> Result<PublishStatusResponse> {
        let body = serde_json::json!({ "publish_id": publish_id });
        self.post_json("/freepublish/get", &body).await
    }

    /// Delete a published article.
    ///
    /// - `article_id`: The article_id from publish
    /// - `index`: Article index (0-based), use 0 for single article
    pub async fn delete_publish(&self, article_id: &str, index: i32) -> Result<()> {
        let body = serde_json::json!({
            "article_id": article_id,
            "index": index
        });
        let _: ApiResponse = self.post_json("/freepublish/delete", &body).await?;
        Ok(())
    }

    /// Get a published article by article_id.
    pub async fn get_publish(&self, article_id: &str) -> Result<PublishContent> {
        let body = serde_json::json!({ "article_id": article_id });

        #[derive(Deserialize)]
        struct Response {
            news_item: Vec<PublishNewsItem>,
        }

        let resp: Response = self.post_json("/freepublish/getarticle", &body).await?;
        Ok(PublishContent {
            news_item: resp.news_item,
        })
    }

    /// Get list of published articles.
    ///
    /// - `offset`: Starting position (0-based)
    /// - `count`: Number of items (1-20)
    /// - `no_content`: Whether to exclude article content
    pub async fn get_publish_list(
        &self,
        offset: i32,
        count: i32,
        no_content: bool,
    ) -> Result<PublishListResponse> {
        let body = serde_json::json!({
            "offset": offset,
            "count": count,
            "no_content": if no_content { 1 } else { 0 }
        });
        self.post_json("/freepublish/batchget", &body).await
    }

    // ========== Mass Send (群发) APIs ==========

    /// Mass send an article to all followers or by tag.
    ///
    /// This is for "群发" - articles will appear in followers' feeds
    /// and push notifications.
    ///
    /// - `media_id`: Draft media_id to send
    /// - `tag_id`: Optional tag to filter recipients (None = all followers)
    /// - `ignore_reprint`: Whether to continue if original check fails (default true)
    pub async fn mass_send_article(
        &self,
        media_id: &str,
        tag_id: Option<i32>,
        ignore_reprint: bool,
    ) -> Result<MassSendResponse> {
        let body = MassSendRequest {
            filter: MassFilter {
                is_to_all: tag_id.is_none(),
                tag_id,
            },
            mpnews: MassMediaId {
                media_id: media_id.to_string(),
            },
            msgtype: "mpnews".to_string(),
            send_ignore_reprint: Some(if ignore_reprint { 1 } else { 0 }),
        };
        self.post_json("/message/mass/sendall", &body).await
    }

    /// Preview a mass send to a specific user (for testing).
    ///
    /// - `media_id`: Draft media_id
    /// - `to_user`: OpenID of the recipient
    pub async fn mass_preview(&self, media_id: &str, to_user: &str) -> Result<i64> {
        let body = serde_json::json!({
            "touser": to_user,
            "mpnews": { "media_id": media_id },
            "msgtype": "mpnews"
        });

        #[derive(Deserialize)]
        struct Response {
            msg_id: i64,
        }

        let resp: Response = self.post_json("/message/mass/preview", &body).await?;
        Ok(resp.msg_id)
    }

    /// Delete a mass send message.
    ///
    /// Can only delete within 30 minutes after sending.
    pub async fn mass_delete(&self, msg_id: i64, article_idx: Option<i32>) -> Result<()> {
        let body = serde_json::json!({
            "msg_id": msg_id,
            "article_idx": article_idx.unwrap_or(0)
        });
        let _: ApiResponse = self.post_json("/message/mass/delete", &body).await?;
        Ok(())
    }

    /// Get mass send status.
    pub async fn mass_get_status(&self, msg_id: i64) -> Result<MassStatusResponse> {
        let body = serde_json::json!({ "msg_id": msg_id.to_string() });
        self.post_json("/message/mass/get", &body).await
    }

    // ========== Image Upload for Articles ==========

    /// Upload an image for use in article content.
    ///
    /// Returns the URL of the uploaded image (can be used in article HTML).
    /// This is different from material upload - these images are specifically
    /// for embedding in article content.
    pub async fn upload_article_image(&self, file_name: &str, file_data: Vec<u8>) -> Result<String> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/media/uploadimg?access_token={}",
            WECHAT_API_BASE, token
        );

        let part = Part::bytes(file_data).file_name(file_name.to_string());
        let form = Form::new().part("media", part);

        let resp = self.http.post(&url).multipart(form).send().await?;
        let text = resp.text().await?;

        #[derive(Deserialize)]
        struct Response {
            url: Option<String>,
            errcode: Option<i64>,
            errmsg: Option<String>,
        }

        let resp: Response = serde_json::from_str(&text)?;

        if let Some(errcode) = resp.errcode {
            if errcode != 0 {
                return Err(WeChatError::Api {
                    errcode,
                    errmsg: resp.errmsg.unwrap_or_default(),
                });
            }
        }

        resp.url.ok_or(WeChatError::TokenUnavailable)
    }
}
