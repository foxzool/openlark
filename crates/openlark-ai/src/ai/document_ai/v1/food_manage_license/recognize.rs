//! 食品经营许可证识别
//!
//! 识别食品经营许可证信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/food_manage_license_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE;

/// 食品经营许可证识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl FoodManageLicenseRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 食品经营许可证识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FoodManageLicenseRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for FoodManageLicenseRecognizeResponse {}

/// 食品经营许可证识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodManageLicenseRecognizeResult {
    /// 许可证编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    /// 许可项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_item: Option<String>,
    /// 有效期至
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
    /// 主体名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_name: Option<String>,
    /// 住所
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 法定代表人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_person: Option<String>,
    /// 签发机关
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_authority: Option<String>,
}

/// 食品经营许可证识别请求
#[derive(Debug, Clone)]
pub struct FoodManageLicenseRecognizeRequest {
    config: Config,
}

impl FoodManageLicenseRecognizeRequest {
    /// 创建新的食品经营许可证识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行食品经营许可证识别请求
    pub async fn execute(
        self,
        body: FoodManageLicenseRecognizeBody,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行食品经营许可证识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: FoodManageLicenseRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<FoodManageLicenseRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE)
                .body(serialize_params(&body, "食品经营许可证识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "食品经营许可证识别")
    }
}

/// 食品经营许可证识别请求构建器
#[derive(Debug, Clone)]
pub struct FoodManageLicenseRecognizeRequestBuilder {
    request: FoodManageLicenseRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl FoodManageLicenseRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: FoodManageLicenseRecognizeRequest::new(config),
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
    pub fn body(self) -> FoodManageLicenseRecognizeBody {
        FoodManageLicenseRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FoodManageLicenseRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行食品经营许可证识别
///
/// docPath: https://open.feishu.cn/document/document_ai-v1/food_manage_license_recognize
pub async fn food_manage_license_recognize(
    config: &Config,
    body: FoodManageLicenseRecognizeBody,
) -> SDKResult<FoodManageLicenseRecognizeResponse> {
    food_manage_license_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行食品经营许可证识别（支持自定义选项）
pub async fn food_manage_license_recognize_with_options(
    config: &Config,
    body: FoodManageLicenseRecognizeBody,
    option: RequestOption,
) -> SDKResult<FoodManageLicenseRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<FoodManageLicenseRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE)
            .body(serialize_params(&body, "食品经营许可证识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "食品经营许可证识别")
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
        let builder = FoodManageLicenseRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = FoodManageLicenseRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = FoodManageLicenseRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
