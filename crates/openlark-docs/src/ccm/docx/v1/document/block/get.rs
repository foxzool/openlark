/// 获取块的内容
///
/// 指定块的 block id 获取指定块的富文本内容数据。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/get>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::api_endpoints::DocxApiV1;
use crate::common::api_utils::*;

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
    pub block: DocxBlock,
}

impl ApiResponseTrait for GetDocumentBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取块内容请求
pub struct GetDocumentBlockRequest {
    config: Config,
}

impl GetDocumentBlockRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: GetDocumentBlockParams,
    ) -> SDKResult<GetDocumentBlockResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        params: GetDocumentBlockParams,
        option: RequestOption,
    ) -> SDKResult<GetDocumentBlockResponse> {
        validate_required!(params.document_id, "文档ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::DocumentBlockGet(params.document_id.clone(), params.block_id.clone());
        let mut api_request: ApiRequest<GetDocumentBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取块的内容")
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
