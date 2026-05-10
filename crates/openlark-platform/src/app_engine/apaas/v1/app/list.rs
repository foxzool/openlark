//! 查看应用基本信息 API
//! docPath: https://open.feishu.cn/document/apaas-v1/app/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct ListAppBuilder {
    page_size: Option<u32>,
    page_token: Option<String>,
    config: Config,
}

impl ListAppBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListAppResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAppResponse {
    pub items: Vec<AppItem>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppItem {
    pub app_id: String,
    pub app_name: String,
    pub app_namespace: String,
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
        let config = openlark_core::config::Config::builder().app_id("test_app").app_secret("test_secret").build();
        let request = ListAppBuilder::new(config.clone())
            .page_size(1)
            .page_token("test".to_string());
        let _ = request;
    }
}
