/// 更新块的内容
///
/// 更新指定块的内容。如果操作成功，接口将返回更新后的块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/patch>
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

/// 更新块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockParams {
    /// 文档ID
    #[serde(skip_serializing)]
    pub document_id: String,
    /// 块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 更新内容（按文档定义传入，例如 update_text_elements 等）
    #[serde(flatten)]
    pub update: serde_json::Value,
}

/// 更新块内容响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDocumentBlockResponse {
    pub block: DocxBlock,
}

impl ApiResponseTrait for UpdateDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新块内容请求
pub struct UpdateDocumentBlockRequest {
    config: Config,
}

impl UpdateDocumentBlockRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: UpdateDocumentBlockParams,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: UpdateDocumentBlockParams,
        option: RequestOption,
    ) -> SDKResult<UpdateDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::DocumentBlockPatch(params.document_id.clone(), params.block_id.clone());
        let mut api_request: ApiRequest<UpdateDocumentBlockResponse> =
            ApiRequest::patch(&api_endpoint.to_url());
        api_request = api_request.json_body(&params);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新块的内容")
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
