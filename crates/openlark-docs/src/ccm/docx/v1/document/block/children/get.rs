/// 获取所有子块
///
/// 获取文档中指定块的所有子块的富文本内容并分页返回。文档版本号可选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get>
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

use crate::ccm::docx::models::common_types::DocxBlock;

/// 获取所有子块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockChildrenParams {
    /// 文档ID
    pub document_id: String,
    /// 父块ID
    pub block_id: String,
    /// 文档版本号（可选，-1 表示最新版本）
    pub document_revision_id: Option<i64>,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取所有子块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlockChildrenResponse {
    /// 子块列表
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    /// 下一页分页标记。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取所有子块请求
///
/// 用于查询文档块下的子块列表。
pub struct GetDocumentBlockChildrenRequest {
    config: Config,
}

impl GetDocumentBlockChildrenRequest {
    /// 创建新的获取子块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: GetDocumentBlockChildrenParams,
    ) -> SDKResult<GetDocumentBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: GetDocumentBlockChildrenParams,
        option: RequestOption,
    ) -> SDKResult<GetDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockChildrenGet(
            params.document_id.clone(),
            params.block_id.clone(),
        );
        let mut api_request: ApiRequest<GetDocumentBlockChildrenResponse> =
            api_endpoint.to_request();

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }

        Transport::request_typed(api_request, &self.config, Some(option), "获取所有子块").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../blocks/{block_id}/children → GetDocumentBlockChildrenResponse（items）。
    #[tokio::test]
    async fn test_get_document_block_children_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/docx/v1/documents/doc1/blocks/blk1/children",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "items": [{ "block_id": "blk1", "block_type": 1 }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetDocumentBlockChildrenRequest::new(config)
            .execute(GetDocumentBlockChildrenParams {
                document_id: "doc1".into(),
                block_id: "blk1".into(),
                document_revision_id: None,
                page_size: None,
                page_token: None,
            })
            .await
            .expect("获取子块应成功");
        assert_eq!(resp.items.len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/blocks/blk1/children"
        );
    }
}
