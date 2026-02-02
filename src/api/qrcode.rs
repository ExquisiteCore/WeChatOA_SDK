use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::{Result, WeChatError};
use crate::models::qrcode::{QrCodeAction, QrCodeResponse};

const WECHAT_MP_BASE: &str = "https://mp.weixin.qq.com";

impl WeChatClient {
    /// Create a QR code with a numeric scene value.
    ///
    /// - `scene_id`: Scene value (32-bit non-zero integer)
    /// - `action`: QR code type (temporary or permanent)
    /// - `expire_seconds`: Expiration time for temporary QR codes (max 2592000 = 30 days)
    pub async fn create_qrcode(
        &self,
        scene_id: u32,
        action: QrCodeAction,
        expire_seconds: Option<i64>,
    ) -> Result<QrCodeResponse> {
        let mut body = serde_json::json!({
            "action_name": action.as_str(),
            "action_info": {
                "scene": {
                    "scene_id": scene_id
                }
            }
        });

        if let Some(expire) = expire_seconds {
            body["expire_seconds"] = serde_json::json!(expire);
        }

        self.post_json("/qrcode/create", &body).await
    }

    /// Create a QR code with a string scene value.
    ///
    /// - `scene_str`: Scene value string (max 64 characters)
    /// - `action`: QR code type (use TemporaryStr or PermanentStr)
    /// - `expire_seconds`: Expiration time for temporary QR codes
    pub async fn create_qrcode_str(
        &self,
        scene_str: &str,
        action: QrCodeAction,
        expire_seconds: Option<i64>,
    ) -> Result<QrCodeResponse> {
        let mut body = serde_json::json!({
            "action_name": action.as_str(),
            "action_info": {
                "scene": {
                    "scene_str": scene_str
                }
            }
        });

        if let Some(expire) = expire_seconds {
            body["expire_seconds"] = serde_json::json!(expire);
        }

        self.post_json("/qrcode/create", &body).await
    }

    /// Get the URL to display a QR code image.
    ///
    /// The ticket should be URL-encoded when used.
    pub fn get_qrcode_url(ticket: &str) -> String {
        format!(
            "{}/cgi-bin/showqrcode?ticket={}",
            WECHAT_MP_BASE,
            urlencoding::encode(ticket)
        )
    }

    /// Download QR code image as bytes.
    pub async fn download_qrcode(&self, ticket: &str) -> Result<Vec<u8>> {
        let url = Self::get_qrcode_url(ticket);
        let resp = self.http.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(WeChatError::Api {
                errcode: resp.status().as_u16() as i64,
                errmsg: "Failed to download QR code".to_string(),
            });
        }

        Ok(resp.bytes().await?.to_vec())
    }

    /// Convert a long URL to a short URL.
    ///
    /// Note: This API may be deprecated. Consider using other URL shorteners.
    pub async fn create_short_url(&self, long_url: &str) -> Result<String> {
        let body = serde_json::json!({
            "action": "long2short",
            "long_url": long_url
        });

        #[derive(Deserialize)]
        struct Response {
            short_url: Option<String>,
            errcode: Option<i64>,
            errmsg: Option<String>,
        }

        let resp: Response = self.post_json("/shorturl", &body).await?;

        if let Some(errcode) = resp.errcode {
            if errcode != 0 {
                return Err(WeChatError::Api {
                    errcode,
                    errmsg: resp.errmsg.unwrap_or_default(),
                });
            }
        }

        resp.short_url.ok_or(WeChatError::TokenUnavailable)
    }

    /// Get the WeChat server IP list.
    ///
    /// Useful for configuring firewalls to allow WeChat callbacks.
    pub async fn get_callback_ip_list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Response {
            ip_list: Vec<String>,
        }

        let resp: Response = self.get("/getcallbackip", &[]).await?;
        Ok(resp.ip_list)
    }

    /// Get the WeChat API server IP list.
    pub async fn get_api_domain_ip_list(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct Response {
            ip_list: Vec<String>,
        }

        let resp: Response = self.get("/get_api_domain_ip", &[]).await?;
        Ok(resp.ip_list)
    }

    /// Check if the current network can access WeChat API.
    pub async fn check_network(&self) -> Result<bool> {
        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct Response {
            dns: Option<Vec<DnsInfo>>,
            ping: Option<Vec<PingInfo>>,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct DnsInfo {
            ip: String,
            real_operator: String,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct PingInfo {
            ip: String,
            from_operator: String,
            package_loss: String,
            time: String,
        }

        let body = serde_json::json!({
            "action": "all",
            "check_operator": "DEFAULT"
        });

        let _resp: Response = self.post_json("/callback/check", &body).await?;
        Ok(true)
    }
}
