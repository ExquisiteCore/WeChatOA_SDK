/// WeChat Official Account SDK configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// The AppID from WeChat Official Account settings.
    pub app_id: String,
    /// The AppSecret from WeChat Official Account settings.
    pub app_secret: String,
    /// The Token used for server callback signature verification.
    pub token: String,
    /// The EncodingAESKey for message encryption/decryption (optional).
    pub encoding_aes_key: Option<String>,
}

impl Config {
    /// Create a new Config.
    ///
    /// - `app_id`: AppID from WeChat Official Account settings
    /// - `app_secret`: AppSecret from WeChat Official Account settings
    /// - `token`: Token for server callback signature verification (set in 服务器配置)
    pub fn new(
        app_id: impl Into<String>,
        app_secret: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            token: token.into(),
            encoding_aes_key: None,
        }
    }

    pub fn with_encoding_aes_key(mut self, key: impl Into<String>) -> Self {
        self.encoding_aes_key = Some(key.into());
        self
    }
}
