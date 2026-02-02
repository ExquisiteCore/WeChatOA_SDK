use serde::{Deserialize, Serialize};

use crate::client::WeChatClient;
use crate::error::Result;

/// Time range for statistics queries.
#[derive(Debug, Clone, Serialize)]
pub struct DateRange {
    pub begin_date: String,
    pub end_date: String,
}

impl DateRange {
    pub fn new(begin: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            begin_date: begin.into(),
            end_date: end.into(),
        }
    }
}

/// User summary data item.
#[derive(Debug, Clone, Deserialize)]
pub struct UserSummaryItem {
    pub ref_date: String,
    pub user_source: i32,
    pub new_user: i32,
    pub cancel_user: i32,
}

/// User cumulate data item.
#[derive(Debug, Clone, Deserialize)]
pub struct UserCumulateItem {
    pub ref_date: String,
    pub cumulate_user: i32,
}

/// Article summary data item.
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleSummaryItem {
    pub ref_date: String,
    pub msgid: Option<String>,
    pub title: Option<String>,
    pub int_page_read_user: Option<i32>,
    pub int_page_read_count: Option<i32>,
    pub ori_page_read_user: Option<i32>,
    pub ori_page_read_count: Option<i32>,
    pub share_user: Option<i32>,
    pub share_count: Option<i32>,
    pub add_to_fav_user: Option<i32>,
    pub add_to_fav_count: Option<i32>,
}

/// Article total data item.
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleTotalItem {
    pub ref_date: String,
    pub msgid: String,
    pub title: String,
    pub details: Vec<ArticleDetailItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArticleDetailItem {
    pub stat_date: String,
    pub target_user: i32,
    pub int_page_read_user: i32,
    pub int_page_read_count: i32,
    pub ori_page_read_user: i32,
    pub ori_page_read_count: i32,
    pub share_user: i32,
    pub share_count: i32,
    pub add_to_fav_user: i32,
    pub add_to_fav_count: i32,
}

/// Upstream message data item.
#[derive(Debug, Clone, Deserialize)]
pub struct UpstreamMsgItem {
    pub ref_date: String,
    pub msg_type: i32,
    pub msg_user: i32,
    pub msg_count: i32,
}

/// Interface summary data item.
#[derive(Debug, Clone, Deserialize)]
pub struct InterfaceSummaryItem {
    pub ref_date: String,
    pub callback_count: i32,
    pub fail_count: i32,
    pub total_time_cost: i32,
    pub max_time_cost: i32,
}

#[derive(Debug, Deserialize)]
struct ListResponse<T> {
    list: Vec<T>,
}

impl WeChatClient {
    // ========== User Analysis ==========

    /// Get user summary statistics.
    /// Date range must be within 7 days.
    pub async fn get_user_summary(&self, range: &DateRange) -> Result<Vec<UserSummaryItem>> {
        let resp: ListResponse<UserSummaryItem> =
            self.post_json("/datacube/getusersummary", range).await?;
        Ok(resp.list)
    }

    /// Get user cumulate statistics.
    /// Date range must be within 7 days.
    pub async fn get_user_cumulate(&self, range: &DateRange) -> Result<Vec<UserCumulateItem>> {
        let resp: ListResponse<UserCumulateItem> =
            self.post_json("/datacube/getusercumulate", range).await?;
        Ok(resp.list)
    }

    // ========== Article Analysis ==========

    /// Get article summary statistics (single day only).
    pub async fn get_article_summary(&self, range: &DateRange) -> Result<Vec<ArticleSummaryItem>> {
        let resp: ListResponse<ArticleSummaryItem> =
            self.post_json("/datacube/getarticlesummary", range).await?;
        Ok(resp.list)
    }

    /// Get article total statistics (single day only).
    pub async fn get_article_total(&self, range: &DateRange) -> Result<Vec<ArticleTotalItem>> {
        let resp: ListResponse<ArticleTotalItem> =
            self.post_json("/datacube/getarticletotal", range).await?;
        Ok(resp.list)
    }

    /// Get user read statistics (up to 3 days).
    pub async fn get_user_read(&self, range: &DateRange) -> Result<Vec<ArticleSummaryItem>> {
        let resp: ListResponse<ArticleSummaryItem> =
            self.post_json("/datacube/getuserread", range).await?;
        Ok(resp.list)
    }

    /// Get user read hour statistics (single day only).
    pub async fn get_user_read_hour(&self, range: &DateRange) -> Result<Vec<ArticleSummaryItem>> {
        let resp: ListResponse<ArticleSummaryItem> =
            self.post_json("/datacube/getuserreadhour", range).await?;
        Ok(resp.list)
    }

    /// Get user share statistics (up to 7 days).
    pub async fn get_user_share(&self, range: &DateRange) -> Result<Vec<ArticleSummaryItem>> {
        let resp: ListResponse<ArticleSummaryItem> =
            self.post_json("/datacube/getusershare", range).await?;
        Ok(resp.list)
    }

    /// Get user share hour statistics (single day only).
    pub async fn get_user_share_hour(&self, range: &DateRange) -> Result<Vec<ArticleSummaryItem>> {
        let resp: ListResponse<ArticleSummaryItem> =
            self.post_json("/datacube/getusersharehour", range).await?;
        Ok(resp.list)
    }

    // ========== Message Analysis ==========

    /// Get upstream message statistics (up to 7 days).
    pub async fn get_upstream_msg(&self, range: &DateRange) -> Result<Vec<UpstreamMsgItem>> {
        let resp: ListResponse<UpstreamMsgItem> =
            self.post_json("/datacube/getupstreammsg", range).await?;
        Ok(resp.list)
    }

    /// Get upstream message hour statistics (single day only).
    pub async fn get_upstream_msg_hour(&self, range: &DateRange) -> Result<Vec<UpstreamMsgItem>> {
        let resp: ListResponse<UpstreamMsgItem> = self
            .post_json("/datacube/getupstreammsghour", range)
            .await?;
        Ok(resp.list)
    }

    // ========== Interface Analysis ==========

    /// Get interface summary statistics (up to 30 days).
    pub async fn get_interface_summary(
        &self,
        range: &DateRange,
    ) -> Result<Vec<InterfaceSummaryItem>> {
        let resp: ListResponse<InterfaceSummaryItem> = self
            .post_json("/datacube/getinterfacesummary", range)
            .await?;
        Ok(resp.list)
    }

    /// Get interface summary hour statistics (single day only).
    pub async fn get_interface_summary_hour(
        &self,
        range: &DateRange,
    ) -> Result<Vec<InterfaceSummaryItem>> {
        let resp: ListResponse<InterfaceSummaryItem> = self
            .post_json("/datacube/getinterfacesummaryhour", range)
            .await?;
        Ok(resp.list)
    }
}
