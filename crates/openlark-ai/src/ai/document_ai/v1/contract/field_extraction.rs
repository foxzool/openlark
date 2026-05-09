//! 合同字段提取
//!
//! 智能提取合同中的关键字段。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/contract_field_extraction

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION;

/// 合同字段提取请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFieldExtractionBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl ContractFieldExtractionBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 合同字段提取响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFieldExtractionResponse {
    /// 提取结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ContractFieldExtractionResult>,
}

impl openlark_core::api::ApiResponseTrait for ContractFieldExtractionResponse {}

/// 合同字段提取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFieldExtractionResult {
    /// 合同标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_title: Option<String>,
    /// 合同编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// 甲方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_a_name: Option<String>,
    /// 乙方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_b_name: Option<String>,
    /// 合同金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_amount: Option<String>,
    /// 合同开始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 合同结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 签署日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_date: Option<String>,
    /// 其他字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_fields: Option<Vec<FieldItem>>,
}

/// 字段项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldItem {
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
}

/// 合同字段提取请求
#[derive(Debug, Clone)]
pub struct ContractFieldExtractionRequest {
    config: Config,
}

impl ContractFieldExtractionRequest {
    /// 创建新的合同字段提取请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行合同字段提取请求
    pub async fn execute(
        self,
        body: ContractFieldExtractionBody,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行合同字段提取请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: ContractFieldExtractionBody,
        option: RequestOption,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        body.validate()?;

        let req: ApiRequest<ContractFieldExtractionResponse> =
            ApiRequest::post(DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION)
                .body(serialize_params(&body, "合同字段提取")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "合同字段提取")
    }
}

/// 合同字段提取请求构建器
#[derive(Debug, Clone)]
pub struct ContractFieldExtractionRequestBuilder {
    request: ContractFieldExtractionRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl ContractFieldExtractionRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: ContractFieldExtractionRequest::new(config),
            file_token: None,
            is_async: None,
        }
    }

    /// 设置文件 token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置是否异步
    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
        self
    }

    /// 构建请求体
    pub fn body(self) -> ContractFieldExtractionBody {
        ContractFieldExtractionBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ContractFieldExtractionResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ContractFieldExtractionResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行合同字段提取
///
/// docPath: https://open.feishu.cn/document/document_ai-v1/contract_field_extraction
pub async fn contract_field_extraction(
    config: &Config,
    body: ContractFieldExtractionBody,
) -> SDKResult<ContractFieldExtractionResponse> {
    contract_field_extraction_with_options(config, body, RequestOption::default()).await
}

/// 执行合同字段提取（支持自定义选项）
pub async fn contract_field_extraction_with_options(
    config: &Config,
    body: ContractFieldExtractionBody,
    option: RequestOption,
) -> SDKResult<ContractFieldExtractionResponse> {
    body.validate()?;

    let req: ApiRequest<ContractFieldExtractionResponse> =
        ApiRequest::post(DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION)
            .body(serialize_params(&body, "合同字段提取")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "合同字段提取")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_state() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = ContractFieldExtractionRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = ContractFieldExtractionBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = ContractFieldExtractionBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
