//! 营业执照识别
//!
//! 识别营业执照中的企业信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/business_license_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE;

/// 营业执照识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessLicenseRecognizeBody {
    pub file_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl BusinessLicenseRecognizeBody {
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessLicenseRecognizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessLicenseRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BusinessLicenseRecognizeResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessLicenseRecognizeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingResult {
    /// 统一社会信用代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_code: Option<String>,
    /// 注册号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// 公司名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// 法定代表人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_person: Option<String>,
    /// 注册资本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_capital: Option<String>,
    /// 成立日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub establish_date: Option<String>,
    /// 营业期限起始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 营业期限结束
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 经营范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_scope: Option<String>,
    /// 注册地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 公司类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BusinessLicenseRecognizeRequest {
    config: Config,
}

impl BusinessLicenseRecognizeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn execute(
        self,
        body: BusinessLicenseRecognizeBody,
    ) -> SDKResult<BusinessLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BusinessLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BusinessLicenseRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<BusinessLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "营业执照识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "营业执照识别")
    }
}

#[derive(Debug, Clone)]
pub struct BusinessLicenseRecognizeRequestBuilder {
    request: BusinessLicenseRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl BusinessLicenseRecognizeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: BusinessLicenseRecognizeRequest::new(config),
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

    pub fn body(self) -> BusinessLicenseRecognizeBody {
        BusinessLicenseRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    pub async fn execute(self) -> SDKResult<BusinessLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BusinessLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

pub async fn business_license_recognize(
    config: &Config,
    body: BusinessLicenseRecognizeBody,
) -> SDKResult<BusinessLicenseRecognizeResponse> {
    business_license_recognize_with_options(config, body, RequestOption::default()).await
}

pub async fn business_license_recognize_with_options(
    config: &Config,
    body: BusinessLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<BusinessLicenseRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<BusinessLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "营业执照识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "营业执照识别")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = BusinessLicenseRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BusinessLicenseRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_ok());
    }
}
