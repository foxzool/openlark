//! 火车票识别
//!
//! 识别火车票信息。
//!
//! docPath: https://open.feishu.cn/document/document_ai-v1/train_invoice_recognize

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE;

/// 火车票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl TrainInvoiceRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        validate_required!(self.file_token, "file_token 不能为空");
        Ok(())
    }
}

/// 火车票识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TrainInvoiceRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for TrainInvoiceRecognizeResponse {}

/// 火车票识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainInvoiceRecognizeResult {
    /// 车票号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_number: Option<String>,
    /// 姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 票价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// 车次
    #[serde(skip_serializing_if = "Option::is_none")]
    pub train_number: Option<String>,
    /// 出发站
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_station: Option<String>,
    /// 到达站
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_station: Option<String>,
    /// 座位号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_number: Option<String>,
}

/// 火车票识别请求
#[derive(Debug, Clone)]
pub struct TrainInvoiceRecognizeRequest {
    config: Config,
}

impl TrainInvoiceRecognizeRequest {
    /// 创建新的火车票识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行火车票识别请求
    pub async fn execute(
        self,
        body: TrainInvoiceRecognizeBody,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行火车票识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: TrainInvoiceRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        body.validate()?;

        let req: ApiRequest<TrainInvoiceRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE)
                .body(serialize_params(&body, "火车票识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "火车票识别")
    }
}

/// 火车票识别请求构建器
#[derive(Debug, Clone)]
pub struct TrainInvoiceRecognizeRequestBuilder {
    request: TrainInvoiceRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl TrainInvoiceRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: TrainInvoiceRecognizeRequest::new(config),
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
    pub fn body(self) -> TrainInvoiceRecognizeBody {
        TrainInvoiceRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TrainInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TrainInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行火车票识别
///
/// docPath: https://open.feishu.cn/document/document_ai-v1/train_invoice_recognize
pub async fn train_invoice_recognize(
    config: &Config,
    body: TrainInvoiceRecognizeBody,
) -> SDKResult<TrainInvoiceRecognizeResponse> {
    train_invoice_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行火车票识别（支持自定义选项）
pub async fn train_invoice_recognize_with_options(
    config: &Config,
    body: TrainInvoiceRecognizeBody,
    option: RequestOption,
) -> SDKResult<TrainInvoiceRecognizeResponse> {
    body.validate()?;

    let req: ApiRequest<TrainInvoiceRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE)
            .body(serialize_params(&body, "火车票识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "火车票识别")
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
        let builder = TrainInvoiceRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = TrainInvoiceRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = TrainInvoiceRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
