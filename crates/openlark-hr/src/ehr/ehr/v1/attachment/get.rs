//! 下载人员的附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/ehr-v1/attachment/get>
//!
//! 官方请求仅路径参数 `token`（无查询参数），成功响应为文件二进制流。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 下载人员的附件请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 文件 token（必填，路径参数）
    attachment_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, attachment_id: String) -> Self {
        Self {
            attachment_id,
            config,
        }
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.attachment_id, "attachment_id 不能为空");
        Ok(())
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::EhrApiV1;

        self.validate()?;

        // 1. 构建端点（官方仅路径参数 token，无查询参数）
        let api_endpoint = EhrApiV1::AttachmentGet(self.attachment_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 2. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "下载人员附件响应数据为空",
        )
        .await
    }
}

/// 下载人员的附件响应
///
/// 官方 200 响应为文件二进制流（非 JSON envelope）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetResponse {
    /// `Content-Disposition` 中的文件名；响应头缺失时为空字符串
    pub file_name: String,
    /// 文件二进制内容
    pub content: Vec<u8>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            file_name,
            content: body,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_ehr_attachment_get_request_builder() {
        let request = GetRequest::new(create_test_config(), "att_1".to_string());
        assert_eq!(request.attachment_id, "att_1");
    }

    #[test]
    fn test_ehr_attachment_get_url_build() {
        let request = GetRequest::new(create_test_config(), "att_2".to_string());
        let api_endpoint =
            crate::common::api_endpoints::EhrApiV1::AttachmentGet(request.attachment_id.clone());
        let api_request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        let url = api_request.build_url();
        assert!(url.contains("att_2"), "URL 应包含路径参数: {url}");
        // 官方无查询参数，URL 不应携带 query string
        assert!(
            !url.contains('?'),
            "官方请求无查询参数，URL 不应包含 query string: {url}"
        );
    }

    #[test]
    fn test_ehr_attachment_get_binary_decode() {
        let response = GetResponse::from_binary("resume.pdf".to_string(), b"pdf-bytes".to_vec())
            .expect("二进制响应解码失败");
        assert_eq!(response.file_name, "resume.pdf");
        assert_eq!(response.content, b"pdf-bytes".to_vec());
    }

    #[test]
    fn test_ehr_attachment_get_validation() {
        let invalid_attachment_request = GetRequest::new(create_test_config(), "  ".to_string());
        assert!(invalid_attachment_request.validate().is_err());

        let valid_request = GetRequest::new(create_test_config(), "att_1".to_string());
        assert!(valid_request.validate().is_ok());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 二进制响应 + 实际请求形状。
    #[tokio::test]
    async fn test_get_attachment_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let file_bytes = b"%PDF-1.4 fake-pdf-content".to_vec();
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/attachments/att_001"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-disposition", "attachment; filename=\"resume.pdf\"")
                    .set_body_bytes(file_bytes.clone()),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetRequest::new(config, "att_001".to_string())
            .execute()
            .await
            .expect("下载人员附件应成功");

        assert_eq!(data.file_name, "resume.pdf");
        assert_eq!(data.content, file_bytes);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/ehr/v1/attachments/att_001"
        );
        // 官方无查询参数
        assert!(
            received[0].url.query().unwrap_or("").is_empty(),
            "官方请求无查询参数: {:?}",
            received[0].url.query()
        );
    }
}
