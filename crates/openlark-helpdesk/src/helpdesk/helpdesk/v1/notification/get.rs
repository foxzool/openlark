//! 获取指定推送通知
//!
//! 获取指定推送通知的详情。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;

/// 获取指定推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNotificationResponse {
    /// 响应数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<NotificationItem>,
}

impl ApiResponseTrait for GetNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 推送通知项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationItem {
    /// 推送ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 获取指定推送通知请求
#[derive(Debug, Clone)]
pub struct GetNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl GetNotificationRequest {
    /// 创建新的获取指定推送通知请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行获取指定推送通知请求
    pub async fn execute(self) -> SDKResult<GetNotificationResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetNotificationResponse> {
        let api_endpoint = HelpdeskApiV1::NotificationGet(self.notification_id.clone());
        let request = ApiRequest::<GetNotificationResponse>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, Some(option), "获取指定推送通知").await
    }
}

/// 获取指定推送通知请求构建器
#[derive(Debug, Clone)]
pub struct GetNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
}

impl GetNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<GetNotificationResponse> {
        let api_endpoint = HelpdeskApiV1::NotificationGet(self.notification_id.clone());
        let request = ApiRequest::<GetNotificationResponse>::get(api_endpoint.to_url());

        Transport::request_typed(request, &self.config, None, "获取指定推送通知").await
    }
}

/// 执行获取指定推送通知
pub async fn get_notification(
    config: &Config,
    notification_id: String,
) -> SDKResult<GetNotificationResponse> {
    let api_endpoint = HelpdeskApiV1::NotificationGet(notification_id);
    let request = ApiRequest::<GetNotificationResponse>::get(api_endpoint.to_url());

    Transport::request_typed(request, config, None, "获取指定推送通知").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder = GetNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
    }

    /// 端到端：GET .../notifications/{id} → 强类型 GetNotificationResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_notification_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/helpdesk/v1/notifications/ntf_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "id": "ntf_001", "title": "系统维护通知", "status": "draft" } }
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

        let resp = GetNotificationRequest::new(config, "ntf_001".to_string())
            .execute()
            .await
            .expect("获取指定推送通知应成功");
        assert_eq!(resp.data.unwrap().id.as_deref(), Some("ntf_001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications/ntf_001"
        );
    }
}
