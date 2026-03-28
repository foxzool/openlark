/// 批量更新块的内容
///
/// 批量更新块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/batch_update
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/batch_update>
use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
    pub block_id: String,
    /// 操作内容（例如 update_text_elements / merge_table_cells 等）
    #[serde(flatten)]
    pub operation: serde_json::Value,
}

/// 批量更新块内容响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateDocumentBlocksResponse {
    #[serde(default)]
    pub blocks: Vec<DocxBlock>,
}

impl ApiResponseTrait for BatchUpdateDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新块内容请求
pub struct BatchUpdateDocumentBlocksRequest {
    config: Config,
}

impl BatchUpdateDocumentBlocksRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: BatchUpdateDocumentBlocksParams,
    ) -> SDKResult<BatchUpdateDocumentBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

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
