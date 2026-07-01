//! 订阅任务变更事件。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task_v2/task_subscription>

use crate::common::{TaskV2Endpoint, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 订阅任务变更事件响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskSubscriptionResponse {
    /// 状态码。
    pub code: i32,
    /// 响应信息。
    pub msg: String,
}

/// 订阅任务变更事件请求。
#[derive(Debug, Clone)]
pub struct TaskSubscriptionRequest {
    config: Arc<Config>,
    user_id_type: Option<String>,
}

impl TaskSubscriptionRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<TaskSubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TaskSubscriptionResponse> {
        let api_endpoint = TaskV2Endpoint::TaskSubscription;
        let mut request = ApiRequest::<TaskSubscriptionResponse>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "订阅任务变更事件")
    }
}

impl ApiResponseTrait for TaskSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_194_task_subscription_builder() {
        let request =
            TaskSubscriptionRequest::new(Arc::new(Config::default())).user_id_type("open_id");

        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
