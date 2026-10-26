//! 执行推送通知
//!
//! 执行推送通知。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/execute_send>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::serialize_params;

/// 执行推送通知请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSendNotificationBody {
    /// 发送时间戳（毫秒）
    pub send_at: String,
}

impl ExecuteSendNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        openlark_core::validate_required!(self.send_at, "send_at 不能为空");
        Ok(())
    }
}

/// 执行推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteSendNotificationResponse {}

impl ApiResponseTrait for ExecuteSendNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 执行推送通知请求
#[derive(Debug, Clone)]
pub struct ExecuteSendNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl ExecuteSendNotificationRequest {
    /// 创建新的执行推送通知请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行执行推送通知请求
    pub async fn execute(
        self,
        body: ExecuteSendNotificationBody,
    ) -> SDKResult<ExecuteSendNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行执行推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: ExecuteSendNotificationBody,
        option: RequestOption,
    ) -> SDKResult<ExecuteSendNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<ExecuteSendNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationExecuteSend(self.notification_id.clone()).to_url(),
        )
        .body(serialize_params(&body, "执行推送通知")?);

        Transport::request_typed(req, &self.config, Some(option), "执行推送通知").await
    }
}

/// 执行推送通知请求构建器
#[derive(Debug, Clone)]
pub struct ExecuteSendNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
    body: ExecuteSendNotificationBody,
}

impl ExecuteSendNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
            body: ExecuteSendNotificationBody {
                send_at: String::new(),
            },
        }
    }

    /// 设置发送时间戳（毫秒）
    pub fn send_at(mut self, send_at: impl Into<String>) -> Self {
        self.body.send_at = send_at.into();
        self
    }

    /// 构建请求体
    pub fn body(&self) -> ExecuteSendNotificationBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<ExecuteSendNotificationResponse> {
        let request =
            ExecuteSendNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute(self.body()).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<ExecuteSendNotificationResponse> {
        let request =
            ExecuteSendNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute_with_options(self.body(), option).await
    }
}

/// 执行执行推送通知
pub async fn execute_send_notification(
    config: &Config,
    notification_id: String,
    body: ExecuteSendNotificationBody,
) -> SDKResult<ExecuteSendNotificationResponse> {
    execute_send_notification_with_options(config, notification_id, body, RequestOption::default())
        .await
}

/// 执行执行推送通知（支持自定义选项）
pub async fn execute_send_notification_with_options(
    config: &Config,
    notification_id: String,
    body: ExecuteSendNotificationBody,
    option: RequestOption,
) -> SDKResult<ExecuteSendNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<ExecuteSendNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationExecuteSend(notification_id).to_url())
            .body(serialize_params(&body, "执行推送通知")?);

    Transport::request_typed(req, config, Some(option), "执行推送通知").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_rejects_blank_send_at() {
        let body = ExecuteSendNotificationBody {
            send_at: String::new(),
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serializes_official_shape() {
        let body = ExecuteSendNotificationBody {
            send_at: "1624326025000".to_string(),
        };
        let value = serde_json::to_value(&body).expect("请求体应可序列化");

        assert_eq!(value["send_at"], "1624326025000");
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            ExecuteSendNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string())
                .send_at("1624326025000");

        assert_eq!(builder.notification_id, "notif_123");
        assert_eq!(builder.body().send_at, "1624326025000");
    }

    /// 端到端：POST .../notifications/{id}/execute_send（body: send_at）→ 空 body 响应成功返回。
    #[tokio::test]
    async fn test_execute_send_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/helpdesk/v1/notifications/ntf_001/execute_send",
            ))
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

        ExecuteSendNotificationRequest::new(config, "ntf_001".to_string())
            .execute(ExecuteSendNotificationBody {
                send_at: "1624326025000".to_string(),
            })
            .await
            .expect("执行推送通知应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications/ntf_001/execute_send"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["send_at"], "1624326025000");
    }
}
