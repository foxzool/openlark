//! 获取人工任务详情 API

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取审批实例详情的请求构建器。
pub struct GetInstanceRequestBuilder {
    approval_instance_id: String,
    config: Config,
}

impl GetInstanceRequestBuilder {
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
    pub async fn execute(self) -> SDKResult<GetInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetInstanceResponse> {
        validate_required!(self.approval_instance_id, "实例ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}",
            self.approval_instance_id
        );
        let api_request: ApiRequest<GetInstanceResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取人工任务详情", "响应数据为空")
        })
    }
}

/// 审批实例详情响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInstanceResponse {
    /// 审批实例 ID。
    pub instance_id: String,
    /// 审批实例状态。
    pub status: String,
    /// 发起人 ID。
    pub initiator_id: String,
    /// 创建时间。
    pub create_time: String,
}

impl ApiResponseTrait for GetInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to GetInstanceRequestBuilder, will be removed in v1.0 (#271)")]
pub type GetInstanceBuilder = GetInstanceRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../approval_instances/{id} → 强类型 GetInstanceResponse。
    #[tokio::test]
    async fn test_get_approval_instance_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/approval_instances/inst_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "instance_id": "inst_001",
                    "status": "APPROVING",
                    "initiator_id": "u_001",
                    "create_time": "1717000000"
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

        let resp = GetInstanceRequestBuilder::new(config)
            .approval_instance_id("inst_001")
            .execute()
            .await
            .expect("获取审批实例详情应成功");
        assert_eq!(resp.instance_id, "inst_001");
        assert_eq!(resp.status, "APPROVING");
        assert_eq!(resp.initiator_id, "u_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/approval_instances/inst_001"
        );
    }
}
