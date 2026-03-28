//! 营业执照识别 API
//!
//! 识别营业执照中的企业信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/business_license_recognize>

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE;
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 营业执照识别请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessLicenseRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
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

/// 营业执照识别响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessLicenseRecognizeResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BusinessLicenseRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BusinessLicenseRecognizeResponse {}

/// 营业执照识别结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessLicenseRecognizeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_result: Option<ParsingResult>,
}

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// 营业执照识别请求
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

/// 营业执照识别请求构建器
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

/// 执行营业执照识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/business_license_recognize>
pub fn create(config: Config) -> BusinessLicenseRecognizeRequestBuilder {
    BusinessLicenseRecognizeRequestBuilder::new(config)
}

/// 执行营业执照识别（支持自定义选项）
pub fn create_with_options(
    config: Config,
    _options: RequestOption,
) -> BusinessLicenseRecognizeRequestBuilder {
    BusinessLicenseRecognizeRequestBuilder::new(config)
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

    #[test]
    fn test_body_validation_whitespace() {
        let body = BusinessLicenseRecognizeBody {
            file_token: "   ".to_string(),
            is_async: None,
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_parsing_result_struct() {
        let parsing_result = ParsingResult {
            credit_code: Some("91110000XXXXXXXXXX".to_string()),
            registration_number: Some("110000XXXXXX".to_string()),
            company_name: Some("测试科技有限公司".to_string()),
            legal_person: Some("张三".to_string()),
            registered_capital: Some("1000万元".to_string()),
            establish_date: Some("2020-01-01".to_string()),
            start_date: Some("2020-01-01".to_string()),
            end_date: Some("长期".to_string()),
            business_scope: Some("技术开发、技术咨询".to_string()),
            address: Some("北京市海淀区".to_string()),
            company_type: Some("有限责任公司".to_string()),
        };
        assert_eq!(
            parsing_result.company_name,
            Some("测试科技有限公司".to_string())
        );
        assert_eq!(parsing_result.legal_person, Some("张三".to_string()));
    }
}
