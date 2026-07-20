//! 订阅审批任务状态变更事件（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/subscribe>

use std::sync::Arc;

use openlark_core::{SDKResult, api::ApiRequest, config::Config};

use crate::common::api_endpoints::ApprovalApiV4;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 订阅审批任务状态变更事件请求（v4）
///
/// 用于订阅审批任务的状态变更事件。
///
/// 请求体与响应均以 `serde_json::Value` 透传（openlark-api 核心契约 2 的
/// 无 schema 范式）：subscription 官方文档当前为 SPA 动态渲染、字段定义
/// 无法静态抓取，故 SDK 不臆测字段。待文档稳定后可收敛为 typed
/// Body/Response（参考同域 `task` 资源其它 leaf）。
pub struct SubscribeTaskRequestV4 {
    config: Arc<Config>,
}

impl SubscribeTaskRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let endpoint = ApprovalApiV4::TaskSubscribe;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(endpoint.to_url())
            .body(serialize_params(&body, "订阅审批任务状态变更")?);

        let resp = openlark_core::http::Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "订阅审批任务状态变更")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_subscribe_task_v4_url() {
        let endpoint = ApprovalApiV4::TaskSubscribe;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/tasks/subscription"
        );
    }

    /// 端到端：POST /open-apis/approval/v4/tasks/subscription，断言 path + body 透传。
    #[tokio::test]
    async fn test_subscribe_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/approval/v4/tasks/subscription"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        SubscribeTaskRequestV4::new(config)
            .execute(json!({ "subscription_type": "task_status_change" }))
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body_str = std::str::from_utf8(&received[0].body).expect("body utf8");
        assert!(
            body_str.contains("task_status_change"),
            "请求体应包含 subscription_type: {body_str}"
        );
    }
}
