//! 取消推送通知审核
//!
//! 取消推送通知审核。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/cancel_approve>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 取消推送通知审核响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelApproveNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CancelApproveNotificationResult>,
}

impl openlark_core::api::ApiResponseTrait for CancelApproveNotificationResponse {}

/// 取消推送通知审核结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelApproveNotificationResult {
    /// 是否取消成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 取消推送通知审核请求
#[derive(Debug, Clone)]
pub struct CancelApproveNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl CancelApproveNotificationRequest {
    /// 创建新的取消推送通知审核请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行取消推送通知审核请求
    pub async fn execute(self) -> SDKResult<CancelApproveNotificationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行取消推送通知审核请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelApproveNotificationResponse> {
        let req: ApiRequest<CancelApproveNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationCancelApprove(self.notification_id.clone()).to_url(),
        );

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "取消推送通知审核")
    }
}

/// 取消推送通知审核请求构建器
#[derive(Debug, Clone)]
pub struct CancelApproveNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
}

impl CancelApproveNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CancelApproveNotificationResponse> {
        let request = CancelApproveNotificationRequest::new(
            self.config.clone(),
            self.notification_id.clone(),
        );
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<CancelApproveNotificationResponse> {
        let request = CancelApproveNotificationRequest::new(
            self.config.clone(),
            self.notification_id.clone(),
        );
        request.execute_with_options(option).await
    }
}

/// 执行取消推送通知审核
pub async fn cancel_approve_notification(
    config: &Config,
    notification_id: String,
) -> SDKResult<CancelApproveNotificationResponse> {
    cancel_approve_notification_with_options(config, notification_id, RequestOption::default())
        .await
}

/// 执行取消推送通知审核（支持自定义选项）
pub async fn cancel_approve_notification_with_options(
    config: &Config,
    notification_id: String,
    option: RequestOption,
) -> SDKResult<CancelApproveNotificationResponse> {
    let req: ApiRequest<CancelApproveNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationCancelApprove(notification_id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "取消推送通知审核")
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
            CancelApproveNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
    }
}
