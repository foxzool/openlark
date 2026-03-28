//! 身份证识别 API
//!
//! 识别身份证中的关键信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/id_card_recognize>

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_ID_CARD_RECOGNIZE;
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 身份证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdCardRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl IdCardRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 身份证识别响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdCardRecognizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IdCardRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for IdCardRecognizeResponse {}

/// 身份证识别结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdCardRecognizeResult {
    /// 解析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsingResult {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 民族
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nation: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 身份证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// 签发机关
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authority: Option<String>,
    /// 有效期限起始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 有效期限结束日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 头像 Base64 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portrait: Option<String>,
}

/// 身份证识别请求
#[derive(Debug, Clone)]
pub struct IdCardRecognizeRequest {
    config: Config,
}

impl IdCardRecognizeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(self, body: IdCardRecognizeBody) -> SDKResult<IdCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: IdCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<IdCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_ID_CARD_RECOGNIZE)
                .body(serialize_params(&body, "身份证识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "身份证识别")
    }
}

/// 身份证识别请求构建器
#[derive(Debug, Clone)]
pub struct IdCardRecognizeRequestBuilder {
    request: IdCardRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl IdCardRecognizeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: IdCardRecognizeRequest::new(config),
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

    pub fn body(self) -> IdCardRecognizeBody {
        IdCardRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    pub async fn execute(self) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行身份证识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/id_card_recognize>
pub fn create(config: Config) -> IdCardRecognizeRequestBuilder {
    IdCardRecognizeRequestBuilder::new(config)
}

/// 执行身份证识别（支持自定义选项）
pub fn create_with_options(
    config: Config,
    _options: RequestOption,
) -> IdCardRecognizeRequestBuilder {
    IdCardRecognizeRequestBuilder::new(config)
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
        let builder = IdCardRecognizeRequestBuilder::new(config.clone());
        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_builder_file_token() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            IdCardRecognizeRequestBuilder::new(config.clone()).file_token("file_token_123");
        assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_body_validation_empty() {
        let body = IdCardRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = IdCardRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(false),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_whitespace() {
        let body = IdCardRecognizeBody {
            file_token: "   ".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }
}
