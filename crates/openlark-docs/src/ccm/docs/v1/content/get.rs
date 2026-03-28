/// 获取云文档内容
///
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/docs-v1/content/get>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocsApiV1, api_utils::*};

/// 获取云文档内容请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetDocsContentRequest {
    /// 文档 Token
    pub doc_token: String,

    /// 文档类型
    pub doc_type: String,

    /// 导出内容类型
    pub content_type: String,

    /// 语言（可选）
    pub lang: Option<String>,
}

impl GetDocsContentRequest {
    pub fn new(
        doc_token: impl Into<String>,

        doc_type: impl Into<String>,

        content_type: impl Into<String>,
    ) -> Self {
        Self {
            doc_token: doc_token.into(),

            doc_type: doc_type.into(),

            content_type: content_type.into(),

            lang: None,
        }
    }

    pub fn lang(mut self, lang: impl Into<String>) -> Self {
        self.lang = Some(lang.into());

        self
    }
}

/// 获取云文档内容响应（data）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetDocsContentResponse {
    pub content: String,
}

impl ApiResponseTrait for GetDocsContentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档内容
pub async fn get_docs_content(
    request: GetDocsContentRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetDocsContentResponse> {
    validate_required!(request.doc_token, "doc_token 不能为空");

    validate_required!(request.doc_type, "doc_type 不能为空");

    validate_required!(request.content_type, "content_type 不能为空");

    let api_endpoint = DocsApiV1::ContentGet;

    let api_request: ApiRequest<GetDocsContentResponse> = ApiRequest::get(&api_endpoint.to_url())
        .query("doc_token", &request.doc_token)
        .query("doc_type", &request.doc_type)
        .query("content_type", &request.content_type)
        .query_opt("lang", request.lang);

    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取云文档内容")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn test_get_docs_content_request_builder() {
        let request = GetDocsContentRequest::new("doc_token", "docx", "markdown");

        assert_eq!(request.doc_token, "doc_token");

        assert_eq!(request.doc_type, "docx");

        assert_eq!(request.content_type, "markdown");

        assert!(request.lang.is_none());
    }

    #[test]

    fn test_get_docs_content_request_with_lang() {
        let request = GetDocsContentRequest::new("doc_token", "docx", "markdown").lang("zh");

        assert_eq!(
            request
                .lang
                .expect("lang should be set when .lang() is called"),
            "zh"
        );
    }

    #[test]

    fn test_response_trait() {
        assert_eq!(GetDocsContentResponse::data_format(), ResponseFormat::Data);
    }
}
