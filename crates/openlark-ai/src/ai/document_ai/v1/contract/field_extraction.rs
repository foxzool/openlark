//! 合同字段提取
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/document_ai-v1/contract/field_extraction>
//!
//! 请求为 `multipart/form-data`：`file` + 必填表单字段 `pdf_page_limit` / `ocr_mode`。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION;

/// 合同字段提取请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFieldExtractionBody {
    /// 合同源文件（pdf/doc/docx，小于 10M）。
    #[serde(skip_serializing)]
    pub file: Vec<u8>,
    /// multipart 文件名，仅用于设置文件 part 的 filename。
    #[serde(rename = "__file_name", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// PDF 页数限制（最大 100）。
    pub pdf_page_limit: i32,
    /// OCR 模式：`force` / `auto` / `unused` 等。
    pub ocr_mode: String,
}

impl ContractFieldExtractionBody {
    /// 验证请求参数。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file, "file 不能为空");
        validate_required!(self.ocr_mode, "ocr_mode 不能为空");
        if self.pdf_page_limit <= 0 {
            return Err(openlark_core::error::CoreError::validation_msg(
                "pdf_page_limit 必须大于 0",
            ));
        }
        Ok(())
    }
}

/// 合同字段提取响应 `data`（结构较深，保留主要顶层字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContractFieldExtractionResponse {
    /// 文件 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// 金额相关抽取结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    /// 时间相关抽取结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    /// 期限相关抽取结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_term: Option<serde_json::Value>,
    /// 份数相关抽取结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy: Option<serde_json::Value>,
    /// 币种相关抽取结果。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<serde_json::Value>,
    /// 正文/主体信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_info: Option<serde_json::Value>,
    /// 银行信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_info: Option<serde_json::Value>,
}

impl openlark_core::api::ApiResponseTrait for ContractFieldExtractionResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// 合同字段提取请求。
#[derive(Debug, Clone)]
pub struct ContractFieldExtractionRequest {
    config: Config,
}

impl ContractFieldExtractionRequest {
    /// 创建新的合同字段提取请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行合同字段提取请求。
    pub async fn execute(
        self,
        body: ContractFieldExtractionBody,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行合同字段提取请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        body: ContractFieldExtractionBody,
        option: RequestOption,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        body.validate()?;

        let req: ApiRequest<ContractFieldExtractionResponse> =
            ApiRequest::post(DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION)
                .body(serialize_params(&body, "合同字段提取")?)
                .file_content(body.file.clone());

        Transport::request_typed(req, &self.config, Some(option), "合同字段提取").await
    }
}

/// 合同字段提取请求构建器。
#[derive(Debug, Clone)]
pub struct ContractFieldExtractionRequestBuilder {
    request: ContractFieldExtractionRequest,
    file: Option<Vec<u8>>,
    file_name: Option<String>,
    pdf_page_limit: Option<i32>,
    ocr_mode: Option<String>,
}

impl ContractFieldExtractionRequestBuilder {
    /// 创建新的构建器。
    pub fn new(config: Config) -> Self {
        Self {
            request: ContractFieldExtractionRequest::new(config),
            file: None,
            file_name: None,
            pdf_page_limit: None,
            ocr_mode: None,
        }
    }

    /// 设置合同源文件。
    pub fn file(mut self, file: impl Into<Vec<u8>>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// 设置 multipart 文件名。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 设置 PDF 页数限制。
    pub fn pdf_page_limit(mut self, pdf_page_limit: i32) -> Self {
        self.pdf_page_limit = Some(pdf_page_limit);
        self
    }

    /// 设置 OCR 模式。
    pub fn ocr_mode(mut self, ocr_mode: impl Into<String>) -> Self {
        self.ocr_mode = Some(ocr_mode.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> ContractFieldExtractionBody {
        ContractFieldExtractionBody {
            file: self.file.unwrap_or_default(),
            file_name: self.file_name,
            pdf_page_limit: self.pdf_page_limit.unwrap_or(0),
            ocr_mode: self.ocr_mode.unwrap_or_default(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ContractFieldExtractionResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行合同字段提取。
pub async fn contract_field_extraction(
    config: &Config,
    body: ContractFieldExtractionBody,
) -> SDKResult<ContractFieldExtractionResponse> {
    contract_field_extraction_with_options(config, body, RequestOption::default()).await
}

/// 执行合同字段提取（支持自定义选项）。
pub async fn contract_field_extraction_with_options(
    config: &Config,
    body: ContractFieldExtractionBody,
    option: RequestOption,
) -> SDKResult<ContractFieldExtractionResponse> {
    body.validate()?;

    let req: ApiRequest<ContractFieldExtractionResponse> =
        ApiRequest::post(DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION)
            .body(serialize_params(&body, "合同字段提取")?)
            .file_content(body.file.clone());

    Transport::request_typed(req, config, Some(option), "合同字段提取").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_form_fields() {
        let body = ContractFieldExtractionBody {
            file: b"%PDF".to_vec(),
            file_name: Some("c.pdf".into()),
            pdf_page_limit: 15,
            ocr_mode: "auto".into(),
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["pdf_page_limit"], 15);
        assert_eq!(v["ocr_mode"], "auto");
        assert_eq!(v["__file_name"], "c.pdf");
        assert!(v.get("file").is_none());
        assert!(v.get("file_token").is_none());
        assert!(v.get("is_async").is_none());
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_builder() {
        let config = Config::builder().app_id("a").app_secret("s").build();
        let body = ContractFieldExtractionRequestBuilder::new(config)
            .file(b"data".to_vec())
            .file_name("a.pdf")
            .pdf_page_limit(10)
            .ocr_mode("force")
            .body();
        assert_eq!(body.pdf_page_limit, 10);
        assert_eq!(body.ocr_mode, "force");
    }
}
