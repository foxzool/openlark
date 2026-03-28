//! 获取任务提醒列表（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskreminder/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务提醒列表项（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskReminderItemV1 {
    /// 提醒 ID
    pub reminder_id: String,
    /// 提醒时间（Unix 时间戳）
    pub trigger_time: i64,
    /// 提醒类型
    pub type_: Option<String>,
}

/// 获取任务提醒列表响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskReminderResponseV1 {
    /// 提醒列表
    pub items: Vec<TaskReminderItemV1>,
}

/// 获取任务提醒列表请求（v1）
#[derive(Debug, Clone)]
pub struct ListTaskReminderRequestV1 {
    config: Arc<Config>,
    task_id: String,
}

impl ListTaskReminderRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskReminderResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskReminderResponseV1> {
        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskReminderList(self.task_id.clone());
        let request = ApiRequest::<ListTaskReminderResponseV1>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListTaskReminderResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_list_task_reminder_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskReminderList("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/reminders"
        );
    }
}
