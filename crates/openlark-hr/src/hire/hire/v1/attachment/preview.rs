//! 获取附件 PDF 格式下载链接
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/attachment/preview>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取附件 PDF 格式下载链接请求
#[derive(Debug, Clone)]
pub struct PreviewRequest {
    /// 配置信息
    config: Config,
    attachment_id: String,
}

impl PreviewRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            attachment_id: String::new(),
        }
    }

    /// 设置 `attachment_id`。
    pub fn attachment_id(mut self, attachment_id: String) -> Self {
        self.attachment_id = attachment_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PreviewResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PreviewResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.attachment_id.trim(), "附件 ID 不能为空");

        let api_endpoint = HireApiV1::AttachmentPreview(self.attachment_id);
        let request = ApiRequest::<PreviewResponse>::get(api_endpoint.to_url());

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取附件 PDF 格式下载链接响应数据为空",
        )
        .await
    }
}

/// 获取附件 PDF 格式下载链接响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PreviewResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `url` 字段。
    pub url: Option<String>,
}

impl ApiResponseTrait for PreviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/hire/v1/attachments/test001/preview
    #[tokio::test]
    async fn test_preview_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/attachments/test001/preview"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        PreviewRequest::new(config)
            .attachment_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
