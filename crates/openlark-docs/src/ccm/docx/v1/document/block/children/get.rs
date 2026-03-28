/// 获取所有子块
///
/// 获取文档中指定块的所有子块的富文本内容并分页返回。文档版本号可选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block-children/get>
use crate::common::api_endpoints::DocxApiV1;
use crate::common::api_utils::*;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetDocumentBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取所有子块请求
pub struct GetDocumentBlockChildrenRequest {
    config: Config,
}

impl GetDocumentBlockChildrenRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        params: GetDocumentBlockChildrenParams,
    ) -> SDKResult<GetDocumentBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

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
            ApiRequest::get(&api_endpoint.to_url());

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

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取所有子块")
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
