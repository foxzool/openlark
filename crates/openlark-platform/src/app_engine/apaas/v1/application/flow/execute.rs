//! 发起流程
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/flow/application-flow/execute>
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/application-flow/execute>

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
pub struct FlowExecuteRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 流程 ID
    flow_id: String,
    /// 流程参数
    params: serde_json::Value,
}

impl FlowExecuteRequestBuilder {
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
    pub instance_id: String,
    /// 流程状态
    #[serde(rename = "status")]
    pub status: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for FlowExecuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to FlowExecuteRequestBuilder, will be removed in v1.0 (#271)")]
pub type FlowExecuteBuilder = FlowExecuteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../flows/{flow_id}/execute → 强类型 FlowExecuteResponse。
    #[tokio::test]
    async fn test_execute_flow_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/flows/flow_001/execute",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "instance_id": "inst_001",
                    "status": "RUNNING",
                    "message": "流程已发起"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = FlowExecuteRequestBuilder::new(config, "ns_test", "flow_001")
            .params(json!({"input": "value"}))
            .execute()
            .await
            .expect("发起流程应成功");
        assert_eq!(resp.instance_id, "inst_001");
        assert_eq!(resp.status, "RUNNING");
        assert_eq!(resp.message, "流程已发起");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/flows/flow_001/execute"
        );
    }
}
