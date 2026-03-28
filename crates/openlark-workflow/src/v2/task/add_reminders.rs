//! 添加任务提醒
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-add_reminders/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 添加任务提醒请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddRemindersBody {
    /// 提醒时间列表（ISO 8601 格式）
    pub reminders: Vec<String>,
}

/// 任务提醒信息
#[derive(Debug, Clone, Deserialize)]
pub struct TaskReminder {
    /// 提醒 GUID
    pub reminder_guid: String,
    /// 提醒时间
    pub remind_at: String,
    /// 创建时间
    pub created_at: String,
}

/// 添加任务提醒响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddRemindersResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 添加的提醒列表
    #[serde(default)]
    pub reminders: Vec<TaskReminder>,
}

/// 添加任务提醒请求
#[derive(Debug, Clone)]
pub struct AddRemindersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: AddRemindersBody,
}

impl AddRemindersRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: AddRemindersBody::default(),
        }
    }

    /// 设置提醒时间列表
    pub fn reminders(mut self, reminders: Vec<String>) -> Self {
        self.body.reminders = reminders;
        self
    }

    /// 添加单个提醒时间
    pub fn add_reminder(mut self, reminder: impl Into<String>) -> Self {
        self.body.reminders.push(reminder.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddRemindersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddRemindersResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.reminders, "提醒时间列表不能为空");

        let api_endpoint = TaskApiV2::TaskAddReminders(self.task_guid.clone());
        let mut request = ApiRequest::<AddRemindersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "添加任务提醒")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "添加任务提醒")
    }
}

impl ApiResponseTrait for AddRemindersResponse {
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
    fn test_add_reminders_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddRemindersRequest::new(config, "task_123")
            .reminders(vec!["2024-01-01T09:00:00Z".to_string()]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.reminders, vec!["2024-01-01T09:00:00Z"]);
    }

    #[test]
    fn test_add_reminder_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddRemindersRequest::new(config, "task_123")
            .add_reminder("2024-01-01T09:00:00Z")
            .add_reminder("2024-01-02T09:00:00Z");

        assert_eq!(
            request.body.reminders,
            vec!["2024-01-01T09:00:00Z", "2024-01-02T09:00:00Z"]
        );
    }

    #[test]
    fn test_task_add_reminders_api_v2_url() {
        let endpoint = TaskApiV2::TaskAddReminders("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/add_reminders"
        );
    }
}
