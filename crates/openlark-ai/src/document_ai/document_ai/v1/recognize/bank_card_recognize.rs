//! 银行卡识别 API
//!
//! 识别银行卡中的关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/bank_card_recognize>

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_BANK_CARD_RECOGNIZE;
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 银行卡识别请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankCardRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl BankCardRecognizeBody {
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 银行卡识别响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankCardRecognizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BankCardRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BankCardRecognizeResponse {}

/// 银行卡识别结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankCardRecognizeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsingResult {
    /// 银行名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// 银行卡号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// 有效期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_date: Option<String>,
    /// 卡片类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
}

/// 银行卡识别请求
#[derive(Debug, Clone)]
pub struct BankCardRecognizeRequest {
    config: Config,
}

impl BankCardRecognizeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        body: BankCardRecognizeBody,
    ) -> SDKResult<BankCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BankCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BankCardRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<BankCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_BANK_CARD_RECOGNIZE)
                .body(serialize_params(&body, "银行卡识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "银行卡识别")
    }
}

/// 银行卡识别请求构建器
#[derive(Debug, Clone)]
pub struct BankCardRecognizeRequestBuilder {
    request: BankCardRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl BankCardRecognizeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BankCardRecognizeRequest::new(config),
            file_token: None,
            is_async: None,
        }
    }

    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
        self
    }

    pub fn body(self) -> BankCardRecognizeBody {
        BankCardRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    pub async fn execute(self) -> SDKResult<BankCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BankCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行银行卡识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/bank_card_recognize>
pub fn create(config: Config) -> BankCardRecognizeRequestBuilder {
    BankCardRecognizeRequestBuilder::new(config)
}

/// 执行银行卡识别（支持自定义选项）
pub fn create_with_options(
    config: Config,
    _options: RequestOption,
) -> BankCardRecognizeRequestBuilder {
    BankCardRecognizeRequestBuilder::new(config)
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
        let builder = BankCardRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file_token.is_none());
    }

    #[test]
    fn test_body_validation_empty() {
        let body = BankCardRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BankCardRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_builder_is_async() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = BankCardRecognizeRequestBuilder::new(config.clone()).is_async(true);
        assert_eq!(builder.is_async, Some(true));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = BankCardRecognizeRequestBuilder::new(config.clone())
            .file_token("token_123")
            .is_async(true)
            .body();
        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.is_async, Some(true));
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = BankCardRecognizeBody {
            file_token: "   ".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_err());
    }
}
