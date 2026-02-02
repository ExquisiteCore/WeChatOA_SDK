use serde::Deserialize;

use crate::client::WeChatClient;
use crate::error::Result;
use crate::models::menu::{ConditionalMenu, GetMenuResponse, MatchRule, Menu, MenuButton};

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApiResponse {
    errcode: Option<i64>,
    errmsg: Option<String>,
}

#[derive(Deserialize)]
struct CreateConditionalMenuResponse {
    menuid: Option<i64>,
}

impl WeChatClient {
    /// Create a custom menu.
    pub async fn create_menu(&self, buttons: Vec<MenuButton>) -> Result<()> {
        let menu = Menu { button: buttons };
        let _: ApiResponse = self.post_json("/menu/create", &menu).await?;
        Ok(())
    }

    /// Get the current menu configuration.
    pub async fn get_menu(&self) -> Result<GetMenuResponse> {
        self.get("/get_current_selfmenu_info", &[]).await
    }

    /// Delete all menus (including conditional menus).
    pub async fn delete_menu(&self) -> Result<()> {
        let _: ApiResponse = self.get("/menu/delete", &[]).await?;
        Ok(())
    }

    /// Create a conditional menu (personalized menu).
    pub async fn create_conditional_menu(
        &self,
        buttons: Vec<MenuButton>,
        match_rule: MatchRule,
    ) -> Result<i64> {
        let menu = ConditionalMenu {
            button: buttons,
            matchrule: match_rule,
            menuid: None,
        };
        let resp: CreateConditionalMenuResponse =
            self.post_json("/menu/addconditional", &menu).await?;
        Ok(resp.menuid.unwrap_or(0))
    }

    /// Delete a conditional menu by menu ID.
    pub async fn delete_conditional_menu(&self, menu_id: i64) -> Result<()> {
        let body = serde_json::json!({
            "menuid": menu_id
        });
        let _: ApiResponse = self.post_json("/menu/delconditional", &body).await?;
        Ok(())
    }

    /// Test which menu a user would see.
    pub async fn try_match_menu(&self, user_id: &str) -> Result<Menu> {
        let body = serde_json::json!({
            "user_id": user_id
        });
        self.post_json("/menu/trymatch", &body).await
    }
}
