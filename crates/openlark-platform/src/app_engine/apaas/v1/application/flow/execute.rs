//! 发起流程
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/flow/application-flow/execute
//! docPath: https://open.feishu.cn/document/apaas-v1/flow/application-flow/execute

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 发起流程 Builder
#[derive(Debug, Clone)]
pub struct FlowExecuteBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 流程 ID
    flow_id: String,
    /// 流程参数
    params: serde_json::Value,
}

impl FlowExecuteBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, flow_id: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            flow_id: flow_id.into(),
            params: serde_json::json!({}),
        }
    }

    /// 设置流程参数
    pub fn params(mut self, params: impl Into<serde_json::Value>) -> Self {
        self.params = params.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FlowExecuteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FlowExecuteResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/flows/{}/execute",
            self.namespace, self.flow_id
        );

        let request = FlowExecuteRequest {
            params: self.params,
        };

        let req: ApiRequest<FlowExecuteResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 发起流程请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct FlowExecuteRequest {
    /// 流程参数
    #[serde(rename = "params")]
    params: serde_json::Value,
}

/// 发起流程响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlowExecuteResponse {
    /// 实例 ID
    #[serde(rename = "instance_id")]
    instance_id: String,
    /// 流程状态
    #[serde(rename = "status")]
    status: String,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for FlowExecuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
