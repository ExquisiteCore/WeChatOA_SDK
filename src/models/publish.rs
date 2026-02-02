use serde::{Deserialize, Serialize};

/// Article item for draft/publish.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    /// Article title
    pub title: String,
    /// Author (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Abstract/digest (optional, auto-generated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// Article content (HTML format, required)
    pub content: String,
    /// Content source URL (optional, "Read original" link)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_source_url: Option<String>,
    /// Cover image media_id (required)
    pub thumb_media_id: String,
    /// Whether to show cover in article (0 = no, 1 = yes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_open_comment: Option<i32>,
    /// Whether only fans can comment (0 = no, 1 = yes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_fans_can_comment: Option<i32>,
    /// Cover image position (0 = article top, 1 = card style, 2/3 other styles)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_crop_235_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_crop_1_1: Option<String>,
}

impl Article {
    /// Create a new article with required fields.
    pub fn new(
        title: impl Into<String>,
        content: impl Into<String>,
        thumb_media_id: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            author: None,
            digest: None,
            content: content.into(),
            content_source_url: None,
            thumb_media_id: thumb_media_id.into(),
            need_open_comment: None,
            only_fans_can_comment: None,
            pic_crop_235_1: None,
            pic_crop_1_1: None,
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn with_digest(mut self, digest: impl Into<String>) -> Self {
        self.digest = Some(digest.into());
        self
    }

    pub fn with_content_source_url(mut self, url: impl Into<String>) -> Self {
        self.content_source_url = Some(url.into());
        self
    }
}

/// Draft request body.
#[derive(Debug, Clone, Serialize)]
pub struct DraftAddRequest {
    pub articles: Vec<Article>,
}

/// Response from adding a draft.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftAddResponse {
    pub media_id: String,
}

/// Draft item in list.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftItem {
    pub media_id: String,
    pub content: DraftContent,
    pub update_time: i64,
}

/// Draft content containing articles.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftContent {
    pub news_item: Vec<DraftArticle>,
}

/// Article in draft.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftArticle {
    pub title: String,
    pub author: Option<String>,
    pub digest: Option<String>,
    pub content: String,
    pub content_source_url: Option<String>,
    pub thumb_media_id: String,
    pub url: Option<String>,
    pub thumb_url: Option<String>,
}

/// Response from getting draft list.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftListResponse {
    pub total_count: i32,
    pub item_count: i32,
    pub item: Vec<DraftItem>,
}

/// Response from getting draft count.
#[derive(Debug, Clone, Deserialize)]
pub struct DraftCountResponse {
    pub total_count: i32,
}

/// Response from submitting publish.
#[derive(Debug, Clone, Deserialize)]
pub struct PublishSubmitResponse {
    pub publish_id: String,
}

/// Publish status.
#[derive(Debug, Clone, Deserialize)]
pub struct PublishStatusResponse {
    pub publish_id: String,
    /// 0 = success, 1 = publishing, 2 = original failed, 3 = regular failed,
    /// 4 = platform review failed, 5 = success (waiting for push), 6 = failed (deleted by system)
    pub publish_status: i32,
    pub article_id: Option<String>,
    pub article_detail: Option<PublishArticleDetail>,
    pub fail_idx: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishArticleDetail {
    pub count: i32,
    pub item: Vec<PublishArticleItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishArticleItem {
    pub idx: i32,
    pub article_url: String,
}

/// Published article list response.
#[derive(Debug, Clone, Deserialize)]
pub struct PublishListResponse {
    pub total_count: i32,
    pub item_count: i32,
    pub item: Vec<PublishItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishItem {
    pub article_id: String,
    pub content: PublishContent,
    pub update_time: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishContent {
    pub news_item: Vec<PublishNewsItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishNewsItem {
    pub title: String,
    pub author: Option<String>,
    pub digest: Option<String>,
    pub content: String,
    pub content_source_url: Option<String>,
    pub thumb_media_id: String,
    pub url: String,
    pub thumb_url: Option<String>,
    pub is_deleted: Option<bool>,
}

/// Mass send filter for targeting.
#[derive(Debug, Clone, Serialize)]
pub struct MassFilter {
    pub is_to_all: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i32>,
}

/// Mass send request for articles.
#[derive(Debug, Clone, Serialize)]
pub struct MassSendRequest {
    pub filter: MassFilter,
    pub mpnews: MassMediaId,
    pub msgtype: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_ignore_reprint: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MassMediaId {
    pub media_id: String,
}

/// Mass send response.
#[derive(Debug, Clone, Deserialize)]
pub struct MassSendResponse {
    pub msg_id: i64,
    pub msg_data_id: Option<i64>,
}

/// Mass send status response.
#[derive(Debug, Clone, Deserialize)]
pub struct MassStatusResponse {
    pub msg_id: i64,
    /// "SEND_SUCCESS", "SENDING", "SEND_FAIL", "DELETE"
    pub msg_status: String,
}
