//! 创建推送通知
//!
//! 创建推送通知。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/helpdesk-v1/notification/create>

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

/// 创建推送通知请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateNotificationBody {
    /// 推送任务 ID，创建成功后由服务端返回。
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

impl CreateNotificationBody {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        Ok(())
    }
}

/// 创建推送通知响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotificationResponse {
    /// 推送任务 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    /// 推送任务状态。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl ApiResponseTrait for CreateNotificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建推送通知请求
#[derive(Debug, Clone)]
pub struct CreateNotificationRequest {
    config: Arc<Config>,
}

impl CreateNotificationRequest {
    /// 创建新的创建推送通知请求
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行创建推送通知请求
    pub async fn execute(
        self,
        body: CreateNotificationBody,
    ) -> SDKResult<CreateNotificationResponse> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行创建推送通知请求（支持自定义选项）
    pub async fn execute_with_options(
        self,
        body: CreateNotificationBody,
        option: RequestOption,
    ) -> SDKResult<CreateNotificationResponse> {
        body.validate()?;

        let req: ApiRequest<CreateNotificationResponse> =
            ApiRequest::post(HelpdeskApiV1::NotificationCreate.to_url())
                .body(serialize_params(&body, "创建推送通知")?);

        Transport::request_typed(req, &self.config, Some(option), "创建推送通知").await
    }
}

/// 创建推送通知请求构建器
#[derive(Debug, Clone)]
pub struct CreateNotificationRequestBuilder {
    config: Arc<Config>,
    body: CreateNotificationBody,
}

impl CreateNotificationRequestBuilder {
    /// 创建新的构建器
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateNotificationBody::default(),
        }
    }

    /// 设置推送任务 ID。
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.body.id = Some(id.into());
        self
    }

    /// 设置推送任务名称。
    pub fn job_name(mut self, job_name: impl Into<String>) -> Self {
        self.body.job_name = Some(job_name.into());
        self
    }

    /// 设置推送任务状态。
    pub fn status(mut self, status: i32) -> Self {
        self.body.status = Some(status);
        self
    }

    /// 设置创建人。
    pub fn create_user(mut self, create_user: NotificationUser) -> Self {
        self.body.create_user = Some(create_user);
        self
    }

    /// 设置创建时间。
    pub fn created_at(mut self, created_at: impl Into<String>) -> Self {
        self.body.created_at = Some(created_at.into());
        self
    }

    /// 设置更新人。
    pub fn update_user(mut self, update_user: NotificationUser) -> Self {
        self.body.update_user = Some(update_user);
        self
    }

    /// 设置更新时间。
    pub fn updated_at(mut self, updated_at: impl Into<String>) -> Self {
        self.body.updated_at = Some(updated_at.into());
        self
    }

    /// 设置目标用户数。
    pub fn target_user_count(mut self, target_user_count: i32) -> Self {
        self.body.target_user_count = Some(target_user_count);
        self
    }

    /// 设置已推送用户数。
    pub fn sent_user_count(mut self, sent_user_count: i32) -> Self {
        self.body.sent_user_count = Some(sent_user_count);
        self
    }

    /// 设置已读用户数。
    pub fn read_user_count(mut self, read_user_count: i32) -> Self {
        self.body.read_user_count = Some(read_user_count);
        self
    }

    /// 设置推送触发时间。
    pub fn send_at(mut self, send_at: impl Into<String>) -> Self {
        self.body.send_at = Some(send_at.into());
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
    pub fn body(&self) -> CreateNotificationBody {
        self.body.clone()
    }

    /// 执行请求
    pub async fn execute(&self) -> SDKResult<CreateNotificationResponse> {
        let body = self.body();
        let request = CreateNotificationRequest::new(self.config.clone());
        request.execute(body).await
    }

    /// 执行请求（支持自定义选项）。
    pub async fn execute_with_options(
        &self,
        option: RequestOption,
    ) -> SDKResult<CreateNotificationResponse> {
        let request = CreateNotificationRequest::new(self.config.clone());
        request.execute_with_options(self.body(), option).await
    }
}

/// 执行创建推送通知
pub async fn create_notification(
    config: &Config,
    body: CreateNotificationBody,
) -> SDKResult<CreateNotificationResponse> {
    create_notification_with_options(config, body, RequestOption::default()).await
}

/// 执行创建推送通知（支持自定义选项）
pub async fn create_notification_with_options(
    config: &Config,
    body: CreateNotificationBody,
    option: RequestOption,
) -> SDKResult<CreateNotificationResponse> {
    body.validate()?;

    let req: ApiRequest<CreateNotificationResponse> =
        ApiRequest::post(HelpdeskApiV1::NotificationCreate.to_url())
            .body(serialize_params(&body, "创建推送通知")?);

    Transport::request_typed(req, config, Some(option), "创建推送通知").await
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_body_validation_valid() {
        let body = CreateNotificationBody {
            job_name: Some("系统维护通知".to_string()),
            push_content: Some(r#"{"elements":[]}"#.to_string()),
            push_type: Some(0),
            push_scope_type: Some(2),
            user_list: Some(vec![NotificationUser {
                user_id: Some("ou_001".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let result = body.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_body_uses_official_flat_fields() {
        let body = CreateNotificationBody {
            job_name: Some("系统维护通知".to_string()),
            push_content: Some("卡片内容".to_string()),
            push_type: Some(0),
            ..Default::default()
        };
        let value = serde_json::to_value(body).expect("请求体应可序列化");

        assert_eq!(value["job_name"], "系统维护通知");
        assert_eq!(value["push_content"], "卡片内容");
        assert_eq!(value["push_type"], 0);
        assert!(value.get("title").is_none());
        assert!(value.get("content").is_none());
    }

    #[test]
    fn test_builder_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let body = CreateNotificationRequestBuilder::new(Arc::new(config))
            .job_name("系统维护通知")
            .push_content("卡片内容")
            .push_type(0)
            .push_scope_type(0)
            .body();

        assert_eq!(body.job_name.as_deref(), Some("系统维护通知"));
        assert_eq!(body.push_type, Some(0));
    }

    /// 端到端：POST .../notifications → 响应直接解析 `data` 内的字段。
    #[tokio::test]
    async fn test_create_notification_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/helpdesk/v1/notifications"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "notification_id": "ntf_001", "status": 0 }
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

        let body = CreateNotificationBody {
            job_name: Some("系统维护通知".to_string()),
            push_content: Some("系统将于今晚维护".to_string()),
            push_type: Some(0),
            push_scope_type: Some(0),
            ..Default::default()
        };
        let resp = CreateNotificationRequest::new(config)
            .execute(body)
            .await
            .expect("创建推送通知应成功");
        assert_eq!(resp.notification_id.as_deref(), Some("ntf_001"));
        assert_eq!(resp.status, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/helpdesk/v1/notifications"
        );
        let request_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(request_body["job_name"], "系统维护通知");
        assert!(request_body.get("title").is_none());
    }
}
