//! 获取勋章授予名单列表 API
//!
//! API文档: <https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取勋章授予名单列表请求
pub struct ListBadgeGrantBuilder {
    badge_id: String,
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListBadgeGrantBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListBadgeGrantResponse> {
        let mut url = format!("/open-apis/admin/v1/badges/{}/grants", self.badge_id);
        let mut params = Vec::new();

        if let Some(size) = self.page_size {
            params.push(format!("page_size={}", size));
        }
        if let Some(token) = self.page_token {
            params.push(format!("page_token={}", token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let api_request: ApiRequest<ListBadgeGrantResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取勋章授予名单列表", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListBadgeGrantResponse {
    pub items: Vec<BadgeGrantItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BadgeGrantItem {
    pub grant_id: String,
    pub badge_id: String,
    pub user_id: String,
    pub create_time: String,
}

impl ApiResponseTrait for ListBadgeGrantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListBadgeGrantBuilder::new(config.clone())
            .badge_id("test".to_string())
            .page_size(1);
        let _ = request;
    }
}
