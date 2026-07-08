/// 删除块
///
/// 指定需要操作的块，删除其指定范围的子块。如果操作成功，接口将返回应用删除操作后的文档版本号。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 删除块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteDocumentBlockChildrenParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 父块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 文档版本号（可选，-1 表示最新版本）
    #[serde(skip_serializing)]
    pub document_revision_id: Option<i64>,
    /// 删除的起始索引（左闭右开）
    pub start_index: i32,
    /// 删除的末尾索引（左闭右开）
    pub end_index: i32,
}

/// 删除块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteDocumentBlockChildrenResponse {
    /// 文档修订版本号。
    pub document_revision_id: i64,
    /// 客户端幂等 token。
    pub client_token: String,
}

impl ApiResponseTrait for BatchDeleteDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除块请求
///
/// 用于删除文档块下指定范围的子块。
pub struct BatchDeleteDocumentBlockChildrenRequest {
    config: Config,
}

impl BatchDeleteDocumentBlockChildrenRequest {
    /// 创建新的删除子块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: BatchDeleteDocumentBlockChildrenParams,
    ) -> SDKResult<BatchDeleteDocumentBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: BatchDeleteDocumentBlockChildrenParams,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockChildrenBatchDelete(
            params.document_id.clone(),
            params.block_id.clone(),
        );

        let mut api_request: ApiRequest<BatchDeleteDocumentBlockChildrenResponse> =
            ApiRequest::delete(&api_endpoint.to_url()).body(serialize_params(&params, "删除块")?);

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除块")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../blocks/{block_id}/children/batch_delete → BatchDeleteDocumentBlockChildrenResponse（document_revision_id）。
    #[tokio::test]
    async fn test_batch_delete_document_block_children_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/docx/v1/documents/doc1/blocks/blk1/children/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "document_revision_id": 2, "client_token": "ct001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = BatchDeleteDocumentBlockChildrenRequest::new(config)
            .execute(BatchDeleteDocumentBlockChildrenParams {
                document_id: "doc1".into(),
                block_id: "blk1".into(),
                document_revision_id: None,
                start_index: 0,
                end_index: 1,
            })
            .await
            .expect("删除子块应成功");
        assert_eq!(resp.document_revision_id, 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/blocks/blk1/children/batch_delete"
        );
    }
}
