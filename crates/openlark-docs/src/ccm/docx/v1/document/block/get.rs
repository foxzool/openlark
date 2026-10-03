/// 获取块的内容
///
/// 指定块的 block id 获取指定块的富文本内容数据。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::api_endpoints::DocxApiV1;

/// 获取块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockParams {
    /// 文档ID
    pub document_id: String,
    /// 块ID
    pub block_id: String,
    /// 文档版本号（可选，-1 表示最新版本）
    pub document_revision_id: Option<i64>,
}

/// 获取块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockResponse {
    /// 块内容。
    pub block: DocxBlock,
}

impl ApiResponseTrait for GetDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块内容请求
///
/// 用于根据块 ID 获取单个文档块。
pub struct GetDocumentBlockRequest {
    config: Config,
}

impl GetDocumentBlockRequest {
    /// 创建新的获取块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: GetDocumentBlockParams,
    ) -> SDKResult<GetDocumentBlockResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: GetDocumentBlockParams,
        option: RequestOption,
    ) -> SDKResult<GetDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::DocumentBlockGet(params.document_id.clone(), params.block_id.clone());
        let mut api_request: ApiRequest<GetDocumentBlockResponse> = api_endpoint.to_request();

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        Transport::request_typed(api_request, &self.config, Some(option), "获取块的内容").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../blocks/{block_id} → GetDocumentBlockResponse（block）。
    #[tokio::test]
    async fn test_get_document_block_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/docx/v1/documents/doc1/blocks/blk1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "block": { "block_id": "blk1", "block_type": 1 } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetDocumentBlockRequest::new(config)
            .execute(GetDocumentBlockParams {
                document_id: "doc1".into(),
                block_id: "blk1".into(),
                document_revision_id: None,
            })
            .await
            .expect("获取块内容应成功");
        assert_eq!(resp.block.block_id, "blk1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/blocks/blk1"
        );
    }
}
