//! 退订审批任务状态变更事件（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/task/unsubscribe>

use std::sync::Arc;

use openlark_core::{SDKResult, api::ApiRequest, config::Config};

use crate::common::api_endpoints::ApprovalApiV4;
use crate::common::api_utils::serialize_params;

/// 退订审批任务状态变更事件请求（v4）
///
/// 用于退订审批任务的状态变更事件。请求体以 `serde_json::Value` 透传
/// （设计说明见 [`SubscribeTaskRequestV4`](super::subscription::SubscribeTaskRequestV4)）。
pub struct UnsubscribeTaskRequestV4 {
    config: Arc<Config>,
}

impl UnsubscribeTaskRequestV4 {
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
        let endpoint = ApprovalApiV4::TaskUnsubscribe;
        let req: ApiRequest<serde_json::Value> = ApiRequest::delete(endpoint.to_url())
            .body(serialize_params(&body, "退订审批任务状态变更")?);

        openlark_core::http::Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "退订审批任务状态变更",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_unsubscribe_task_v4_url() {
        let endpoint = ApprovalApiV4::TaskUnsubscribe;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/tasks/subscription"
        );
    }

    /// 端到端：DELETE /open-apis/approval/v4/tasks/subscription，断言 method + path + body。
    #[tokio::test]
    async fn test_unsubscribe_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
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

        UnsubscribeTaskRequestV4::new(config)
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
