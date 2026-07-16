/// 创建嵌套块
///
/// 在指定块的子块列表中，新创建一批有父子关系的子块，并放置到指定位置。
/// 如果操作成功，接口将返回新建块与临时 block_id 的映射关系。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-descendant/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-descendant/create>
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
use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 创建嵌套块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockDescendantParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 父块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 文档版本号（可选，-1 表示最新版本）
    #[serde(skip_serializing)]
    pub document_revision_id: Option<i64>,

    /// 插入位置索引（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 需要插入的子块临时 ID 列表（顺序即插入顺序）
    pub children_id: Vec<String>,
    /// 以临时 ID 组织的嵌套块结构（按文档定义传入）
    pub descendants: Vec<serde_json::Value>,
}

/// 创建嵌套块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockDescendantResponse {
    /// 临时块 ID 与真实块 ID 的映射。
    #[serde(default)]
    pub block_id_relations: Vec<BlockIdRelation>,
    /// 新建子块列表（部分场景返回）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DocxBlock>>,
    /// 文档版本号（操作后的文档版本）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_revision_id: Option<i32>,
    /// 幂等标记（请求时传入的 client_token 原样回传）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

/// 临时 block_id 与实际 block_id 的映射关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockIdRelation {
    /// 真实块 ID。
    pub block_id: String,
    /// 临时块 ID。
    pub temporary_block_id: String,
}

impl ApiResponseTrait for CreateDocumentBlockDescendantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建嵌套块请求
///
/// 用于在文档块下创建一批存在父子关系的嵌套块。
pub struct CreateDocumentBlockDescendantRequest {
    config: Config,
}

impl CreateDocumentBlockDescendantRequest {
    /// 创建新的创建嵌套块请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(
        self,
        params: CreateDocumentBlockDescendantParams,
    ) -> SDKResult<CreateDocumentBlockDescendantResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        params: CreateDocumentBlockDescendantParams,
        option: RequestOption,
    ) -> SDKResult<CreateDocumentBlockDescendantResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children_id, "children_id不能为空");
        validate_required!(params.descendants, "descendants不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockDescendantCreate(
            params.document_id.clone(),
            params.block_id.clone(),
        );

        let mut api_request: ApiRequest<CreateDocumentBlockDescendantResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&params, "创建嵌套块")?);

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建嵌套块")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../blocks/{block_id}/descendant → CreateDocumentBlockDescendantResponse（block_id_relations）。
    #[tokio::test]
    async fn test_create_document_block_descendant_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/docx/v1/documents/doc1/blocks/blk1/descendant",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "block_id_relations": [] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateDocumentBlockDescendantRequest::new(config)
            .execute(CreateDocumentBlockDescendantParams {
                document_id: "doc1".into(),
                block_id: "blk1".into(),
                document_revision_id: None,
                index: None,
                children_id: vec!["tmp1".into()],
                descendants: vec![json!({ "block_id": "tmp1", "block_type": 2 })],
            })
            .await
            .expect("创建嵌套块应成功");
        assert!(resp.block_id_relations.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/documents/doc1/blocks/blk1/descendant"
        );
    }
}
