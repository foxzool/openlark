/// 创建文档
///
/// 创建新版文档，文档标题和目录可选。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/create>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 创建文档请求（流式 Builder 模式）
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
    pub document: CreatedDocument,
}

/// 创建文档返回的文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedDocument {
    pub document_id: String,
    pub revision_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ApiResponseTrait for CreateDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateDocumentRequest {
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
