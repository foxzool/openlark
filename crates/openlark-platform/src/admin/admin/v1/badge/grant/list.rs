//! 获取勋章授予名单列表 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge-grant/list
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
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
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            badge_id: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置勋章 ID。
    pub fn badge_id(mut self, badge_id: impl Into<String>) -> Self {
        self.badge_id = badge_id.into();
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页游标。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListBadgeGrantResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListBadgeGrantResponse> {
        let mut url = format!("/open-apis/admin/v1/badges/{}/grants", self.badge_id);
        let mut params = Vec::new();

        if let Some(size) = self.page_size {
            params.push(format!("page_size={size}"));
        }
        if let Some(token) = self.page_token {
            params.push(format!("page_token={token}"));
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
/// 获取勋章授予名单列表的响应。
pub struct ListBadgeGrantResponse {
    /// 结果条目列表。
    pub items: Vec<BadgeGrantItem>,
    /// 下一页分页游标。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// BadgeGrantItem。
pub struct BadgeGrantItem {
    /// 授予记录 ID。
    pub grant_id: String,
    /// 勋章 ID。
    pub badge_id: String,
    /// 用户 ID。
    pub user_id: String,
    /// 创建时间。
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
