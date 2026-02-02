use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::Result;
use crate::models::user::{
    BatchUserInfoResponse, CreateTagResponse, Tag, TagListResponse, UserInfo, UserListResponse,
    UsersByTagResponse,
};

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiResponse {
    errcode: Option<i64>,
    errmsg: Option<String>,
}

impl WeChatClient {
    /// Get user information by OpenID.
    pub async fn get_user_info(&self, openid: &str, lang: Option<&str>) -> Result<UserInfo> {
        let lang = lang.unwrap_or("zh_CN");
        self.get("/user/info", &[("openid", openid), ("lang", lang)])
            .await
    }

    /// Batch get user information.
    pub async fn batch_get_user_info(
        &self,
        openids: &[&str],
        lang: Option<&str>,
    ) -> Result<BatchUserInfoResponse> {
        let lang = lang.unwrap_or("zh_CN");
        let user_list: Vec<_> = openids
            .iter()
            .map(|openid| {
                serde_json::json!({
                    "openid": openid,
                    "lang": lang
                })
            })
            .collect();

        let body = serde_json::json!({
            "user_list": user_list
        });

        self.post_json("/user/info/batchget", &body).await
    }

    /// Get user list (followers).
    pub async fn get_user_list(&self, next_openid: Option<&str>) -> Result<UserListResponse> {
        let params: Vec<(&str, &str)> = match next_openid {
            Some(openid) => vec![("next_openid", openid)],
            None => vec![],
        };
        self.get("/user/get", &params).await
    }

    /// Set user remark.
    pub async fn set_user_remark(&self, openid: &str, remark: &str) -> Result<()> {
        let body = serde_json::json!({
            "openid": openid,
            "remark": remark
        });
        let _: ApiResponse = self.post_json("/user/info/updateremark", &body).await?;
        Ok(())
    }

    // ========== Tag Management ==========

    /// Create a user tag.
    pub async fn create_tag(&self, name: &str) -> Result<Tag> {
        let body = serde_json::json!({
            "tag": { "name": name }
        });
        let resp: CreateTagResponse = self.post_json("/tags/create", &body).await?;
        Ok(resp.tag)
    }

    /// Get all tags.
    pub async fn get_tags(&self) -> Result<TagListResponse> {
        self.get("/tags/get", &[]).await
    }

    /// Update a tag name.
    pub async fn update_tag(&self, tag_id: i32, name: &str) -> Result<()> {
        let body = serde_json::json!({
            "tag": {
                "id": tag_id,
                "name": name
            }
        });
        let _: ApiResponse = self.post_json("/tags/update", &body).await?;
        Ok(())
    }

    /// Delete a tag.
    pub async fn delete_tag(&self, tag_id: i32) -> Result<()> {
        let body = serde_json::json!({
            "tag": { "id": tag_id }
        });
        let _: ApiResponse = self.post_json("/tags/delete", &body).await?;
        Ok(())
    }

    /// Get users with a specific tag.
    pub async fn get_users_by_tag(
        &self,
        tag_id: i32,
        next_openid: Option<&str>,
    ) -> Result<UsersByTagResponse> {
        let body = serde_json::json!({
            "tagid": tag_id,
            "next_openid": next_openid.unwrap_or("")
        });
        self.post_json("/user/tag/get", &body).await
    }

    /// Batch tag users.
    pub async fn batch_tag_users(&self, openids: &[&str], tag_id: i32) -> Result<()> {
        let body = serde_json::json!({
            "openid_list": openids,
            "tagid": tag_id
        });
        let _: ApiResponse = self.post_json("/tags/members/batchtagging", &body).await?;
        Ok(())
    }

    /// Batch untag users.
    pub async fn batch_untag_users(&self, openids: &[&str], tag_id: i32) -> Result<()> {
        let body = serde_json::json!({
            "openid_list": openids,
            "tagid": tag_id
        });
        let _: ApiResponse = self.post_json("/tags/members/batchuntagging", &body).await?;
        Ok(())
    }

    /// Get tags of a user.
    pub async fn get_user_tags(&self, openid: &str) -> Result<Vec<i32>> {
        let body = serde_json::json!({
            "openid": openid
        });

        #[derive(Deserialize)]
        struct Response {
            tagid_list: Vec<i32>,
        }

        let resp: Response = self.post_json("/tags/getidlist", &body).await?;
        Ok(resp.tagid_list)
    }
}
