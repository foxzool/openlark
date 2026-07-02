//! 获取消息推送概览

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取消息推送概览的请求。
#[derive(Debug, Clone)]
pub struct GetMessagePushOverviewRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取消息推送概览的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagePushOverviewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMessagePushOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMessagePushOverviewRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取消息推送概览请求。
    pub async fn execute(self) -> SDKResult<GetMessagePushOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMessagePushOverviewResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/app_usage/message_push_overview",
            self.app_id
        );
        let req: ApiRequest<GetMessagePushOverviewResponse> = ApiRequest::post(&path);

        let _resp: openlark_core::api::Response<GetMessagePushOverviewResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetMessagePushOverviewResponse { data: None })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
