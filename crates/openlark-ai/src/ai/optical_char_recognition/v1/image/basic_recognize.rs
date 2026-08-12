//! OCR 基础识别
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/ai/optical_char_recognition-v1/image/basic_recognize>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::serialize_params;
use crate::endpoints::OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE;

/// OCR 基础识别请求体。
///
/// 官方文档将 `image` 标为选填；SDK 将其建模为必填字符串（调用必须提供 base64）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicRecognizeBody {
    /// 图片 base64 编码后的字符串（文件需小于 5M）。
    pub image: String,
}

impl BasicRecognizeBody {
    /// 验证请求参数。
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.image, "image 不能为空");
        Ok(())
    }
}

/// OCR 基础识别响应 `data`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BasicRecognizeResponse {
    /// 按区域返回的文本列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_list: Option<Vec<String>>,
}

impl openlark_core::api::ApiResponseTrait for BasicRecognizeResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

/// OCR 基础识别请求。
#[derive(Debug, Clone)]
pub struct BasicRecognizeRequest {
    config: Config,
}

impl BasicRecognizeRequest {
    /// 创建新的 OCR 基础识别请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行 OCR 基础识别请求。
    pub async fn execute(self, body: BasicRecognizeBody) -> SDKResult<BasicRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行 OCR 基础识别请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        body: BasicRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BasicRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<BasicRecognizeResponse> =
            ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE)
                .body(serialize_params(&body, "OCR 基础识别")?);

        Transport::request_typed(req, &self.config, Some(option), "OCR 基础识别").await
    }
}

/// OCR 基础识别请求构建器。
#[derive(Debug, Clone)]
pub struct BasicRecognizeRequestBuilder {
    request: BasicRecognizeRequest,
    image: Option<String>,
}

impl BasicRecognizeRequestBuilder {
    /// 创建新的构建器。
    pub fn new(config: Config) -> Self {
        Self {
            request: BasicRecognizeRequest::new(config),
            image: None,
        }
    }

    /// 设置图片 base64 内容。
    pub fn image(mut self, image: impl Into<String>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// 构建请求体。
    pub fn body(self) -> BasicRecognizeBody {
        BasicRecognizeBody {
            image: self.image.unwrap_or_default(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BasicRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BasicRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行 OCR 基础识别。
pub async fn basic_recognize(
    config: &Config,
    body: BasicRecognizeBody,
) -> SDKResult<BasicRecognizeResponse> {
    basic_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行 OCR 基础识别（支持自定义选项）。
pub async fn basic_recognize_with_options(
    config: &Config,
    body: BasicRecognizeBody,
    option: RequestOption,
) -> SDKResult<BasicRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<BasicRecognizeResponse> =
        ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_IMAGE_BASIC_RECOGNIZE)
            .body(serialize_params(&body, "OCR 基础识别")?);

    Transport::request_typed(req, config, Some(option), "OCR 基础识别").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty_image() {
        let body = BasicRecognizeBody {
            image: "".to_string(),
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BasicRecognizeBody {
            image: "base64xxx".to_string(),
        };
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_body_serialization_official_fields() {
        let body = BasicRecognizeBody {
            image: "abc".into(),
        };
        let v = serde_json::to_value(&body).unwrap();
        assert_eq!(v["image"], "abc");
        assert!(v.get("file_token").is_none());
        assert!(v.get("recognition_model").is_none());
    }

    #[test]
    fn test_builder_image() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = BasicRecognizeRequestBuilder::new(config)
            .image("img_b64")
            .body();
        assert_eq!(body.image, "img_b64");
    }
}
