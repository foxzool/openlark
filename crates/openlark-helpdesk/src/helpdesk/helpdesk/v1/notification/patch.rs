//! 更新指定推送通知
//!
//! 更新指定推送通知的信息。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/patch>

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
use crate::helpdesk::helpdesk::v1::notification::models::{
    NotificationChat, NotificationDepartment, NotificationUser,
};

/// 更新推送通知请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchNotificationBody {
    /// 推送任务 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 推送任务名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    /// 推送任务状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建人。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_user: Option<NotificationUser>,
    /// 创建时间，毫秒时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新人。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_user: Option<NotificationUser>,
    /// 更新时间，毫秒时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 目标用户数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_count: Option<i32>,
    /// 已推送用户数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_user_count: Option<i32>,
    /// 已读用户数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_user_count: Option<i32>,
    /// 推送触发时间，毫秒时间戳。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_at: Option<String>,
    /// 推送内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_content: Option<String>,
    /// 推送类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_type: Option<i32>,
    /// 推送范围类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_scope_type: Option<i32>,
    /// 新员工入职范围类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_staff_scope_type: Option<i32>,
    /// 新员工入职生效部门列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_staff_scope_department_list: Option<Vec<NotificationDepartment>>,
    /// 推送用户列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<NotificationUser>>,
    /// 推送部门列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_list: Option<Vec<NotificationDepartment>>,
    /// 推送群聊列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_list: Option<Vec<NotificationChat>>,
    /// 扩展字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}

impl PatchNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        Ok(())
    }
}

/// 更新推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchNotificationResponse {}

impl ApiResponseTrait for PatchNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新推送通知请求
#[derive(Debug, Clone)]
pub struct PatchNotificationRequest {
    config: Arc<Config>,
    notification_id: String,
}

impl PatchNotificationRequest {
    /// 创建新的更新推送通知请求
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
        }
    }

    /// 执行更新推送通知请求
    pub async fn execute(
        self,
        body: PatchNotificationBody,
    ) -> SDKResult<PatchNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行更新推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: PatchNotificationBody,
        option: RequestOption,
    ) -> SDKResult<PatchNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<PatchNotificationResponse> = ApiRequest::patch(
            HelpdeskApiV1::NotificationPatch(self.notification_id.clone()).to_url(),
        )
        .body(serialize_params(&body, "更新推送通知")?);

        Transport::request_typed(req, &self.config, Some(option), "更新推送通知").await
    }
}

/// 更新推送通知请求构建器
#[derive(Debug, Clone)]
pub struct PatchNotificationRequestBuilder {
    config: Arc<Config>,
    notification_id: String,
    body: PatchNotificationBody,
}

impl PatchNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>, notification_id: String) -> Self {
        Self {
            config,
            notification_id,
            body: PatchNotificationBody::default(),
        }
    }

    /// 设置推送任务名称。
    pub fn job_name(mut self, job_name: impl Into<String>) -> Self {
        self.body.job_name = Some(job_name.into());
        self
    }

    /// 设置推送内容。
    pub fn push_content(mut self, push_content: impl Into<String>) -> Self {
        self.body.push_content = Some(push_content.into());
        self
    }

    /// 设置推送类型。
    pub fn push_type(mut self, push_type: i32) -> Self {
        self.body.push_type = Some(push_type);
        self
    }

    /// 设置推送范围类型。
    pub fn push_scope_type(mut self, push_scope_type: i32) -> Self {
        self.body.push_scope_type = Some(push_scope_type);
        self
    }

    /// 设置新员工入职范围类型。
    pub fn new_staff_scope_type(mut self, new_staff_scope_type: i32) -> Self {
        self.body.new_staff_scope_type = Some(new_staff_scope_type);
        self
    }

    /// 设置新员工入职生效部门列表。
    pub fn new_staff_scope_department_list(
        mut self,
        departments: Vec<NotificationDepartment>,
    ) -> Self {
        self.body.new_staff_scope_department_list = Some(departments);
        self
    }

    /// 设置推送用户列表。
    pub fn user_list(mut self, users: Vec<NotificationUser>) -> Self {
        self.body.user_list = Some(users);
        self
    }

    /// 设置推送部门列表。
    pub fn department_list(mut self, departments: Vec<NotificationDepartment>) -> Self {
        self.body.department_list = Some(departments);
        self
    }

    /// 设置推送群聊列表。
    pub fn chat_list(mut self, chats: Vec<NotificationChat>) -> Self {
        self.body.chat_list = Some(chats);
        self
    }

    /// 设置扩展字段。
    pub fn ext(mut self, ext: impl Into<String>) -> Self {
        self.body.ext = Some(ext.into());
        self
    }

    /// 构建请求体。
    pub fn body(&self) -> PatchNotificationBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<PatchNotificationResponse> {
        let body = self.body();
        let request =
            PatchNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<PatchNotificationResponse> {
        let body = self.body();
        let request =
            PatchNotificationRequest::new(self.config.clone(), self.notification_id.clone());
        request.execute_with_options(body, option).await
    }
}

/// 执行更新推送通知
pub async fn patch_notification(
    config: &Config,
    notification_id: String,
    body: PatchNotificationBody,
) -> SDKResult<PatchNotificationResponse> {
    patch_notification_with_options(config, notification_id, body, RequestOption::default()).await
}

/// 执行更新推送通知（支持自定义选项）
pub async fn patch_notification_with_options(
    config: &Config,
    notification_id: String,
    body: PatchNotificationBody,
    option: RequestOption,
) -> SDKResult<PatchNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<PatchNotificationResponse> =
        ApiRequest::patch(HelpdeskApiV1::NotificationPatch(notification_id).to_url())
            .body(serialize_params(&body, "更新推送通知")?);

    Transport::request_typed(req, config, Some(option), "更新推送通知").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_empty() {
        let body = PatchNotificationBody::default();
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_validation_valid() {
        let body = PatchNotificationBody {
            job_name: Some("新任务名称".to_string()),
            push_content: Some("新推送内容".to_string()),
            ..Default::default()
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_uses_official_flat_fields() {
        let body = PatchNotificationBody {
            job_name: Some("新任务名称".to_string()),
            push_scope_type: Some(2),
            chat_list: Some(vec![NotificationChat {
                chat_id: Some("oc_001".to_string()),
                name: Some("测试群".to_string()),
            }]),
            ..Default::default()
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["job_name"], "新任务名称");
        assert_eq!(value["push_scope_type"], 2);
        assert_eq!(value["chat_list"][0]["chat_id"], "oc_001");
        assert!(value.get("title").is_none());
        assert!(value.get("content").is_none());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let builder =
            PatchNotificationRequestBuilder::new(Arc::new(config), "notif_123".to_string());

        assert_eq!(builder.notification_id, "notif_123");
        assert!(builder.body.job_name.is_none());
    }

    /// 端到端：PATCH .../notifications/{id} → 空对象 `data` 解析成功。
    #[tokio::test]
    async fn test_patch_notification_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/helpdesk/v1/notifications/ntf_001"))
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

        let body = PatchNotificationBody {
            job_name: Some("新任务名称".to_string()),
            push_content: Some("新推送内容".to_string()),
            ..Default::default()
        };
        PatchNotificationRequest::new(config, "ntf_001".to_string())
            .execute(body)
            .await
            .expect("更新推送通知应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications/ntf_001"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["job_name"], "新任务名称");
        assert!(request_body.get("title").is_none());
    }
}
