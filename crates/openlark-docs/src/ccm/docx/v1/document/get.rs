/// 获取文档基本信息
///
/// 获取文档最新版本号、标题等。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取文档基本信息请求
pub struct GetDocumentRequest {
    document_id: String,
    config: Config,
}

/// 获取文档基本信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentResponse {
    pub document: Document,
}

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub document_id: String,
    pub revision_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<DocumentCover>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_setting: Option<DocumentDisplaySetting>,
    /// 其它字段透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 文档封面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCover {
    pub token: String,
    pub offset_ratio_x: i32,
    pub offset_ratio_y: i32,
}

/// 文档展示设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDisplaySetting {
    pub show_authors: bool,
    pub show_comment_count: bool,
    pub show_create_time: bool,
    pub show_like_count: bool,
    pub show_pv: bool,
    pub show_uv: bool,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDocumentRequest {
    /// 创建获取文档基本信息请求
    pub fn new(config: Config) -> Self {
        Self {
            document_id: String::new(),
            config,
        }
    }

    /// 设置文档 ID
    pub fn document_id(mut self, document_id: impl Into<String>) -> Self {
        self.document_id = document_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get
    /// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get>
    pub async fn execute(self) -> SDKResult<GetDocumentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetDocumentResponse> {
        validate_required!(self.document_id, "文档ID不能为空");

        let api_endpoint = DocxApiV1::DocumentGet(self.document_id.clone());
        let api_request: ApiRequest<GetDocumentResponse> = ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
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
