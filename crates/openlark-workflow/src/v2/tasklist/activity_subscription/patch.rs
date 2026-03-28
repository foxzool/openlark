//! 更新动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::{
    ActivitySubscriptionTargetType, ActivitySubscriptionType, UpdateActivitySubscriptionBody,
    UpdateActivitySubscriptionResponse,
};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
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
}

impl UpdateActivitySubscriptionRequest {
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
        }
    }

    /// 设置订阅类型
    pub fn subscription_type(mut self, subscription_type: ActivitySubscriptionType) -> Self {
        self.body.subscription_type = Some(subscription_type);
        self
    }

    /// 设置目标类型
    pub fn target_type(mut self, target_type: ActivitySubscriptionTargetType) -> Self {
        self.body.target_type = Some(target_type);
        self
    }

    /// 设置目标 URL
    pub fn target_url(mut self, target_url: impl Into<String>) -> Self {
        self.body.target_url = Some(target_url.into());
        self
    }

    /// 设置飞书群 ID
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.body.chat_id = Some(chat_id.into());
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

        let api_endpoint = TaskApiV2::ActivitySubscriptionUpdate(
            self.tasklist_guid.clone(),
            self.subscription_guid.clone(),
        );
        let mut request =
            ApiRequest::<UpdateActivitySubscriptionResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新动态订阅")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新动态订阅")
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
                .subscription_type(ActivitySubscriptionType::TaskUpdated)
                .target_type(ActivitySubscriptionTargetType::Chat)
                .chat_id("chat_789");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.subscription_guid, "subscription_456");
        assert_eq!(
            request.body.subscription_type,
            Some(ActivitySubscriptionType::TaskUpdated)
        );
        assert_eq!(
            request.body.target_type,
            Some(ActivitySubscriptionTargetType::Chat)
        );
        assert_eq!(request.body.chat_id, Some("chat_789".to_string()));
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
