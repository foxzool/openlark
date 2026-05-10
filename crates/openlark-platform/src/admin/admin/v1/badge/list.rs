//! 获取勋章列表 API
//!
//! API文档: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/list
//! docPath: https://open.feishu.cn/document/server-docs/admin-v1/badge/badge/list

use crate::common::api_endpoints::AdminApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取勋章列表请求
pub struct ListBadgeBuilder {
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListBadgeBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
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
    pub async fn execute(self) -> SDKResult<ListBadgeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListBadgeResponse> {
        let mut url = AdminApiV1::ListBadge.path().to_string();
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

        let api_request: ApiRequest<ListBadgeResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("获取勋章列表", "响应数据为空"))
    }
}

/// 获取勋章列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
/// 获取勋章列表的响应。
pub struct ListBadgeResponse {
    /// 结果条目列表。
    pub items: Vec<BadgeItem>,
    /// 下一页分页游标。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// BadgeItem。
pub struct BadgeItem {
    /// 勋章 ID。
    pub badge_id: String,
    /// 名称。
    pub name: String,
    /// 描述。
    pub description: Option<String>,
    /// 图标地址。
    pub icon_url: Option<String>,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for ListBadgeResponse {
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
        let request = ListBadgeBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
