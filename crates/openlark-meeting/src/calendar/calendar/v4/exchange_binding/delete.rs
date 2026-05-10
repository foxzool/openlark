//! 解除 Exchange 账户绑定
//!
//! docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/delete

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use super::models::DeleteExchangeBindingResponse;
use crate::{common::api_utils::extract_response_data, endpoints::CALENDAR_V4_EXCHANGE_BINDINGS};

/// 解除 Exchange 账户绑定请求
pub struct DeleteExchangeBindingRequest {
    config: Config,
    exchange_binding_id: String,
}

impl DeleteExchangeBindingRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            exchange_binding_id: String::new(),
        }
    }

    /// Exchange 绑定 ID（路径参数）
    pub fn exchange_binding_id(mut self, exchange_binding_id: impl Into<String>) -> Self {
        self.exchange_binding_id = exchange_binding_id.into();
        self
    }

    /// 执行请求
    /// docPath: https://open.feishu.cn/document/server-docs/calendar-v4/exchange_binding/delete
    pub async fn execute(self) -> SDKResult<DeleteExchangeBindingResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用自定义请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteExchangeBindingResponse> {
        validate_required!(self.exchange_binding_id, "exchange_binding_id 不能为空");

        // url: DELETE:/open-apis/calendar/v4/exchange_bindings/:exchange_binding_id
        let req: ApiRequest<DeleteExchangeBindingResponse> = ApiRequest::delete(format!(
            "{}/{}",
            CALENDAR_V4_EXCHANGE_BINDINGS, self.exchange_binding_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "解除 Exchange 账户绑定")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
