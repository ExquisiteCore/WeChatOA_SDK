use std::time::{Duration, Instant};

use serde::Deserialize;
use tokio::sync::RwLock;
use tracing;

use crate::config::Config;
use crate::error::{Result, WeChatError};

const WECHAT_TOKEN_URL: &str = "https://api.weixin.qq.com/cgi-bin/token";
/// Refresh the token 5 minutes before it expires.
const REFRESH_BUFFER: Duration = Duration::from_secs(300);

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    expires_in: Option<u64>,
    errcode: Option<i64>,
    errmsg: Option<String>,
}

struct TokenState {
    access_token: String,
    expires_at: Instant,
}

pub(crate) struct TokenManager {
    state: RwLock<Option<TokenState>>,
    app_id: String,
    app_secret: String,
    http: reqwest::Client,
}

impl TokenManager {
    pub fn new(config: &Config, http: reqwest::Client) -> Self {
        Self {
            state: RwLock::new(None),
            app_id: config.app_id.clone(),
            app_secret: config.app_secret.clone(),
            http,
        }
    }

    pub async fn get_token(&self) -> Result<String> {
        // Try read lock first — fast path
        {
            let state = self.state.read().await;
            if let Some(ref s) = *state {
                if Instant::now() < s.expires_at {
                    return Ok(s.access_token.clone());
                }
            }
        }

        // Need refresh — acquire write lock
        let mut state = self.state.write().await;

        // Double-check after acquiring write lock (another task may have refreshed)
        if let Some(ref s) = *state {
            if Instant::now() < s.expires_at {
                return Ok(s.access_token.clone());
            }
        }

        let new_state = self.fetch_token().await?;
        let token = new_state.access_token.clone();
        *state = Some(new_state);
        Ok(token)
    }

    async fn fetch_token(&self) -> Result<TokenState> {
        tracing::debug!("Fetching new access token");

        let resp: TokenResponse = self
            .http
            .get(WECHAT_TOKEN_URL)
            .query(&[
                ("grant_type", "client_credential"),
                ("appid", &self.app_id),
                ("secret", &self.app_secret),
            ])
            .send()
            .await?
            .json()
            .await?;

        if let Some(errcode) = resp.errcode {
            if errcode != 0 {
                return Err(WeChatError::Api {
                    errcode,
                    errmsg: resp.errmsg.unwrap_or_default(),
                });
            }
        }

        let access_token = resp.access_token.ok_or(WeChatError::TokenUnavailable)?;
        let expires_in = resp.expires_in.unwrap_or(7200);

        tracing::debug!("Access token refreshed, expires in {}s", expires_in);

        Ok(TokenState {
            access_token,
            expires_at: Instant::now() + Duration::from_secs(expires_in) - REFRESH_BUFFER,
        })
    }
}
