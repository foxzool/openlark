//! OCR 基础识别
//!
//! 基础的 OCR 文字识别服务，将图片中的文字转换为可编辑文本。
//!
//! docPath: <https://open.feishu.cn/document/optical-char-recognition-v1/basic_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;

/// OCR 基础识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 识别结果的优先级顺序，默认值为 `[0,1]`，表示优先返回文本框位置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_model: Option<RecognitionModel>,
}

impl BasicRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 识别结果优先级模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecognitionModel {
    /// 优先返回文本框位置信息
    TextBox,
    /// 优先返回ocr的识别结果
    Ocr,
}

/// OCR 基础识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicRecognizeResponse {
    /// OCR 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BasicRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for BasicRecognizeResponse {}

/// OCR 基础识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicRecognizeResult {
    /// 识别到的文本片段列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_blocks: Option<Vec<TextBlock>>,
}

/// 文本块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    /// 文本片段的起始坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_selection_start: Option<i32>,
    /// 文本片段的结束坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_selection_end: Option<i32>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 文本框坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_boxes: Option<Vec<TextBox>>,
}

/// 文本框坐标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBox {
    /// 左上角 x 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<i32>,
    /// 左上角 y 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    /// 右下角 x 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<i32>,
    /// 右下角 y 坐标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<i32>,
}

/// OCR 基础识别请求
#[derive(Debug, Clone)]
pub struct BasicRecognizeRequest {
    config: Config,
}

impl BasicRecognizeRequest {
    /// 创建新的 OCR 基础识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行 OCR 基础识别请求
    pub async fn execute(self, body: BasicRecognizeBody) -> SDKResult<BasicRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行 OCR 基础识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: BasicRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<BasicRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<BasicRecognizeResponse> =
            ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE)
                .body(serialize_params(&body, "OCR 基础识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "OCR 基础识别")
    }
}

/// OCR 基础识别请求构建器
#[derive(Debug, Clone)]
pub struct BasicRecognizeRequestBuilder {
    request: BasicRecognizeRequest,
    file_token: Option<String>,
    recognition_model: Option<RecognitionModel>,
}

impl BasicRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: BasicRecognizeRequest::new(config),
            file_token: None,
            recognition_model: None,
        }
    }

    /// 设置文件 token
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = Some(file_token.into());
        self
    }

    /// 设置识别模型
    pub fn recognition_model(mut self, recognition_model: RecognitionModel) -> Self {
        self.recognition_model = Some(recognition_model);
        self
    }

    /// 构建请求体
    pub fn body(self) -> BasicRecognizeBody {
        BasicRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            recognition_model: self.recognition_model,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BasicRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BasicRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行 OCR 基础识别
///
/// docPath: <https://open.feishu.cn/document/optical-char-recognition-v1/basic_recognize>
pub async fn basic_recognize(
    config: &Config,
    body: BasicRecognizeBody,
) -> SDKResult<BasicRecognizeResponse> {
    basic_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行 OCR 基础识别（支持自定义选项）
pub async fn basic_recognize_with_options(
    config: &Config,
    body: BasicRecognizeBody,
    option: RequestOption,
) -> SDKResult<BasicRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<BasicRecognizeResponse> =
        ApiRequest::post(OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE)
            .body(serialize_params(&body, "OCR 基础识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "OCR 基础识别")
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
        let builder = BasicRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.recognition_model.is_none());
    }

    #[test]
    fn test_builder_file_token() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            BasicRecognizeRequestBuilder::new(config.clone()).file_token("file_token_123");

        assert_eq!(builder.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_builder_recognition_model() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = BasicRecognizeRequestBuilder::new(config.clone())
            .recognition_model(RecognitionModel::TextBox);

        assert_eq!(builder.recognition_model, Some(RecognitionModel::TextBox));
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = BasicRecognizeBody {
            file_token: "".to_string(),
            recognition_model: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = BasicRecognizeBody {
            file_token: "valid_token".to_string(),
            recognition_model: Some(RecognitionModel::Ocr),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_from_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = BasicRecognizeRequestBuilder::new(config.clone())
            .file_token("token_123")
            .recognition_model(RecognitionModel::TextBox)
            .body();

        assert_eq!(body.file_token, "token_123");
        assert_eq!(body.recognition_model, Some(RecognitionModel::TextBox));
    }
}
