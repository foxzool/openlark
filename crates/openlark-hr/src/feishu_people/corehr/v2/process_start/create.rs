//! 发起流程
//!
//! docPath: <https://open.feishu.cn/document/corehr-v1/process-form_variable_data/process-instance/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 发起流程请求体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessStartBody {
    /// 流程编码。
    pub process_code: Option<String>,
    /// 表单数据。
    pub form_data: Option<serde_json::Value>,
}
/// 发起流程响应。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessStartResponse {
    /// 流程实例 ID。
    pub process_instance_id: Option<String>,
}
impl ApiResponseTrait for ProcessStartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
/// 发起流程请求。
#[derive(Debug, Clone)]
pub struct ProcessStartRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 请求体。
    body: ProcessStartBody,
}
impl ProcessStartRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ProcessStartBody::default(),
        }
    }
    /// 设置请求体。
    pub fn body(mut self, body: ProcessStartBody) -> Self {
        self.body = body;
        self
    }
    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ProcessStartResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }
    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ProcessStartResponse> {
        let request =
            ApiRequest::<ProcessStartResponse>::post("/open-apis/corehr/v2/process_start")
                .body(serde_json::to_value(&self.body)?);
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("发起流程", "响应数据为空"))
    }
}
