//! 获取知识库图片
//!
//! 获取知识库的图片。
//!
//! docPath: https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/faq_image

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取知识库图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFaqImageResponse {
    /// 图片Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_key: Option<String>,
    /// 图片URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ApiResponseTrait for GetFaqImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识库图片请求
#[derive(Debug, Clone)]
pub struct GetFaqImageRequest {
    config: Arc<Config>,
    id: String,
    image_key: String,
}

impl GetFaqImageRequest {
    /// 创建新的获取知识库图片请求
    pub fn new(config: Arc<Config>, id: String, image_key: String) -> Self {
        Self {
            config,
            id,
            image_key,
        }
    }

    /// 执行获取知识库图片请求
    pub async fn execute(self) -> SDKResult<GetFaqImageResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetFaqImageResponse> {
        let api_endpoint = HelpdeskApiV1::FaqImage(self.id.clone(), self.image_key.clone());
        let request = ApiRequest::<GetFaqImageResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取知识库图片")
    }
}

/// 获取知识库图片请求构建器
#[derive(Debug, Clone)]
pub struct GetFaqImageRequestBuilder {
    config: Arc<Config>,
    id: String,
    image_key: String,
}

impl GetFaqImageRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, id: String, image_key: String) -> Self {
        Self {
            config,
            id,
            image_key,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetFaqImageResponse> {
        let api_endpoint = HelpdeskApiV1::FaqImage(self.id.clone(), self.image_key.clone());
        let request = ApiRequest::<GetFaqImageResponse>::get(api_endpoint.to_url());

        let response = Transport::request(request, &self.config, None).await?;
        extract_response_data(response, "获取知识库图片")
    }
}

/// 执行获取知识库图片
pub async fn get_faq_image(
    config: &Config,
    id: String,
    image_key: String,
) -> SDKResult<GetFaqImageResponse> {
    let api_endpoint = HelpdeskApiV1::FaqImage(id, image_key);
    let request = ApiRequest::<GetFaqImageResponse>::get(api_endpoint.to_url());

    let response = Transport::request(request, config, None).await?;
    extract_response_data(response, "获取知识库图片")
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = GetFaqImageRequestBuilder::new(
            Arc::new(config),
            "faq_123".to_string(),
            "image_key_456".to_string(),
        );

        assert_eq!(builder.id, "faq_123");
        assert_eq!(builder.image_key, "image_key_456");
    }
}
