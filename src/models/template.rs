use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Template message data item.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateDataItem {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl TemplateDataItem {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            color: None,
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Mini program link for template message.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateMiniProgram {
    pub appid: String,
    pub pagepath: Option<String>,
}

/// Template message request.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateMessage {
    /// Receiver's OpenID
    pub touser: String,
    /// Template ID
    pub template_id: String,
    /// Click URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Mini program link (optional, takes priority over url)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miniprogram: Option<TemplateMiniProgram>,
    /// Template data
    pub data: HashMap<String, TemplateDataItem>,
    /// Client message ID (for deduplication)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_msg_id: Option<String>,
}

impl TemplateMessage {
    pub fn new(
        to_user: impl Into<String>,
        template_id: impl Into<String>,
        data: HashMap<String, TemplateDataItem>,
    ) -> Self {
        Self {
            touser: to_user.into(),
            template_id: template_id.into(),
            url: None,
            miniprogram: None,
            data,
            client_msg_id: None,
        }
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_miniprogram(mut self, appid: impl Into<String>, pagepath: Option<String>) -> Self {
        self.miniprogram = Some(TemplateMiniProgram {
            appid: appid.into(),
            pagepath,
        });
        self
    }

    pub fn with_client_msg_id(mut self, id: impl Into<String>) -> Self {
        self.client_msg_id = Some(id.into());
        self
    }
}

/// Response from sending template message.
#[derive(Debug, Clone, Deserialize)]
pub struct SendTemplateResponse {
    pub errcode: i64,
    pub errmsg: String,
    pub msgid: Option<i64>,
}

/// Template item from template list.
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateItem {
    pub template_id: String,
    pub title: String,
    pub primary_industry: String,
    pub deputy_industry: String,
    pub content: String,
    pub example: String,
}

/// Response from getting template list.
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateListResponse {
    pub template_list: Vec<TemplateItem>,
}

/// Industry information.
#[derive(Debug, Clone, Deserialize)]
pub struct Industry {
    pub first_class: String,
    pub second_class: String,
}

/// Response from getting industry.
#[derive(Debug, Clone, Deserialize)]
pub struct GetIndustryResponse {
    pub primary_industry: Industry,
    pub secondary_industry: Industry,
}
