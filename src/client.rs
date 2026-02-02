use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Deserialize;

use crate::access_token::TokenManager;
use crate::config::Config;
use crate::error::{Result, WeChatError};

const WECHAT_API_BASE: &str = "https://api.weixin.qq.com/cgi-bin";

/// The main entry point for the WeChat Official Account SDK.
pub struct WeChatClient {
    pub(crate) config: Config,
    pub(crate) http: reqwest::Client,
    pub(crate) token_manager: Arc<TokenManager>,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    errcode: Option<i64>,
    errmsg: Option<String>,
}

impl WeChatClient {
    /// Create a new `WeChatClient` with the given configuration.
    pub fn new(config: Config) -> Self {
        let http = reqwest::Client::new();
        let token_manager = Arc::new(TokenManager::new(&config, http.clone()));
        Self {
            config,
            http,
            token_manager,
        }
    }

    /// Get a valid access token (auto-refreshed if expired).
    pub async fn access_token(&self) -> Result<String> {
        self.token_manager.get_token().await
    }

    /// Make a GET request to a WeChat API endpoint with automatic token injection.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str, query: &[(&str, &str)]) -> Result<T> {
        let token = self.access_token().await?;
        let url = format!("{}{}", WECHAT_API_BASE, path);

        let mut params = vec![("access_token", token.as_str())];
        params.extend(query.iter().map(|(k, v)| (*k, *v)));

        let resp = self.http.get(&url).query(&params).send().await?;
        self.parse_response(resp).await
    }

    /// Make a POST request with a JSON body to a WeChat API endpoint.
    pub(crate) async fn post_json<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        let token = self.access_token().await?;
        let url = format!("{}{}?access_token={}", WECHAT_API_BASE, path, token);

        let resp = self.http.post(&url).json(body).send().await?;
        self.parse_response(resp).await
    }

    /// Parse a response, checking for WeChat API errors.
    async fn parse_response<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T> {
        let text = resp.text().await?;

        // Check for API error first
        if let Ok(err_resp) = serde_json::from_str::<ApiErrorResponse>(&text) {
            if let Some(errcode) = err_resp.errcode {
                if errcode != 0 {
                    return Err(WeChatError::Api {
                        errcode,
                        errmsg: err_resp.errmsg.unwrap_or_default(),
                    });
                }
            }
        }

        serde_json::from_str(&text).map_err(WeChatError::from)
    }
}
