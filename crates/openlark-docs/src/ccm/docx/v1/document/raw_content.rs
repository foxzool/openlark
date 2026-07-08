/// 获取文档纯文本内容
///
/// 获取文档的纯文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content>
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 获取文档纯文本内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentParams {
    /// 文档ID
    pub document_id: String,
    /// 语言（可选）
    pub lang: Option<i32>,
}

/// 获取文档纯文本内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentRawContentResponse {
    /// 文档内容
    pub content: String,
}

impl ApiResponseTrait for GetDocumentRawContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档纯文本内容请求
pub struct GetDocumentRawContentRequest {
    config: Config,
}

impl GetDocumentRawContentRequest {
    /// 创建获取文档纯文本内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content
    pub async fn execute(
        self,
        params: GetDocumentRawContentParams,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/raw_content
    pub async fn execute_with_options(
        self,
        params: GetDocumentRawContentParams,
        option: RequestOption,
    ) -> SDKResult<GetDocumentRawContentResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::DocumentRawContent(params.document_id.clone());

        // 创建API请求
        let mut api_request: ApiRequest<GetDocumentRawContentResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(lang) = params.lang {
            api_request = api_request.query("lang", &lang.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文档纯文本内容")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../documents/{document_id}/raw_content → GetDocumentRawContentResponse（content）。
    #[tokio::test]
    async fn test_get_document_raw_content_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/docx/v1/documents/doc1/raw_content"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "content": "文档纯文本内容" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetDocumentRawContentRequest::new(config)
            .execute(GetDocumentRawContentParams {
                document_id: "doc1".into(),
                lang: None,
            })
            .await
            .expect("获取文档纯文本应成功");
        assert_eq!(resp.content, "文档纯文本内容");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/raw_content"
        );
    }
}
