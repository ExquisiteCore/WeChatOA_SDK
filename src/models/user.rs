use serde::{Deserialize, Serialize};

/// User information returned by WeChat API.
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    /// Whether user has subscribed (0 = no, 1 = yes)
    pub subscribe: i32,
    /// User's OpenID
    pub openid: String,
    /// User's language setting
    pub language: Option<String>,
    /// Subscribe timestamp
    pub subscribe_time: Option<i64>,
    /// User's UnionID (if bound to open platform)
    pub unionid: Option<String>,
    /// Remark set by the official account
    pub remark: Option<String>,
    /// Group ID the user belongs to
    pub groupid: Option<i32>,
    /// Tag IDs the user has
    pub tagid_list: Option<Vec<i32>>,
    /// Subscribe scene
    pub subscribe_scene: Option<String>,
    /// QR scene value
    pub qr_scene: Option<i64>,
    /// QR scene string
    pub qr_scene_str: Option<String>,
}

/// Response from batch get user info.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchUserInfoResponse {
    pub user_info_list: Vec<UserInfo>,
}

/// User list response.
#[derive(Debug, Clone, Deserialize)]
pub struct UserListResponse {
    pub total: i32,
    pub count: i32,
    pub data: Option<UserOpenIdList>,
    pub next_openid: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOpenIdList {
    pub openid: Vec<String>,
}

/// Tag information.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
    pub count: Option<i32>,
}

/// Response from getting tags.
#[derive(Debug, Clone, Deserialize)]
pub struct TagListResponse {
    pub tags: Vec<Tag>,
}

/// Response from creating a tag.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTagResponse {
    pub tag: Tag,
}

/// Request to get users by tag.
#[derive(Debug, Clone, Serialize)]
pub struct GetUsersByTagRequest {
    pub tagid: i32,
    pub next_openid: Option<String>,
}

/// Response from getting users by tag.
#[derive(Debug, Clone, Deserialize)]
pub struct UsersByTagResponse {
    pub count: i32,
    pub data: Option<UserOpenIdList>,
    pub next_openid: Option<String>,
}
