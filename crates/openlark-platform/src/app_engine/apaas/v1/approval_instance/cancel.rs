//! 撤销人工任务 API
//! docPath: <https://open.feishu.cn/document/apaas-v1/flow/user-task/cancel>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 撤销审批实例的请求构建器。
pub struct CancelInstanceRequestBuilder {
    approval_instance_id: String,
    config: Config,
}

impl CancelInstanceRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_instance_id: String::new(),
            config,
        }
    }

    /// 设置审批实例 ID。
    pub fn approval_instance_id(mut self, approval_instance_id: impl Into<String>) -> Self {
        self.approval_instance_id = approval_instance_id.into();
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CancelInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelInstanceResponse> {
        validate_required!(self.approval_instance_id, "实例ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}/cancel",
            self.approval_instance_id
        );
        let api_request: ApiRequest<CancelInstanceResponse> = ApiRequest::post(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("撤销人工任务", "响应数据为空"))
    }
}

/// 撤销审批实例的响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelInstanceResponse {
    /// 撤销执行结果。
    pub result: String,
}

impl ApiResponseTrait for CancelInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to CancelInstanceRequestBuilder, will be removed in v1.0 (#271)")]
pub type CancelInstanceBuilder = CancelInstanceRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../approval_instances/{id}/cancel → 强类型 CancelInstanceResponse。
    #[tokio::test]
    async fn test_cancel_approval_instance_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/approval_instances/inst_001/cancel",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "result": "success" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CancelInstanceRequestBuilder::new(config)
            .approval_instance_id("inst_001")
            .execute()
            .await
            .expect("撤销审批实例应成功");
        assert_eq!(resp.result, "success");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_instances/inst_001/cancel"
        );
    }
}
