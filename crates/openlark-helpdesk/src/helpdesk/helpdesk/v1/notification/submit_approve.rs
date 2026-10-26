//! 提交推送通知审核
//!
//! 提交推送通知审核。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/submit_approve>

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

/// 提交推送通知审核请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApproveNotificationBody {
    /// 提交审批理由
    pub reason: String,
}

impl SubmitApproveNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        openlark_core::validate_required!(self.reason, "reason 不能为空");
        Ok(())
    }
}

/// 提交推送通知审核响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApproveNotificationResponse {
    /// 是否有权限创建或者管理审批流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_access: Option<bool>,
}

impl ApiResponseTrait for SubmitApproveNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
    pub async fn execute(
        self,
        body: SubmitApproveNotificationBody,
    ) -> SDKResult<SubmitApproveNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行提交推送通知审核请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: SubmitApproveNotificationBody,
        option: RequestOption,
    ) -> SDKResult<SubmitApproveNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<SubmitApproveNotificationResponse> = ApiRequest::post(
            HelpdeskApiV1::NotificationSubmitApprove(self.notification_id.clone()).to_url(),
        )
        .body(serialize_params(&body, "提交推送通知审核")?);

        Transport::request_typed(req, &self.config, Some(option), "提交推送通知审核").await
    }
}

/// 提交推送通知审核请求构建器
#[derive(Debug, Clone)]
pub struct SubmitApproveNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
    body: SubmitApproveNotificationBody,
}

impl SubmitApproveNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
            body: SubmitApproveNotificationBody {
                reason: String::new(),
            },
        }
    }

    /// 设置提交审批理由
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.body.reason = reason.into();
        self
    }

    /// 构建请求体
    pub fn body(&self) -> SubmitApproveNotificationBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<SubmitApproveNotificationResponse> {
        let request = SubmitApproveNotificationRequest::new(
            self.config.clone(),
            self.notification_id.clone(),
        );
        request.execute(self.body()).await
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
        request.execute_with_options(self.body(), option).await
    }
}

/// 执行提交推送通知审核
pub async fn submit_approve_notification(
    config: &Config,
    notification_id: String,
    body: SubmitApproveNotificationBody,
) -> SDKResult<SubmitApproveNotificationResponse> {
    submit_approve_notification_with_options(
        config,
        notification_id,
        body,
        RequestOption::default(),
    )
    .await
}

/// 执行提交推送通知审核（支持自定义选项）
pub async fn submit_approve_notification_with_options(
    config: &Config,
    notification_id: String,
    body: SubmitApproveNotificationBody,
    option: RequestOption,
) -> SDKResult<SubmitApproveNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<SubmitApproveNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationSubmitApprove(notification_id).to_url())
            .body(serialize_params(&body, "提交推送通知审核")?);

    Transport::request_typed(req, config, Some(option), "提交推送通知审核").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_rejects_blank_reason() {
        let body = SubmitApproveNotificationBody {
            reason: "  ".to_string(),
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn test_body_serializes_official_shape() {
        let body = SubmitApproveNotificationBody {
            reason: "测试发送消息".to_string(),
        };
        let value = serde_json::to_value(&body).expect("请求体应可序列化");

        assert_eq!(value["reason"], "测试发送消息");
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            SubmitApproveNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string())
                .reason("测试发送消息");

        assert_eq!(builder.notification_id, "notif_123");
        assert_eq!(builder.body().reason, "测试发送消息");
    }

    /// 端到端：POST .../notifications/{id}/submit_approve（body: reason）→ 响应解析 `data` 内的 has_access。
    #[tokio::test]
    async fn test_submit_approve_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/helpdesk/v1/notifications/ntf_001/submit_approve",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "has_access": true }
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

        let resp = SubmitApproveNotificationRequest::new(config, "ntf_001".to_string())
            .execute(SubmitApproveNotificationBody {
                reason: "测试发送消息".to_string(),
            })
            .await
            .expect("提交推送通知审核应成功");
        assert_eq!(resp.has_access, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications/ntf_001/submit_approve"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["reason"], "测试发送消息");
    }
}
