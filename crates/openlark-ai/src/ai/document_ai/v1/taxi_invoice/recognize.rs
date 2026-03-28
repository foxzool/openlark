//! 出租车发票识别
//!
//! 识别出租车发票信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/taxi_invoice_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE;

/// 出租车发票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxiInvoiceRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl TaxiInvoiceRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 出租车发票识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxiInvoiceRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaxiInvoiceRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for TaxiInvoiceRecognizeResponse {}

/// 出租车发票识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxiInvoiceRecognizeResult {
    /// 发票代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_code: Option<String>,
    /// 发票号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// 上车时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_time: Option<String>,
    /// 下车时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alighting_time: Option<String>,
    /// 里程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mileage: Option<String>,
    /// 车牌号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plate_number: Option<String>,
}

/// 出租车发票识别请求
#[derive(Debug, Clone)]
pub struct TaxiInvoiceRecognizeRequest {
    config: Config,
}

impl TaxiInvoiceRecognizeRequest {
    /// 创建新的出租车发票识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行出租车发票识别请求
    pub async fn execute(
        self,
        body: TaxiInvoiceRecognizeBody,
    ) -> SDKResult<TaxiInvoiceRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行出租车发票识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: TaxiInvoiceRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<TaxiInvoiceRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<TaxiInvoiceRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE)
                .body(serialize_params(&body, "出租车发票识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "出租车发票识别")
    }
}

/// 出租车发票识别请求构建器
#[derive(Debug, Clone)]
pub struct TaxiInvoiceRecognizeRequestBuilder {
    request: TaxiInvoiceRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl TaxiInvoiceRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: TaxiInvoiceRecognizeRequest::new(config),
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
    pub fn body(self) -> TaxiInvoiceRecognizeBody {
        TaxiInvoiceRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TaxiInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TaxiInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行出租车发票识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/taxi_invoice_recognize>
pub async fn taxi_invoice_recognize(
    config: &Config,
    body: TaxiInvoiceRecognizeBody,
) -> SDKResult<TaxiInvoiceRecognizeResponse> {
    taxi_invoice_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行出租车发票识别（支持自定义选项）
pub async fn taxi_invoice_recognize_with_options(
    config: &Config,
    body: TaxiInvoiceRecognizeBody,
    option: RequestOption,
) -> SDKResult<TaxiInvoiceRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<TaxiInvoiceRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE)
            .body(serialize_params(&body, "出租车发票识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "出租车发票识别")
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
        let builder = TaxiInvoiceRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = TaxiInvoiceRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = TaxiInvoiceRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
