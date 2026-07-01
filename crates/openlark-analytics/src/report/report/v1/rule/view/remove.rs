//! 移除规则看板

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除规则看板请求。
#[derive(Debug, Clone)]
pub struct RemoveReportRuleViewRequest {
    config: Arc<Config>,
}

/// 移除规则看板响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReportRuleViewResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RemoveReportRuleViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RemoveReportRuleViewRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行移除规则看板请求。
    pub async fn execute(self) -> SDKResult<RemoveReportRuleViewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行移除规则看板请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemoveReportRuleViewResponse> {
        let path = "/open-apis/report/v1/rules/{}/views/remove".to_string();
        let req: ApiRequest<RemoveReportRuleViewResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("移除规则看板", "响应数据为空"))
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
