//! 预览推送通知
//!
//! 预览推送通知内容。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/preview>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 预览推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PreviewNotificationResult>,
}

impl openlark_core::api::ApiResponseTrait for PreviewNotificationResponse {}

/// 预览推送通知结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewNotificationResult {
    /// 预览内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 预览推送通知请求
#[derive(Debug, Clone)]
pub struct PreviewNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl PreviewNotificationRequest {
    /// 创建新的预览推送通知请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行预览推送通知请求
    pub async fn execute(self) -> SDKResult<PreviewNotificationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行预览推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PreviewNotificationResponse> {
        let req: ApiRequest<PreviewNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationPreview(self.notification_id.clone()).to_url(),
        );

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "预览推送通知")
    }
}

/// 预览推送通知请求构建器
#[derive(Debug, Clone)]
pub struct PreviewNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
}

impl PreviewNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PreviewNotificationResponse> {
        let request =
            PreviewNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PreviewNotificationResponse> {
        let request =
            PreviewNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute_with_options(option).await
    }
}

/// 执行预览推送通知
pub async fn preview_notification(
    config: &Config,
    notification_id: String,
) -> SDKResult<PreviewNotificationResponse> {
    preview_notification_with_options(config, notification_id, RequestOption::default()).await
}

/// 执行预览推送通知（支持自定义选项）
pub async fn preview_notification_with_options(
    config: &Config,
    notification_id: String,
    option: RequestOption,
) -> SDKResult<PreviewNotificationResponse> {
    let req: ApiRequest<PreviewNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationPreview(notification_id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "预览推送通知")
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
        let builder =
            PreviewNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
    }
}
