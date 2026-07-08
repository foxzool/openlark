/// 批量更新块的内容
///
/// 批量更新块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/batch_update
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/batch_update>
use crate::ccm::docx::models::block_update::BlockUpdateOperation;
use crate::ccm::docx::models::common_types::DocxBlock;
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

/// 批量更新块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocumentBlocksParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 批量请求
    pub requests: Vec<BatchUpdateRequest>,
}

/// 单个批量更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateRequest {
    /// 块 ID。
    pub block_id: String,
    /// 操作内容（update_text_elements / merge_table_cells 等 15 种之一）
    #[serde(flatten)]
    pub operation: BlockUpdateOperation,
}

/// 批量更新块内容响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocumentBlocksResponse {
    /// 更新后的块列表。
    #[serde(default)]
    pub blocks: Vec<DocxBlock>,
    /// 文档版本号（操作后的文档版本）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_revision_id: Option<i32>,
    /// 幂等标记（请求时传入的 client_token 原样回传）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ApiResponseTrait for BatchUpdateDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新块内容请求
///
/// 用于批量修改文档中的多个块内容。
pub struct BatchUpdateDocumentBlocksRequest {
    config: Config,
}

impl BatchUpdateDocumentBlocksRequest {
    /// 创建新的批量更新块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: BatchUpdateDocumentBlocksParams,
    ) -> SDKResult<BatchUpdateDocumentBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: BatchUpdateDocumentBlocksParams,
        option: RequestOption,
    ) -> SDKResult<BatchUpdateDocumentBlocksResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.requests, "批量请求不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockBatchUpdate(params.document_id.clone());
        let mut api_request: ApiRequest<BatchUpdateDocumentBlocksResponse> =
            ApiRequest::patch(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量更新块的内容")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH .../blocks/batch_update → BatchUpdateDocumentBlocksResponse（blocks）。
    #[tokio::test]
    async fn test_batch_update_document_blocks_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/docx/v1/documents/doc1/blocks/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "blocks": [], "document_revision_id": 2 }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = BatchUpdateDocumentBlocksRequest::new(config)
            .execute(BatchUpdateDocumentBlocksParams {
                document_id: "doc1".into(),
                requests: vec![BatchUpdateRequest {
                    block_id: "blk1".into(),
                    operation: BlockUpdateOperation::Raw(json!({})),
                }],
            })
            .await
            .expect("批量更新块应成功");
        assert!(resp.blocks.is_empty());
        assert_eq!(resp.document_revision_id, Some(2));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/blocks/batch_update"
        );
    }
}
