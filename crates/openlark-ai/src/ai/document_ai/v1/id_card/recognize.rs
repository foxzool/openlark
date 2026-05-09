//! 身份证识别
//!
//! 识别身份证中的关键信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/id_card_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_ID_CARD_RECOGNIZE;

/// 身份证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl IdCardRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 身份证识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardRecognizeResponse {
    /// data 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<IdCardRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for IdCardRecognizeResponse {}

/// 身份证识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdCardRecognizeResult {
    /// 解析结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: IdCardRecognizeBody) -> SDKResult<IdCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: IdCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        body.validate()?;

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
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            request: IdCardRecognizeRequest::new(config),
            file_token: None,
            is_async: None,
        }
    }

    /// file_token。
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置 is_async。
    pub fn is_async(mut self, is_async: impl Into<bool>) -> Self {
        self.is_async = Some(is_async.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> IdCardRecognizeBody {
        IdCardRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<IdCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行身份证识别
pub async fn id_card_recognize(
    config: &Config,
    body: IdCardRecognizeBody,
) -> SDKResult<IdCardRecognizeResponse> {
    id_card_recognize_with_options(config, body, RequestOption::default()).await
}

/// id_card_recognize_with_options。
pub async fn id_card_recognize_with_options(
    config: &Config,
    body: IdCardRecognizeBody,
    option: RequestOption,
) -> SDKResult<IdCardRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<IdCardRecognizeResponse> = ApiRequest::post(DOCUMENT_AI_ID_CARD_RECOGNIZE)
        .body(serialize_params(&body, "身份证识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "身份证识别")
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
    fn test_builder_is_async() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = IdCardRecognizeRequestBuilder::new(config.clone()).is_async(true);
        assert_eq!(builder.is_async, Some(true));
    }

    #[test]
    fn test_builder_body_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = IdCardRecognizeRequestBuilder::new(config.clone())
            .file_token("token_123")
            .is_async(true)
            .body();
        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.is_async, Some(true));
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

    #[test]
    fn test_body_serialization() {
        let body = IdCardRecognizeBody {
            file_token: "token_123".to_string(),
            is_async: Some(true),
        };
        let json_str = serde_json::to_string(&body).expect("序列化失败");
        assert!(json_str.contains("file_token"));
        assert!(json_str.contains("token_123"));
    }

    #[test]
    fn test_parsing_result_serialization() {
        let parsing_result = ParsingResult {
            name: Some("张三".to_string()),
            gender: Some("男".to_string()),
            nation: Some("汉".to_string()),
            birth_date: Some("19900101".to_string()),
            address: Some("北京市".to_string()),
            id_number: Some("110101199001011234".to_string()),
            issuing_authority: Some("北京市公安局".to_string()),
            start_date: Some("20200101".to_string()),
            end_date: Some("20300101".to_string()),
            portrait: Some("base64_image_data".to_string()),
        };
        let json_str = serde_json::to_string(&parsing_result).expect("序列化失败");
        assert!(json_str.contains("张三"));
        assert!(json_str.contains("110101199001011234"));
    }

    #[test]
    fn test_response_struct() {
        let response = IdCardRecognizeResponse { data: None };
        assert!(response.data.is_none());

        let result = IdCardRecognizeResult {
            parsing_result: Some(ParsingResult {
                name: Some("李四".to_string()),
                gender: None,
                nation: None,
                birth_date: None,
                address: None,
                id_number: None,
                issuing_authority: None,
                start_date: None,
                end_date: None,
                portrait: None,
            }),
        };
        let response_with_data = IdCardRecognizeResponse { data: Some(result) };
        assert!(response_with_data.data.is_some());
        let name = response_with_data
            .data
            .as_ref()
            .and_then(|data| data.parsing_result.as_ref())
            .and_then(|result| result.name.as_deref());
        assert_eq!(name, Some("李四"));
    }
}
