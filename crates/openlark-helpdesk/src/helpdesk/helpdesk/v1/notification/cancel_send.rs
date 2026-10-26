//! 取消推送通知发送
//!
//! 取消推送通知发送。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/cancel_send>

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

/// 取消推送通知发送请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSendNotificationBody {
    /// 是否召回已发送的消息
    pub is_recall: bool,
}

impl CancelSendNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        Ok(())
    }
}

/// 取消推送通知发送响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelSendNotificationResponse {}

impl ApiResponseTrait for CancelSendNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消推送通知发送请求
#[derive(Debug, Clone)]
pub struct CancelSendNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl CancelSendNotificationRequest {
    /// 创建新的取消推送通知发送请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行取消推送通知发送请求
    pub async fn execute(
        self,
        body: CancelSendNotificationBody,
    ) -> SDKResult<CancelSendNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行取消推送通知发送请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CancelSendNotificationBody,
        option: RequestOption,
    ) -> SDKResult<CancelSendNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<CancelSendNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationCancelSend(self.notification_id.clone()).to_url(),
        )
        .body(serialize_params(&body, "取消推送通知发送")?);

        Transport::request_typed(req, &self.config, Some(option), "取消推送通知发送").await
    }
}

/// 取消推送通知发送请求构建器
#[derive(Debug, Clone)]
pub struct CancelSendNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
    body: CancelSendNotificationBody,
}

impl CancelSendNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
            body: CancelSendNotificationBody { is_recall: false },
        }
    }

    /// 设置是否召回已发送的消息
    pub fn is_recall(mut self, is_recall: bool) -> Self {
        self.body.is_recall = is_recall;
        self
    }

    /// 构建请求体
    pub fn body(&self) -> CancelSendNotificationBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CancelSendNotificationResponse> {
        let request =
            CancelSendNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute(self.body()).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<CancelSendNotificationResponse> {
        let request =
            CancelSendNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute_with_options(self.body(), option).await
    }
}

/// 执行取消推送通知发送
pub async fn cancel_send_notification(
    config: &Config,
    notification_id: String,
    body: CancelSendNotificationBody,
) -> SDKResult<CancelSendNotificationResponse> {
    cancel_send_notification_with_options(config, notification_id, body, RequestOption::default())
        .await
}

/// 执行取消推送通知发送（支持自定义选项）
pub async fn cancel_send_notification_with_options(
    config: &Config,
    notification_id: String,
    body: CancelSendNotificationBody,
    option: RequestOption,
) -> SDKResult<CancelSendNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<CancelSendNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationCancelSend(notification_id).to_url())
            .body(serialize_params(&body, "取消推送通知发送")?);

    Transport::request_typed(req, config, Some(option), "取消推送通知发送").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_serializes_official_shape() {
        let body = CancelSendNotificationBody { is_recall: true };
        let value = serde_json::to_value(&body).expect("请求体应可序列化");

        assert_eq!(value["is_recall"], true);
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            CancelSendNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string())
                .is_recall(true);

        assert_eq!(builder.notification_id, "notif_123");
        assert!(builder.body().is_recall);
    }

    /// 端到端：POST .../notifications/{id}/cancel_send（body: is_recall）→ 空 body 响应成功返回。
    #[tokio::test]
    async fn test_cancel_send_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/helpdesk/v1/notifications/ntf_001/cancel_send",
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

        CancelSendNotificationRequest::new(config, "ntf_001".to_string())
            .execute(CancelSendNotificationBody { is_recall: true })
            .await
            .expect("取消推送通知发送应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications/ntf_001/cancel_send"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["is_recall"], true);
    }
}
