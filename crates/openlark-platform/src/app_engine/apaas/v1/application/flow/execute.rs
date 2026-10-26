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
    validate_required,
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
    /// 请求体
    body: FlowExecuteBody,
}

impl FlowExecuteRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>, flow_id: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            flow_id: flow_id.into(),
            body: FlowExecuteBody {
                operator: String::new(),
                is_async: None,
                idempotent_key: None,
                loop_masks: None,
                params: None,
            },
        }
    }

    /// 设置操作人（必填，_id 和 email 至少填一个，JSON 字符串，如 "{\"_id\": 1111, \"email\": \"apaas@bytedance.com\"}"）
    pub fn operator(mut self, operator: impl Into<String>) -> Self {
        self.body.operator = operator.into();
        self
    }

    /// 设置流程入参（JSON 字符串，无入参不传）
    pub fn params(mut self, params: impl Into<String>) -> Self {
        self.body.params = Some(params.into());
        self
    }

    /// 设置是否异步执行（不传默认 false）
    pub fn is_async(mut self, is_async: bool) -> Self {
        self.body.is_async = Some(is_async);
        self
    }

    /// 设置幂等键（建议本地生成 uuid 传入，重复的话请求会报错）
    pub fn idempotent_key(mut self, idempotent_key: impl Into<String>) -> Self {
        self.body.idempotent_key = Some(idempotent_key.into());
        self
    }

    /// 设置循环标志信息（当前版本可不传）
    pub fn loop_masks(mut self, loop_masks: Vec<String>) -> Self {
        self.body.loop_masks = Some(loop_masks);
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
        validate_required!(self.body.operator, "操作人不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/flows/{}/execute",
            self.namespace, self.flow_id
        );

        let req: ApiRequest<FlowExecuteResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "发起流程").await
    }
}

/// 发起流程请求体（官方字段：operator 必填，其余可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecuteBody {
    /// 操作人（JSON 字符串，_id 和 email 至少填一个）
    pub operator: String,
    /// 是否异步执行（不传默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_async: Option<bool>,
    /// 幂等键
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_key: Option<String>,
    /// 循环标志信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_masks: Option<Vec<String>>,
    /// 流程入参（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
}

/// 发起流程响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlowExecuteResponse {
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 输出参数（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_params: Option<String>,
    /// 执行 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_msg: Option<String>,
    /// code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
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

    /// 端到端：POST .../flows/{flow_id}/execute → 官方响应形状（status/out_params/execution_id）。
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
                "data": {
                    "code": "",
                    "error_msg": "",
                    "execution_id": "1816970091484211",
                    "out_params": "[{\"api_name\":\"_flowExecutionID\",\"value\":1816970091484211}]",
                    "status": "end"
                },
                "msg": "success"
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
            .operator("{\"_id\": 1111, \"email\": \"apaas@bytedance.com\"}")
            .params("{\"variable_rgrtgqworiginal\": {\"_id\": 5454545}}")
            .is_async(true)
            .execute()
            .await
            .expect("发起流程应成功");
        assert_eq!(resp.status.as_deref(), Some("end"));
        assert_eq!(resp.execution_id.as_deref(), Some("1816970091484211"));
        assert!(
            resp.out_params
                .as_deref()
                .is_some_and(|p| p.contains("_flowExecutionID"))
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/flows/flow_001/execute"
        );
        // 请求体须与官方文档一致：operator + params + is_async
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "operator": "{\"_id\": 1111, \"email\": \"apaas@bytedance.com\"}",
                "params": "{\"variable_rgrtgqworiginal\": {\"_id\": 5454545}}",
                "is_async": true
            })
        );
    }
}
