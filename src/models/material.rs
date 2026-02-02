use serde::{Deserialize, Serialize};

/// Material type for upload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialType {
    Image,
    Voice,
    Video,
    Thumb,
}

impl MaterialType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Voice => "voice",
            Self::Video => "video",
            Self::Thumb => "thumb",
        }
    }
}

/// Response from uploading temporary media.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadTempMediaResponse {
    #[serde(rename = "type")]
    pub media_type: String,
    pub media_id: String,
    pub created_at: i64,
}

/// Response from uploading permanent media.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadPermanentMediaResponse {
    pub media_id: String,
    pub url: Option<String>,
}

/// Material item in list response.
#[derive(Debug, Clone, Deserialize)]
pub struct MaterialItem {
    pub media_id: String,
    pub name: Option<String>,
    pub update_time: i64,
    pub url: Option<String>,
}

/// Response from getting material list.
#[derive(Debug, Clone, Deserialize)]
pub struct MaterialListResponse {
    pub total_count: i32,
    pub item_count: i32,
    pub item: Vec<MaterialItem>,
}

/// Response from getting material count.
#[derive(Debug, Clone, Deserialize)]
pub struct MaterialCountResponse {
    pub voice_count: i32,
    pub video_count: i32,
    pub image_count: i32,
    pub news_count: i32,
}

/// Video description for permanent video upload.
#[derive(Debug, Clone, Serialize)]
pub struct VideoDescription {
    pub title: String,
    pub introduction: String,
}
