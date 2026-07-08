/// 创建文档
///
/// 创建新版文档，文档标题和目录可选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 创建文档请求（流式 Builder 模式）
///
/// 用于创建新版文档。
pub struct CreateDocumentRequest {
    config: Config,
    /// 文档标题（可选）
    title: Option<String>,
    /// 文件夹 token（可选）
    folder_token: Option<String>,
}

/// 创建文档请求体（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CreateDocumentRequestBody {
    /// 文档标题（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文件夹 token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

/// 创建文档响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocumentResponse {
    /// 创建后的文档信息。
    pub document: CreatedDocument,
}

/// 创建文档返回的文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedDocument {
    /// 文档 ID。
    pub document_id: String,
    /// 文档修订版本号。
    pub revision_id: i64,
    /// 文档标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDocumentRequest {
    /// 创建新的文档创建请求。
    /// 创建创建文档请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            title: None,
            folder_token: None,
        }
    }

    /// 设置文档标题
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置文件夹 token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create>
    pub async fn execute(self) -> SDKResult<CreateDocumentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateDocumentResponse> {
        let api_endpoint = DocxApiV1::DocumentCreate;

        let request_body = CreateDocumentRequestBody {
            title: self.title,
            folder_token: self.folder_token,
        };

        let api_request: ApiRequest<CreateDocumentResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&request_body, "创建文档")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/docx/v1/documents → CreateDocumentResponse（document）。
    #[tokio::test]
    async fn test_create_document_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/docx/v1/documents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "document": { "document_id": "doc1", "revision_id": 1 } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateDocumentRequest::new(config)
            .title("测试文档")
            .execute()
            .await
            .expect("创建文档应成功");
        assert_eq!(resp.document.document_id, "doc1");
        assert_eq!(resp.document.revision_id, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/docx/v1/documents");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["title"], "测试文档");
    }
}
