use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::{Result, WeChatError};
use crate::models::material::{
    MaterialCountResponse, MaterialListResponse, MaterialType, UploadPermanentMediaResponse,
    UploadTempMediaResponse, VideoDescription,
};

const WECHAT_API_BASE: &str = "https://api.weixin.qq.com/cgi-bin";

#[derive(Deserialize)]
struct ApiResponse {
    errcode: Option<i64>,
    errmsg: Option<String>,
}

impl WeChatClient {
    /// Upload a temporary media file.
    ///
    /// Temporary media is valid for 3 days.
    pub async fn upload_temp_media(
        &self,
        media_type: MaterialType,
        file_name: &str,
        file_data: Vec<u8>,
    ) -> Result<UploadTempMediaResponse> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/media/upload?access_token={}&type={}",
            WECHAT_API_BASE,
            token,
            media_type.as_str()
        );

        let part = Part::bytes(file_data).file_name(file_name.to_string());
        let form = Form::new().part("media", part);

        let resp = self.http.post(&url).multipart(form).send().await?;
        let text = resp.text().await?;

        // Check for error
        if let Ok(err) = serde_json::from_str::<ApiResponse>(&text) {
            if let Some(errcode) = err.errcode {
                if errcode != 0 {
                    return Err(WeChatError::Api {
                        errcode,
                        errmsg: err.errmsg.unwrap_or_default(),
                    });
                }
            }
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Get a temporary media file.
    ///
    /// Returns the raw bytes of the media file.
    pub async fn get_temp_media(&self, media_id: &str) -> Result<Vec<u8>> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/media/get?access_token={}&media_id={}",
            WECHAT_API_BASE, token, media_id
        );

        let resp = self.http.get(&url).send().await?;

        // Check if response is an error JSON
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let bytes = resp.bytes().await?.to_vec();

        // If it's JSON, check for error response
        if content_type.contains("application/json") || content_type.contains("text/plain") {
            if let Ok(text) = std::str::from_utf8(&bytes) {
                if let Ok(err) = serde_json::from_str::<ApiResponse>(text) {
                    if let Some(errcode) = err.errcode {
                        if errcode != 0 {
                            return Err(WeChatError::Api {
                                errcode,
                                errmsg: err.errmsg.unwrap_or_default(),
                            });
                        }
                    }
                }
            }
        }

        Ok(bytes)
    }

    /// Upload a permanent media file (image, voice, thumb).
    pub async fn upload_permanent_media(
        &self,
        media_type: MaterialType,
        file_name: &str,
        file_data: Vec<u8>,
    ) -> Result<UploadPermanentMediaResponse> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/material/add_material?access_token={}&type={}",
            WECHAT_API_BASE,
            token,
            media_type.as_str()
        );

        let part = Part::bytes(file_data).file_name(file_name.to_string());
        let form = Form::new().part("media", part);

        let resp = self.http.post(&url).multipart(form).send().await?;
        let text = resp.text().await?;

        if let Ok(err) = serde_json::from_str::<ApiResponse>(&text) {
            if let Some(errcode) = err.errcode {
                if errcode != 0 {
                    return Err(WeChatError::Api {
                        errcode,
                        errmsg: err.errmsg.unwrap_or_default(),
                    });
                }
            }
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Upload a permanent video file.
    pub async fn upload_permanent_video(
        &self,
        file_name: &str,
        file_data: Vec<u8>,
        description: &VideoDescription,
    ) -> Result<UploadPermanentMediaResponse> {
        let token = self.access_token().await?;
        let url = format!(
            "{}/material/add_material?access_token={}&type=video",
            WECHAT_API_BASE, token
        );

        let part = Part::bytes(file_data).file_name(file_name.to_string());
        let desc_json = serde_json::to_string(description)?;

        let form = Form::new()
            .part("media", part)
            .text("description", desc_json);

        let resp = self.http.post(&url).multipart(form).send().await?;
        let text = resp.text().await?;

        if let Ok(err) = serde_json::from_str::<ApiResponse>(&text) {
            if let Some(errcode) = err.errcode {
                if errcode != 0 {
                    return Err(WeChatError::Api {
                        errcode,
                        errmsg: err.errmsg.unwrap_or_default(),
                    });
                }
            }
        }

        Ok(serde_json::from_str(&text)?)
    }

    /// Get permanent material count.
    pub async fn get_material_count(&self) -> Result<MaterialCountResponse> {
        self.get("/material/get_materialcount", &[]).await
    }

    /// Get permanent material list.
    pub async fn get_material_list(
        &self,
        media_type: MaterialType,
        offset: i32,
        count: i32,
    ) -> Result<MaterialListResponse> {
        let body = serde_json::json!({
            "type": media_type.as_str(),
            "offset": offset,
            "count": count
        });
        self.post_json("/material/batchget_material", &body).await
    }

    /// Delete a permanent material.
    pub async fn delete_material(&self, media_id: &str) -> Result<()> {
        let body = serde_json::json!({
            "media_id": media_id
        });
        let _: ApiResponse = self.post_json("/material/del_material", &body).await?;
        Ok(())
    }
}
