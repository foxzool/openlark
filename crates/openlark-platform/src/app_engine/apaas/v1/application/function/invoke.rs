//! 执行函数
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-function/invoke>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-function/invoke>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 执行函数 Builder
#[derive(Debug, Clone)]
pub struct FunctionInvokeRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 函数 API 名称
    function_api_name: String,
    /// 函数参数
    params: serde_json::Value,
}

impl FunctionInvokeRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        function_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            function_api_name: function_api_name.into(),
            params: serde_json::json!({}),
        }
    }

    /// 设置函数参数
    pub fn params(mut self, params: impl Into<serde_json::Value>) -> Self {
        self.params = params.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<FunctionInvokeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FunctionInvokeResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/functions/{}/invoke",
            self.namespace, self.function_api_name
        );

        let request = FunctionInvokeRequest {
            params: self.params,
        };

        let req: ApiRequest<FunctionInvokeResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 执行函数请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct FunctionInvokeRequest {
    /// 函数参数
    #[serde(rename = "params")]
    params: serde_json::Value,
}

/// 执行函数响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionInvokeResponse {
    /// 执行结果
    #[serde(rename = "result")]
    pub result: serde_json::Value,
    /// 执行状态
    #[serde(rename = "status")]
    pub status: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for FunctionInvokeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to FunctionInvokeRequestBuilder, will be removed in v1.0 (#271)")]
pub type FunctionInvokeBuilder = FunctionInvokeRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../functions/{api_name}/invoke → 强类型 FunctionInvokeResponse。
    #[tokio::test]
    async fn test_invoke_function_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/functions/func_001/invoke",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "result": {"output": "ok"},
                    "status": "SUCCESS",
                    "message": "执行成功"
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

        let resp = FunctionInvokeRequestBuilder::new(config, "ns_test", "func_001")
            .params(json!({"arg": 1}))
            .execute()
            .await
            .expect("执行函数应成功");
        assert_eq!(resp.status, "SUCCESS");
        assert_eq!(resp.message, "执行成功");
        assert_eq!(resp.result["output"], "ok");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/functions/func_001/invoke"
        );
    }
}
