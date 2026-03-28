//! 行驶证识别
//!
//! 识别行驶证信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/vehicle_license_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE;

/// 行驶证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl VehicleLicenseRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 行驶证识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VehicleLicenseRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for VehicleLicenseRecognizeResponse {}

/// 行驶证识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleLicenseRecognizeResult {
    /// 号牌号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_number: Option<String>,
    /// 车辆类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,
    /// 所有人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// 住址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 品牌型号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_model: Option<String>,
    /// 注册日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_date: Option<String>,
    /// 发证日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<String>,
    /// 检验有效期至
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    /// 车辆识别代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    /// 发动机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_number: Option<String>,
}

/// 行驶证识别请求
#[derive(Debug, Clone)]
pub struct VehicleLicenseRecognizeRequest {
    config: Config,
}

impl VehicleLicenseRecognizeRequest {
    /// 创建新的行驶证识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行行驶证识别请求
    pub async fn execute(
        self,
        body: VehicleLicenseRecognizeBody,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行行驶证识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: VehicleLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<VehicleLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "行驶证识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "行驶证识别")
    }
}

/// 行驶证识别请求构建器
#[derive(Debug, Clone)]
pub struct VehicleLicenseRecognizeRequestBuilder {
    request: VehicleLicenseRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl VehicleLicenseRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: VehicleLicenseRecognizeRequest::new(config),
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
    pub fn body(self) -> VehicleLicenseRecognizeBody {
        VehicleLicenseRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<VehicleLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VehicleLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行行驶证识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/vehicle_license_recognize>
pub async fn vehicle_license_recognize(
    config: &Config,
    body: VehicleLicenseRecognizeBody,
) -> SDKResult<VehicleLicenseRecognizeResponse> {
    vehicle_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行行驶证识别（支持自定义选项）
pub async fn vehicle_license_recognize_with_options(
    config: &Config,
    body: VehicleLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<VehicleLicenseRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<VehicleLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "行驶证识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "行驶证识别")
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
        let builder = VehicleLicenseRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = VehicleLicenseRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = VehicleLicenseRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
