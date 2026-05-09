//! 驾驶证识别
//!
//! 识别驾驶证中的关键信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/driving_license_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE;

/// 驾驶证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrivingLicenseRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl DrivingLicenseRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 驾驶证识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrivingLicenseRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DrivingLicenseRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for DrivingLicenseRecognizeResponse {}

/// 驾驶证识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrivingLicenseRecognizeResult {
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 性别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// 国籍
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// 住址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 出生日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
    /// 初次领证日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// 准驾车型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,
    /// 有效起始日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    /// 有效截止日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    /// 证号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
}

/// 驾驶证识别请求
#[derive(Debug, Clone)]
pub struct DrivingLicenseRecognizeRequest {
    config: Config,
}

impl DrivingLicenseRecognizeRequest {
    /// 创建新的驾驶证识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行驾驶证识别请求
    pub async fn execute(
        self,
        body: DrivingLicenseRecognizeBody,
    ) -> SDKResult<DrivingLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行驾驶证识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: DrivingLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<DrivingLicenseRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<DrivingLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "驾驶证识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "驾驶证识别")
    }
}

/// 驾驶证识别请求构建器
#[derive(Debug, Clone)]
pub struct DrivingLicenseRecognizeRequestBuilder {
    request: DrivingLicenseRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl DrivingLicenseRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: DrivingLicenseRecognizeRequest::new(config),
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
    pub fn body(self) -> DrivingLicenseRecognizeBody {
        DrivingLicenseRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DrivingLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DrivingLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行驾驶证识别
///
/// docPath: https://open.feishu.cn/document/document_ai-v1/driving_license_recognize
pub async fn driving_license_recognize(
    config: &Config,
    body: DrivingLicenseRecognizeBody,
) -> SDKResult<DrivingLicenseRecognizeResponse> {
    driving_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行驾驶证识别（支持自定义选项）
pub async fn driving_license_recognize_with_options(
    config: &Config,
    body: DrivingLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<DrivingLicenseRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<DrivingLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "驾驶证识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "驾驶证识别")
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
        let builder = DrivingLicenseRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = DrivingLicenseRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = DrivingLicenseRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
