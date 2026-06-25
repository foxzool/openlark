//! 查看应用基本信息 API
//! docPath: https://open.feishu.cn/document/apaas-v1/app/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查看应用基本信息请求 Builder。
pub struct ListAppBuilder {
    /// 分页大小。
    page_size: Option<u32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 配置信息。
    config: Config,
}

impl ListAppBuilder {
    /// 创建新的实例。
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

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListAppResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListAppResponse> {
        let mut url = String::from("/open-apis/apaas/v1/apps");
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

        let api_request: ApiRequest<ListAppResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("查看应用基本信息", "响应数据为空")
        })
    }
}

/// 查看应用基本信息响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAppResponse {
    /// 应用条目列表。
    pub items: Vec<AppItem>,
    /// 分页标记。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 应用条目信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppItem {
    /// 应用 ID。
    pub app_id: String,
    /// 应用名称。
    pub app_name: String,
    /// 应用命名空间。
    pub app_namespace: String,
    /// 应用描述。
    pub description: Option<String>,
}

impl ApiResponseTrait for ListAppResponse {
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
        let request = ListAppBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
