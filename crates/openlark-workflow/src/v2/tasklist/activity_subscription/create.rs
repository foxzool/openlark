//! 创建动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::{
    ActivitySubscriptionTargetType, ActivitySubscriptionType, CreateActivitySubscriptionBody,
    CreateActivitySubscriptionResponse,
};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
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
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            body: CreateActivitySubscriptionBody::default(),
        }
    }

    /// 设置订阅类型
    pub fn subscription_type(mut self, subscription_type: ActivitySubscriptionType) -> Self {
        self.body.subscription_type = subscription_type;
        self
    }

    /// 设置目标类型
    pub fn target_type(mut self, target_type: ActivitySubscriptionTargetType) -> Self {
        self.body.target_type = target_type;
        self
    }

    /// 设置目标 URL（当 target_type 为 webhook 时）
    pub fn target_url(mut self, target_url: impl Into<String>) -> Self {
        self.body.target_url = Some(target_url.into());
        self
    }

    /// 设置飞书群 ID（当 target_type 为 chat 时）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.body.chat_id = Some(chat_id.into());
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

        let api_endpoint = TaskApiV2::ActivitySubscriptionCreate(self.tasklist_guid.clone());
        let mut request =
            ApiRequest::<CreateActivitySubscriptionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建动态订阅")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建动态订阅")
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
            .subscription_type(ActivitySubscriptionType::TaskCreated)
            .target_type(ActivitySubscriptionTargetType::Webhook)
            .target_url("https://example.com/webhook");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(
            request.body.subscription_type,
            ActivitySubscriptionType::TaskCreated
        );
        assert_eq!(
            request.body.target_type,
            ActivitySubscriptionTargetType::Webhook
        );
        assert_eq!(
            request.body.target_url,
            Some("https://example.com/webhook".to_string())
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
