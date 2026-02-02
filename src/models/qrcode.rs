use serde::Deserialize;

/// QR code action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QrCodeAction {
    /// Temporary QR code (expires)
    Temporary,
    /// Permanent QR code (never expires, limited quantity)
    Permanent,
    /// Temporary QR code with string scene
    TemporaryStr,
    /// Permanent QR code with string scene
    PermanentStr,
}

impl QrCodeAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Temporary => "QR_SCENE",
            Self::Permanent => "QR_LIMIT_SCENE",
            Self::TemporaryStr => "QR_STR_SCENE",
            Self::PermanentStr => "QR_LIMIT_STR_SCENE",
        }
    }
}

/// QR code creation response.
#[derive(Debug, Clone, Deserialize)]
pub struct QrCodeResponse {
    /// Ticket for fetching QR code image
    pub ticket: String,
    /// Expiration time in seconds (only for temporary QR codes)
    pub expire_seconds: Option<i64>,
    /// URL to display the QR code (can be converted to short URL)
    pub url: String,
}

/// Short URL response.
#[derive(Debug, Clone, Deserialize)]
pub struct ShortUrlResponse {
    pub short_url: String,
}
