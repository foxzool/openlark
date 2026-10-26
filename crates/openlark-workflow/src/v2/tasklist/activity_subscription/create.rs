//! 创建动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::{
    CreateActivitySubscriptionBody, CreateActivitySubscriptionResponse,
};
use crate::v2::tasklist::models::TasklistMemberDetail;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use std::sync::Arc;

/// 创建动态订阅请求
#[derive(Debug, Clone)]
pub struct CreateActivitySubscriptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: CreateActivitySubscriptionBody,
}

impl CreateActivitySubscriptionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            body: Default::default(),
        }
    }

    /// 设置订阅名称（必填，最大 50 个字符）
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.name = name.into();
        self
    }

    /// 设置订阅者列表（必填，最大 50 个；member 目前仅支持 chat 类型）
    pub fn subscribers(mut self, subscribers: Vec<TasklistMemberDetail>) -> Self {
        self.body.subscribers = subscribers;
        self
    }

    /// 设置订阅的清单动态 event key 列表（必填，可为空数组；元素不可重复）
    pub fn include_keys(mut self, include_keys: Vec<i32>) -> Self {
        self.body.include_keys = include_keys;
        self
    }

    /// 设置该订阅是否停用
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.body.disabled = Some(disabled);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateActivitySubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateActivitySubscriptionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.name.trim(), "订阅名称不能为空");
        validate_required_list!(
            self.body.subscribers,
            50,
            "subscribers 不能为空且不能超过 50 个"
        );
        // include_keys 必填但允许为空数组（表示不通知任何动态），不做非空校验

        let api_endpoint = TaskApiV2::ActivitySubscriptionCreate(self.tasklist_guid.clone());
        let mut request =
            ApiRequest::<CreateActivitySubscriptionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建动态订阅")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建动态订阅",
        )
        .await
    }
}

impl ApiResponseTrait for CreateActivitySubscriptionResponse {
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
    fn test_create_activity_subscription_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateActivitySubscriptionRequest::new(config, "tasklist_123")
            .name("我的订阅")
            .subscribers(vec![TasklistMemberDetail {
                id: Some("oc_2cefb2f014f8d0c6c2d2eb7bafb0e54f".to_string()),
                r#type: Some("chat".to_string()),
                ..Default::default()
            }])
            .include_keys(vec![100, 103]);

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.name, "我的订阅");
        assert_eq!(request.body.subscribers.len(), 1);
        assert_eq!(request.body.include_keys, vec![100, 103]);

        // 必填字段以官方形状序列化：name / subscribers / include_keys 位于顶层
        let json = serde_json::to_value(&request.body).unwrap();
        assert_eq!(json["name"], "我的订阅");
        assert_eq!(json["include_keys"], serde_json::json!([100, 103]));
        assert_eq!(
            json["subscribers"][0]["id"],
            "oc_2cefb2f014f8d0c6c2d2eb7bafb0e54f"
        );
    }

    #[test]
    fn test_activity_subscription_create_api_v2_url() {
        let endpoint = TaskApiV2::ActivitySubscriptionCreate("tasklist_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/activity_subscriptions"
        );
    }
}
