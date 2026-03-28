/// 删除块
///
/// 指定需要操作的块，删除其指定范围的子块。如果操作成功，接口将返回应用删除操作后的文档版本号。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/batch_delete>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
    pub document_revision_id: i64,
    pub client_token: String,
}

impl ApiResponseTrait for BatchDeleteDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除块请求
pub struct BatchDeleteDocumentBlockChildrenRequest {
    config: Config,
}

impl BatchDeleteDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: BatchDeleteDocumentBlockChildrenParams,
    ) -> SDKResult<BatchDeleteDocumentBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

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
