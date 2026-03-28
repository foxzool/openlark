//! 删除动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/delete>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::DeleteActivitySubscriptionResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 删除动态订阅请求
#[derive(Debug, Clone)]
pub struct DeleteActivitySubscriptionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 订阅 GUID
    subscription_guid: String,
}

impl DeleteActivitySubscriptionRequest {
    pub fn new(
        config: Arc<Config>,
        tasklist_guid: impl Into<String>,
        subscription_guid: impl Into<String>,
    ) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            subscription_guid: subscription_guid.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteActivitySubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteActivitySubscriptionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.subscription_guid.trim(), "订阅GUID不能为空");

        let api_endpoint = TaskApiV2::ActivitySubscriptionDelete(
            self.tasklist_guid.clone(),
            self.subscription_guid.clone(),
        );
        let request =
            ApiRequest::<DeleteActivitySubscriptionResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除动态订阅")
    }
}

impl ApiResponseTrait for DeleteActivitySubscriptionResponse {
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
    fn test_delete_activity_subscription_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = DeleteActivitySubscriptionRequest::new(
            Arc::new(config),
            "tasklist_123",
            "subscription_456",
        );

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.subscription_guid, "subscription_456");
    }

    #[test]
    fn test_activity_subscription_delete_api_v2_url() {
        let endpoint = TaskApiV2::ActivitySubscriptionDelete(
            "tasklist_123".to_string(),
            "subscription_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/activity_subscriptions/subscription_456"
        );
    }
}
