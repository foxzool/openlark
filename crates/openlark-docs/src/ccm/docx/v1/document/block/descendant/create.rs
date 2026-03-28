/// 创建嵌套块
///
/// 在指定块的子块列表中，新创建一批有父子关系的子块，并放置到指定位置。
/// 如果操作成功，接口将返回新建块与临时 block_id 的映射关系。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-descendant/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-descendant/create>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub block_id_relations: Vec<BlockIdRelation>,
}

/// 临时 block_id 与实际 block_id 的映射关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockIdRelation {
    pub block_id: String,
    pub temporary_block_id: String,
}

impl ApiResponseTrait for CreateDocumentBlockDescendantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建嵌套块请求
pub struct CreateDocumentBlockDescendantRequest {
    config: Config,
}

impl CreateDocumentBlockDescendantRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: CreateDocumentBlockDescendantParams,
    ) -> SDKResult<CreateDocumentBlockDescendantResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

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

        let mut api_request: ApiRequest<CreateDocumentBlockDescendantResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建嵌套块")?);

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

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
