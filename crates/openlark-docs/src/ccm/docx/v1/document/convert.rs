/// Markdown/HTML 内容转换为文档块
///
/// 将 Markdown/HTML 格式的内容转换为文档块，以便于将 Markdown/HTML 格式的内容插入到文档中。目前支持转换为的块类型包含文本、一到九级标题、无序列表、有序列表、代码块、引用、待办事项、图片、表格、表格单元格。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/convert>
use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::api_endpoints::DocxApiV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
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
    /// Markdown 内容。
    #[serde(rename = "markdown")]
    Markdown,
    /// HTML 内容。
    #[serde(rename = "html")]
    Html,
}

/// Markdown/HTML 内容转换为文档块响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertContentToBlocksResponse {
    /// 一级块 ID 列表。
    #[serde(default)]
    pub first_level_block_ids: Vec<String>,
    /// 转换后的完整块列表（部分场景返回）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<DocxBlock>>,
    /// 块 ID 到图片 URL 的映射（含图片的转换结果）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_id_to_image_urls: Option<serde_json::Value>,
}

impl ApiResponseTrait for ConvertContentToBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Markdown/HTML 内容转换为文档块请求
///
/// 用于把 markdown/html 转换为可插入文档的块结构。
pub struct ConvertContentToBlocksRequest {
    config: Config,
}

impl ConvertContentToBlocksRequest {
    /// 创建新的内容转换请求。
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

    /// 使用指定请求选项执行请求。
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
        let api_request: ApiRequest<ConvertContentToBlocksResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&params, "Markdown/HTML 内容转换为文档块")?);

        // 发送请求
        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Markdown/HTML 内容转换为文档块",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/docx/documents/blocks/convert → ConvertContentToBlocksResponse（first_level_block_ids）。
    #[tokio::test]
    async fn test_convert_content_to_blocks_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/docx/documents/blocks/convert"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "first_level_block_ids": ["blk1", "blk2"] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ConvertContentToBlocksRequest::new(config)
            .execute(ConvertContentToBlocksParams {
                content_type: ContentType::Markdown,
                content: "# 标题".into(),
            })
            .await
            .expect("转换内容应成功");
        assert_eq!(resp.first_level_block_ids.len(), 2);
        assert_eq!(resp.first_level_block_ids[0], "blk1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/documents/blocks/convert"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["content_type"], "markdown");
    }
}
