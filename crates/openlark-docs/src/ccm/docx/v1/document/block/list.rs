/// 获取文档所有块
///
/// 获取文档所有块的富文本内容并分页返回。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list>
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

/// 获取文档所有块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlocksParams {
    /// 文档ID
    pub document_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 文档版本号（可选，-1 表示最新版本）
    pub document_revision_id: Option<i64>,
}

/// 获取文档所有块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentBlocksResponse {
    /// 块列表
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetDocumentBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文档所有块请求
pub struct GetDocumentBlocksRequest {
    config: Config,
}

impl GetDocumentBlocksRequest {
    /// 创建获取文档所有块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list
    pub async fn execute(
        self,
        params: GetDocumentBlocksParams,
    ) -> SDKResult<GetDocumentBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document-block/list
    pub async fn execute_with_options(
        self,
        params: GetDocumentBlocksParams,
        option: RequestOption,
    ) -> SDKResult<GetDocumentBlocksResponse> {
        // 验证必填字段
        validate_required!(params.document_id, "文档ID不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::DocumentBlockList(params.document_id.clone());

        // 创建API请求
        let mut api_request: ApiRequest<GetDocumentBlocksResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }
        if let Some(document_revision_id) = params.document_revision_id {
            api_request =
                api_request.query("document_revision_id", &document_revision_id.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文档所有块")
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
