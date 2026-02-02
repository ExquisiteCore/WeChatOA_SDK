use thiserror::Error;

pub type Result<T> = std::result::Result<T, WeChatError>;

#[derive(Debug, Error)]
pub enum WeChatError {
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WeChat API error: [{errcode}] {errmsg}")]
    Api { errcode: i64, errmsg: String },

    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::DeError),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Access token not available")]
    TokenUnavailable,
}
