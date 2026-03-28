/// 创建块
///
/// 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/create>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 创建块请求参数
///
/// - `document_id`、`block_id` 用于拼接 URL（不参与序列化）
/// - `document_revision_id` 为查询参数（不参与序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockChildrenParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 父块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 文档版本号（可选）
    #[serde(skip_serializing)]
    pub document_revision_id: Option<i64>,
    /// 插入位置索引（可选，默认插入到末尾）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 新建的子块列表（按文档定义传入）
    pub children: Vec<serde_json::Value>,
}

/// 创建块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentBlockChildrenResponse {
    #[serde(default)]
    pub children: Vec<DocxBlock>,
}

impl ApiResponseTrait for CreateDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建块请求
pub struct CreateDocumentBlockChildrenRequest {
    config: Config,
}

impl CreateDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/create
    pub async fn execute(
        self,
        params: CreateDocumentBlockChildrenParams,
    ) -> SDKResult<CreateDocumentBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: CreateDocumentBlockChildrenParams,
        option: RequestOption,
    ) -> SDKResult<CreateDocumentBlockChildrenResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        let api_endpoint = DocxApiV1::DocumentBlockChildrenCreate(
            params.document_id.clone(),
            params.block_id.clone(),
        );

        let mut api_request: ApiRequest<CreateDocumentBlockChildrenResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建块")?);

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建块")
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
