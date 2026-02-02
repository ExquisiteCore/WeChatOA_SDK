use serde::{Deserialize, Serialize};

/// Menu button types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MenuButtonType {
    Click,
    View,
    #[serde(rename = "scancode_push")]
    ScancodePush,
    #[serde(rename = "scancode_waitmsg")]
    ScancodeWaitmsg,
    #[serde(rename = "pic_sysphoto")]
    PicSysphoto,
    #[serde(rename = "pic_photo_or_album")]
    PicPhotoOrAlbum,
    #[serde(rename = "pic_weixin")]
    PicWeixin,
    #[serde(rename = "location_select")]
    LocationSelect,
    #[serde(rename = "media_id")]
    MediaId,
    #[serde(rename = "article_id")]
    ArticleId,
    #[serde(rename = "article_view_limited")]
    ArticleViewLimited,
    #[serde(rename = "miniprogram")]
    MiniProgram,
}

/// A menu button (can have sub-buttons).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuButton {
    /// Button name (max 16 bytes for main, 60 bytes for sub)
    pub name: String,
    /// Button type (required for leaf buttons)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub button_type: Option<String>,
    /// Key for click type buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// URL for view type buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Media ID for media_id type buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_id: Option<String>,
    /// Article ID for article_id type buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article_id: Option<String>,
    /// Mini program app ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// Mini program page path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagepath: Option<String>,
    /// Sub-buttons (max 5)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_button: Option<Vec<MenuButton>>,
}

impl MenuButton {
    /// Create a click type button.
    pub fn click(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            button_type: Some("click".to_string()),
            key: Some(key.into()),
            url: None,
            media_id: None,
            article_id: None,
            appid: None,
            pagepath: None,
            sub_button: None,
        }
    }

    /// Create a view (URL) type button.
    pub fn view(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            button_type: Some("view".to_string()),
            key: None,
            url: Some(url.into()),
            media_id: None,
            article_id: None,
            appid: None,
            pagepath: None,
            sub_button: None,
        }
    }

    /// Create a parent button with sub-buttons.
    pub fn parent(name: impl Into<String>, sub_buttons: Vec<MenuButton>) -> Self {
        Self {
            name: name.into(),
            button_type: None,
            key: None,
            url: None,
            media_id: None,
            article_id: None,
            appid: None,
            pagepath: None,
            sub_button: Some(sub_buttons),
        }
    }

    /// Create a mini program button.
    pub fn miniprogram(
        name: impl Into<String>,
        url: impl Into<String>,
        appid: impl Into<String>,
        pagepath: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            button_type: Some("miniprogram".to_string()),
            key: None,
            url: Some(url.into()),
            media_id: None,
            article_id: None,
            appid: Some(appid.into()),
            pagepath: Some(pagepath.into()),
            sub_button: None,
        }
    }
}

/// Menu structure for creating menus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    pub button: Vec<MenuButton>,
}

/// Response from getting current menu.
#[derive(Debug, Clone, Deserialize)]
pub struct GetMenuResponse {
    pub menu: Option<Menu>,
    pub conditionalmenu: Option<Vec<ConditionalMenu>>,
}

/// Conditional menu with match rules.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConditionalMenu {
    pub button: Vec<MenuButton>,
    pub matchrule: MatchRule,
    pub menuid: Option<i64>,
}

/// Match rules for conditional menus.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MatchRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_platform_type: Option<String>,
}
