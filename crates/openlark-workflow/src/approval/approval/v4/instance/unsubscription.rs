//! 退订审批实例状态变更事件（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/unsubscription>

use std::sync::Arc;

use openlark_core::{SDKResult, api::ApiRequest, config::Config};

use crate::common::api_endpoints::ApprovalApiV4;

/// 退订审批实例状态变更事件请求（v4）
///
/// 用于退订审批实例的状态变更事件。官方契约：DELETE 请求，
/// 唯一参数为查询参数 `subscription_type`（INVOLVED_APPROVAL 参与审批订阅 /
/// MANAGED_APPROVAL 管理审批订阅），不传时取消所有类别订阅，无请求体。
pub struct UnsubscribeInstanceRequestV4 {
    config: Arc<Config>,
    subscription_type: Option<String>,
}

impl UnsubscribeInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            subscription_type: None,
        }
    }

    /// 设置需取消订阅的审批类型
    ///
    /// - `INVOLVED_APPROVAL`：参与审批订阅
    /// - `MANAGED_APPROVAL`：管理审批订阅
    ///
    /// 不设置时取消所有类别订阅。
    pub fn subscription_type(mut self, subscription_type: impl Into<String>) -> Self {
        self.subscription_type = Some(subscription_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let endpoint = ApprovalApiV4::InstanceUnsubscribe;
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::delete(endpoint.to_url());

        if let Some(subscription_type) = self.subscription_type {
            req = req.query("subscription_type", subscription_type);
        }

        openlark_core::http::Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "退订审批实例状态变更",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_unsubscribe_instance_v4_url() {
        let endpoint = ApprovalApiV4::InstanceUnsubscribe;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/subscription"
        );
    }

    /// 端到端：DELETE /open-apis/approval/v4/instances/subscription，
    /// 断言 method + path + query 参数 subscription_type，且无请求体。
    #[tokio::test]
    async fn test_unsubscribe_instance_sends_subscription_type_query() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/approval/v4/instances/subscription"))
            .and(query_param("subscription_type", "INVOLVED_APPROVAL"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .expect(1)
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

        UnsubscribeInstanceRequestV4::new(config)
            .subscription_type("INVOLVED_APPROVAL")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let url = received[0].url.clone();
        assert!(
            url.query_pairs()
                .any(|(k, v)| k == "subscription_type" && v == "INVOLVED_APPROVAL"),
            "query 应包含 subscription_type=INVOLVED_APPROVAL: {url}"
        );
        assert!(
            received[0].body.is_empty(),
            "官方契约 DELETE 无请求体，body 应为空"
        );
    }
}
