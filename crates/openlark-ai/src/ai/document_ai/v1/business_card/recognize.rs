//! 名片识别
//!
//! 智能识别名片中的联系信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/business_card_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE;

/// 名片识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl BusinessCardRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 名片识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessCardRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BusinessCardRecognizeResponse {}

/// 名片识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessCardRecognizeResult {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// 公司
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 网站
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
}

/// 名片识别请求
#[derive(Debug, Clone)]
pub struct BusinessCardRecognizeRequest {
    config: Config,
}

impl BusinessCardRecognizeRequest {
    /// 创建新的名片识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行名片识别请求
    pub async fn execute(
        self,
        body: BusinessCardRecognizeBody,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行名片识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: BusinessCardRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<BusinessCardRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE)
                .body(serialize_params(&body, "名片识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "名片识别")
    }
}

/// 名片识别请求构建器
#[derive(Debug, Clone)]
pub struct BusinessCardRecognizeRequestBuilder {
    request: BusinessCardRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl BusinessCardRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: BusinessCardRecognizeRequest::new(config),
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
    pub fn body(self) -> BusinessCardRecognizeBody {
        BusinessCardRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BusinessCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BusinessCardRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行名片识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/business_card_recognize>
pub async fn business_card_recognize(
    config: &Config,
    body: BusinessCardRecognizeBody,
) -> SDKResult<BusinessCardRecognizeResponse> {
    business_card_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行名片识别（支持自定义选项）
pub async fn business_card_recognize_with_options(
    config: &Config,
    body: BusinessCardRecognizeBody,
    option: RequestOption,
) -> SDKResult<BusinessCardRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<BusinessCardRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE)
            .body(serialize_params(&body, "名片识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "名片识别")
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
        let builder = BusinessCardRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = BusinessCardRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BusinessCardRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
