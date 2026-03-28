//! 机动车发票识别
//!
//! 识别机动车发票信息。
//!
//! docPath: <https://open.feishu.cn/document/document_ai-v1/vehicle_invoice_recognize>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE;

/// 机动车发票识别请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInvoiceRecognizeBody {
    /// 用户的 file_id，通过上传文件接口获取
    pub file_token: String,
    /// 是否异步识别，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
}

impl VehicleInvoiceRecognizeBody {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("file_token 不能为空".to_string());
        }
        Ok(())
    }
}

/// 机动车发票识别响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInvoiceRecognizeResponse {
    /// 识别结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<VehicleInvoiceRecognizeResult>,
}

impl openlark_core::api::ApiResponseTrait for VehicleInvoiceRecognizeResponse {}

/// 机动车发票识别结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleInvoiceRecognizeResult {
    /// 发票代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_code: Option<String>,
    /// 发票号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// 日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 价税合计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    /// 不含税价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_excluded_amount: Option<String>,
    /// 税额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<String>,
    /// 购买方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_name: Option<String>,
    /// 销售方名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_name: Option<String>,
    /// 车辆类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle_type: Option<String>,
    /// 厂牌型号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_model: Option<String>,
}

/// 机动车发票识别请求
#[derive(Debug, Clone)]
pub struct VehicleInvoiceRecognizeRequest {
    config: Config,
}

impl VehicleInvoiceRecognizeRequest {
    /// 创建新的机动车发票识别请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行机动车发票识别请求
    pub async fn execute(
        self,
        body: VehicleInvoiceRecognizeBody,
    ) -> SDKResult<VehicleInvoiceRecognizeResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行机动车发票识别请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: VehicleInvoiceRecognizeBody,
        option: RequestOption,
    ) -> SDKResult<VehicleInvoiceRecognizeResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        let req: ApiRequest<VehicleInvoiceRecognizeResponse> =
            ApiRequest::post(DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE)
                .body(serialize_params(&body, "机动车发票识别")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "机动车发票识别")
    }
}

/// 机动车发票识别请求构建器
#[derive(Debug, Clone)]
pub struct VehicleInvoiceRecognizeRequestBuilder {
    request: VehicleInvoiceRecognizeRequest,
    file_token: Option<String>,
    is_async: Option<bool>,
}

impl VehicleInvoiceRecognizeRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Config) -> Self {
        Self {
            request: VehicleInvoiceRecognizeRequest::new(config),
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
    pub fn body(self) -> VehicleInvoiceRecognizeBody {
        VehicleInvoiceRecognizeBody {
            file_token: self.file_token.unwrap_or_default(),
            is_async: self.is_async,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<VehicleInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VehicleInvoiceRecognizeResponse> {
        let body = self.clone().body();
        self.request.execute_with_options(body, option).await
    }
}

/// 执行机动车发票识别
///
/// docPath: <https://open.feishu.cn/document/document_ai-v1/vehicle_invoice_recognize>
pub async fn vehicle_invoice_recognize(
    config: &Config,
    body: VehicleInvoiceRecognizeBody,
) -> SDKResult<VehicleInvoiceRecognizeResponse> {
    vehicle_invoice_recognize_with_options(config, body, RequestOption::default()).await
}

/// 执行机动车发票识别（支持自定义选项）
pub async fn vehicle_invoice_recognize_with_options(
    config: &Config,
    body: VehicleInvoiceRecognizeBody,
    option: RequestOption,
) -> SDKResult<VehicleInvoiceRecognizeResponse> {
    body.validate()
        .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

    let req: ApiRequest<VehicleInvoiceRecognizeResponse> =
        ApiRequest::post(DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE)
            .body(serialize_params(&body, "机动车发票识别")?);

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "机动车发票识别")
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
        let builder = VehicleInvoiceRecognizeRequestBuilder::new(config.clone());

        assert!(builder.file_token.is_none());
        assert!(builder.is_async.is_none());
    }

    #[test]
    fn test_body_validation_empty_file_token() {
        let body = VehicleInvoiceRecognizeBody {
            file_token: "".to_string(),
            is_async: None,
        };
        let result = body.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = VehicleInvoiceRecognizeBody {
            file_token: "valid_token".to_string(),
            is_async: Some(true),
        };
        let result = body.validate();
        assert!(result.is_ok());
    }
}
