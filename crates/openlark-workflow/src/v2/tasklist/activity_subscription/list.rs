//! 列取动态订阅
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist-activity_subscription/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::activity_subscription::models::ListActivitySubscriptionsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 列取动态订阅请求
#[derive(Debug, Clone)]
pub struct ListActivitySubscriptionsRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListActivitySubscriptionsRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: impl Into<String>) -> Self {
        Self {
            config,
            tasklist_guid: tasklist_guid.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListActivitySubscriptionsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListActivitySubscriptionsResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");

        let api_endpoint = TaskApiV2::ActivitySubscriptionList(self.tasklist_guid.clone());
        let mut request =
            ApiRequest::<ListActivitySubscriptionsResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取动态订阅")
    }
}

impl ApiResponseTrait for ListActivitySubscriptionsResponse {
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
    fn test_list_activity_subscriptions_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = ListActivitySubscriptionsRequest::new(Arc::new(config), "tasklist_123")
            .page_size(20)
            .page_token("next_page_token");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_activity_subscription_list_api_v2_url() {
        let endpoint = TaskApiV2::ActivitySubscriptionList("tasklist_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasklists/tasklist_123/activity_subscriptions"
        );
    }
}
