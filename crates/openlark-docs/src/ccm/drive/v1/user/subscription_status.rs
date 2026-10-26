//! 查询用户云文档事件订阅状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/user/subscription_status>

use std::sync::Arc;

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::DriveApi;

/// 查询用户云文档事件订阅状态请求。
#[derive(Debug, Clone)]
pub struct UserSubscriptionStatusRequest {
    config: Arc<Config>,
    /// 事件类型，当前仅支持 drive.notice.comment_add_v1（添加评论、回复通知事件）
    event_type: String,
}

impl UserSubscriptionStatusRequest {
    /// 创建请求。
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `event_type` - 事件类型，当前仅支持 drive.notice.comment_add_v1
    pub fn new(config: Arc<Config>, event_type: impl Into<String>) -> Self {
        Self {
            config,
            event_type: event_type.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.event_type, "event_type 不能为空");
        let req: ApiRequest<serde_json::Value> = DriveApi::UserSubscriptionStatus
            .to_request()
            .query("event_type", &self.event_type);
        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "查询用户云文档事件订阅状态",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let request = UserSubscriptionStatusRequest::new(config, "drive.notice.comment_add_v1");
        assert_eq!(request.event_type, "drive.notice.comment_add_v1");
    }

    /// 测试空白 event_type 校验失败
    #[tokio::test]
    async fn rejects_blank_event_type() {
        let config = Arc::new(Config::default());
        let err = UserSubscriptionStatusRequest::new(config, "  ")
            .execute()
            .await
            .expect_err("空白 event_type 应校验失败");
        assert!(err.to_string().contains("event_type"));
    }

    /// 端到端：GET /open-apis/drive/v1/user/subscription_status?event_type=... → 订阅状态。
    #[tokio::test]
    async fn test_subscription_status_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/drive/v1/user/subscription_status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "data": {
                        "is_subscribe": true
                    }
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

        let resp =
            UserSubscriptionStatusRequest::new(Arc::new(config), "drive.notice.comment_add_v1")
                .execute()
                .await
                .expect("查询用户云文档事件订阅状态应成功");
        // 官方文档响应示例为嵌套信封 data.data.is_subscribe，透传 Value 原样返回
        assert_eq!(
            resp.pointer("/data/is_subscribe"),
            Some(&serde_json::json!(true))
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/user/subscription_status"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("event_type=drive.notice.comment_add_v1")
        );
    }
}
