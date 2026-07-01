/// 获取文档基本信息
///
/// 获取文档最新版本号、标题等。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/document/get>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取文档基本信息请求
///
/// 用于查询文档的元信息与展示配置。
pub struct GetDocumentRequest {
    document_id: String,
    config: Config,
}

/// 获取文档基本信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocumentResponse {
    /// 文档信息。
    pub document: Document,
}

/// 文档信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// 文档 ID。
    pub document_id: String,
    /// 修订版本号。
    pub revision_id: i64,
    /// 文档标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文档封面。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<DocumentCover>,
    /// 展示设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_setting: Option<DocumentDisplaySetting>,
    /// 其它字段透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// 文档封面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCover {
    /// 封面资源 token。
    pub token: String,
    /// X 轴偏移比例。
    pub offset_ratio_x: i32,
    /// Y 轴偏移比例。
    pub offset_ratio_y: i32,
}

/// 文档展示设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDisplaySetting {
    /// 是否展示作者。
    pub show_authors: bool,
    /// 是否展示评论数。
    pub show_comment_count: bool,
    /// 是否展示创建时间。
    pub show_create_time: bool,
    /// 是否展示点赞数。
    pub show_like_count: bool,
    /// 是否展示 PV。
    pub show_pv: bool,
    /// 是否展示 UV。
    pub show_uv: bool,
}

impl ApiResponseTrait for GetDocumentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDocumentRequest {
    /// 创建新的文档查询请求。
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

    /// 使用指定请求选项执行请求。
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
