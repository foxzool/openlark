//! 更新动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::{
    UpdateActivitySubscriptionBody, UpdateActivitySubscriptionResponse,
};
use crate::v2::tasklist::models::TasklistMemberDetail;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 更新动态订阅请求
#[derive(Debug, Clone)]
pub struct UpdateActivitySubscriptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 订阅 GUID
    subscription_guid: String,
    /// 请求体
    body: UpdateActivitySubscriptionBody,
    /// 用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    user_id_type: Option<String>,
}

impl UpdateActivitySubscriptionRequest {
    /// 创建新的请求构建器。
    pub fn new(
        config: Arc<Config>,
        tasklist_guid: impl Into<String>,
        subscription_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            subscription_guid: subscription_guid.into(),
            body: UpdateActivitySubscriptionBody::default(),
            user_id_type: None,
        }
    }

    /// 将字段名加入 update_fields（去重）
    fn push_update_field(&mut self, field: &str) {
        if !self.body.update_fields.iter().any(|f| f == field) {
            self.body.update_fields.push(field.to_string());
        }
    }

    /// 设置订阅名称，并将 name 加入 update_fields
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.activity_subscription.name = Some(name.into());
        self.push_update_field("name");
        self
    }

    /// 设置订阅者列表（最大 50 个），并将 subscribers 加入 update_fields
    pub fn subscribers(mut self, subscribers: Vec<TasklistMemberDetail>) -> Self {
        self.body.activity_subscription.subscribers = Some(subscribers);
        self.push_update_field("subscribers");
        self
    }

    /// 设置订阅的清单动态 event key 列表，并将 include_keys 加入 update_fields
    pub fn include_keys(mut self, include_keys: Vec<i32>) -> Self {
        self.body.activity_subscription.include_keys = Some(include_keys);
        self.push_update_field("include_keys");
        self
    }

    /// 设置该订阅是否停用，并将 disabled 加入 update_fields
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.body.activity_subscription.disabled = Some(disabled);
        self.push_update_field("disabled");
        self
    }

    /// 显式设置要更新的字段名列表（覆盖 Builder 自动收集的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateActivitySubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateActivitySubscriptionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.subscription_guid.trim(), "订阅GUID不能为空");
        validate_required!(self.body.update_fields, "update_fields 不能为空");

        let api_endpoint = TaskApiV2::ActivitySubscriptionUpdate(
            self.tasklist_guid.clone(),
            self.subscription_guid.clone(),
        );
        let mut request =
            ApiRequest::<UpdateActivitySubscriptionResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新动态订阅")?);
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新动态订阅",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateActivitySubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_update_activity_subscription_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            UpdateActivitySubscriptionRequest::new(config, "tasklist_123", "subscription_456")
                .name("Roadmap订阅")
                .include_keys(vec![101])
                .user_id_type("open_id");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.subscription_guid, "subscription_456");
        assert_eq!(
            request.body.activity_subscription.name,
            Some("Roadmap订阅".to_string())
        );
        assert_eq!(
            request.body.activity_subscription.include_keys,
            Some(vec![101])
        );
        assert_eq!(request.body.update_fields, vec!["name", "include_keys"]);
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));

        // 官方形状：activity_subscription 对象 + update_fields 数组
        let json = serde_json::to_value(&request.body).unwrap();
        assert_eq!(json["activity_subscription"]["name"], "Roadmap订阅");
        assert_eq!(
            json["update_fields"],
            serde_json::json!(["name", "include_keys"])
        );
    }

    #[test]
    fn test_activity_subscription_update_api_v2_url() {
        let endpoint = TaskApiV2::ActivitySubscriptionUpdate(
            "tasklist_123".to_string(),
            "subscription_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/activity_subscriptions/subscription_456"
        );
    }
}
