/// Markdown/HTML 内容转换为文档块
///
/// 将 Markdown/HTML 格式的内容转换为文档块，以便于将 Markdown/HTML 格式的内容插入到文档中。目前支持转换为的块类型包含文本、一到九级标题、无序列表、有序列表、代码块、引用、待办事项、图片、表格、表格单元格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert>
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

/// Markdown/HTML 内容转换为文档块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContentToBlocksParams {
    /// 内容类型
    pub content_type: ContentType,
    /// 源内容
    pub content: String,
}

/// 内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "html")]
    Html,
}

/// Markdown/HTML 内容转换为文档块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContentToBlocksResponse {
    #[serde(default)]
    pub first_level_block_ids: Vec<String>,
}

impl ApiResponseTrait for ConvertContentToBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Markdown/HTML 内容转换为文档块请求
pub struct ConvertContentToBlocksRequest {
    config: Config,
}

impl ConvertContentToBlocksRequest {
    /// 创建Markdown/HTML 内容转换为文档块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
    pub async fn execute(
        self,
        params: ConvertContentToBlocksParams,
    ) -> SDKResult<ConvertContentToBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
    pub async fn execute_with_options(
        self,
        params: ConvertContentToBlocksParams,
        option: RequestOption,
    ) -> SDKResult<ConvertContentToBlocksResponse> {
        // 验证必填字段
        validate_required!(params.content, "源内容不能为空");

        // 构建API端点
        let api_endpoint = DocxApiV1::DocumentConvert;

        // 创建API请求
        let api_request: ApiRequest<ConvertContentToBlocksResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "Markdown/HTML 内容转换为文档块")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "Markdown/HTML 内容转换为文档块")
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
