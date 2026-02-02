use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::Result;
use crate::models::template::{
    GetIndustryResponse, SendTemplateResponse, TemplateListResponse, TemplateMessage,
};

#[derive(Deserialize)]
#[allow(dead_code)]
struct AddTemplateResponse {
    errcode: i64,
    errmsg: String,
    template_id: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct DeleteResponse {
    errcode: i64,
    errmsg: String,
}

impl WeChatClient {
    /// Send a template message.
    pub async fn send_template_message(&self, message: &TemplateMessage) -> Result<i64> {
        let resp: SendTemplateResponse = self.post_json("/message/template/send", message).await?;
        Ok(resp.msgid.unwrap_or(0))
    }

    /// Set the industry for template messages.
    pub async fn set_industry(&self, industry_id1: &str, industry_id2: &str) -> Result<()> {
        let body = serde_json::json!({
            "industry_id1": industry_id1,
            "industry_id2": industry_id2
        });
        let _: DeleteResponse = self.post_json("/template/api_set_industry", &body).await?;
        Ok(())
    }

    /// Get the current industry setting.
    pub async fn get_industry(&self) -> Result<GetIndustryResponse> {
        self.get("/template/get_industry", &[]).await
    }

    /// Add a template from the template library.
    pub async fn add_template(&self, template_id_short: &str) -> Result<String> {
        let body = serde_json::json!({
            "template_id_short": template_id_short
        });
        let resp: AddTemplateResponse = self.post_json("/template/api_add_template", &body).await?;
        Ok(resp.template_id.unwrap_or_default())
    }

    /// Get all templates.
    pub async fn get_all_templates(&self) -> Result<TemplateListResponse> {
        self.get("/template/get_all_private_template", &[]).await
    }

    /// Delete a template.
    pub async fn delete_template(&self, template_id: &str) -> Result<()> {
        let body = serde_json::json!({
            "template_id": template_id
        });
        let _: DeleteResponse = self.post_json("/template/del_private_template", &body).await?;
        Ok(())
    }
}
