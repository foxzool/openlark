//! 获取知识库图片
//!
//! 获取知识库的图片。
//!
//! 官方文档响应为「返回文件二进制流」（无 JSON 字段表），按二进制下载处理，
//! 返回原始文件字节。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/faq-management/faq/faq_image>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

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

    /// 执行获取知识库图片请求（返回图片二进制内容）
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求（返回图片二进制内容）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Vec<u8>> {
        validate_required!(self.id, "id 不能为空");
        validate_required!(self.image_key, "image_key 不能为空");
        let api_endpoint = HelpdeskApiV1::FaqImage(self.id, self.image_key);
        let request = ApiRequest::<Vec<u8>>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, Some(option), "获取知识库图片").await
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

    /// 执行请求（返回图片二进制内容）
    pub async fn execute(&self) -> SDKResult<Vec<u8>> {
        validate_required!(self.id, "id 不能为空");
        validate_required!(self.image_key, "image_key 不能为空");
        let api_endpoint = HelpdeskApiV1::FaqImage(self.id.clone(), self.image_key.clone());
        let request = ApiRequest::<Vec<u8>>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, None, "获取知识库图片").await
    }
}

/// 执行获取知识库图片（返回图片二进制内容）
pub async fn get_faq_image(config: &Config, id: String, image_key: String) -> SDKResult<Vec<u8>> {
    validate_required!(id, "id 不能为空");
    validate_required!(image_key, "image_key 不能为空");
    let api_endpoint = HelpdeskApiV1::FaqImage(id, image_key);
    let request = ApiRequest::<Vec<u8>>::get(api_endpoint.to_url());

    Transport::request_typed(request, config, None, "获取知识库图片").await
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

    /// 端到端：GET .../faqs/{id}/image/{image_key} → 返回原始文件二进制流。
    #[tokio::test]
    async fn test_get_faq_image_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let image_bytes = b"PNG_FAKE_BYTES".to_vec();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/helpdesk/v1/faqs/faq_001/image/img_key_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(image_bytes.clone()))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let bytes =
            GetFaqImageRequest::new(config, "faq_001".to_string(), "img_key_001".to_string())
                .execute()
                .await
                .expect("获取知识库图片应成功");
        assert_eq!(bytes, image_bytes);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/faqs/faq_001/image/img_key_001"
        );
    }
}
