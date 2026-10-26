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
    /// 请求体
    body: CancelInstanceBody,
}

impl CancelInstanceRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            approval_instance_id: String::new(),
            config,
            body: CancelInstanceBody {
                user_id: String::new(),
                opinion: String::new(),
            },
        }
    }

    /// 设置审批实例 ID。
    pub fn approval_instance_id(mut self, approval_instance_id: impl Into<String>) -> Self {
        self.approval_instance_id = approval_instance_id.into();
        self
    }

    /// 设置操作用户的 kunlunUserID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 设置撤销原因。
    pub fn opinion(mut self, opinion: impl Into<String>) -> Self {
        self.body.opinion = opinion.into();
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
        validate_required!(self.body.user_id, "操作用户ID不能为空");
        validate_required!(self.body.opinion, "撤销原因不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}/cancel",
            self.approval_instance_id
        );
        let api_request: ApiRequest<CancelInstanceResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(api_request, &self.config, Some(option), "撤销人工任务").await
    }
}

/// 撤销审批实例的请求体。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelInstanceBody {
    /// 操作用户的 kunlunUserID，可通过 aPaaS 用户管理页面获取
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 撤销原因
    #[serde(rename = "opinion")]
    pub opinion: String,
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
            .user_id("u_001")
            .opinion("撤销了")
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
        // 请求体须与官方文档一致：user_id + opinion
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "user_id": "u_001",
                "opinion": "撤销了"
            })
        );
    }
}
