//! 提交推送通知审核
//!
//! 提交推送通知审核。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/submit_approve>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_endpoints::HelpdeskApiV1;
use crate::common::api_utils::extract_response_data;

/// 提交推送通知审核响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApproveNotificationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<SubmitApproveNotificationResult>,
}

impl openlark_core::api::ApiResponseTrait for SubmitApproveNotificationResponse {}

/// 提交推送通知审核结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApproveNotificationResult {
    /// 是否提交成功
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

/// 提交推送通知审核请求
#[derive(Debug, Clone)]
pub struct SubmitApproveNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl SubmitApproveNotificationRequest {
    /// 创建新的提交推送通知审核请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行提交推送通知审核请求
    pub async fn execute(self) -> SDKResult<SubmitApproveNotificationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行提交推送通知审核请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SubmitApproveNotificationResponse> {
        let req: ApiRequest<SubmitApproveNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationSubmitApprove(self.notification_id.clone()).to_url(),
        );

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "提交推送通知审核")
    }
}

/// 提交推送通知审核请求构建器
#[derive(Debug, Clone)]
pub struct SubmitApproveNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
}

impl SubmitApproveNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<SubmitApproveNotificationResponse> {
        let request = SubmitApproveNotificationRequest::new(
            self.config.clone(),
            self.notification_id.clone(),
        );
        request.execute().await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<SubmitApproveNotificationResponse> {
        let request = SubmitApproveNotificationRequest::new(
            self.config.clone(),
            self.notification_id.clone(),
        );
        request.execute_with_options(option).await
    }
}

/// 执行提交推送通知审核
pub async fn submit_approve_notification(
    config: &Config,
    notification_id: String,
) -> SDKResult<SubmitApproveNotificationResponse> {
    submit_approve_notification_with_options(config, notification_id, RequestOption::default())
        .await
}

/// 执行提交推送通知审核（支持自定义选项）
pub async fn submit_approve_notification_with_options(
    config: &Config,
    notification_id: String,
    option: RequestOption,
) -> SDKResult<SubmitApproveNotificationResponse> {
    let req: ApiRequest<SubmitApproveNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationSubmitApprove(notification_id).to_url());

    let resp = Transport::request(req, config, Some(option)).await?;
    extract_response_data(resp, "提交推送通知审核")
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
            SubmitApproveNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
    }
}
