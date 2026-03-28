//! 移除任务提醒
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-remove_reminders/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 移除任务提醒请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct RemoveRemindersBody {
    /// 提醒 GUID 列表
    pub reminder_guids: Vec<String>,
}

/// 移除任务提醒响应
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveRemindersResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 移除的提醒 GUID 列表
    #[serde(default)]
    pub removed_reminders: Vec<String>,
}

/// 移除任务提醒请求
#[derive(Debug, Clone)]
pub struct RemoveRemindersRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: RemoveRemindersBody,
}

impl RemoveRemindersRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: RemoveRemindersBody::default(),
        }
    }

    /// 设置提醒 GUID 列表
    pub fn reminder_guids(mut self, reminder_guids: Vec<String>) -> Self {
        self.body.reminder_guids = reminder_guids;
        self
    }

    /// 移除单个提醒
    pub fn remove_reminder(mut self, reminder_guid: impl Into<String>) -> Self {
        self.body.reminder_guids.push(reminder_guid.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveRemindersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveRemindersResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.reminder_guids, "提醒GUID列表不能为空");

        let api_endpoint = TaskApiV2::TaskRemoveReminders(self.task_guid.clone());
        let mut request = ApiRequest::<RemoveRemindersResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "移除任务提醒")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "移除任务提醒")
    }
}

impl ApiResponseTrait for RemoveRemindersResponse {
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
    fn test_remove_reminders_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveRemindersRequest::new(config, "task_123")
            .reminder_guids(vec!["reminder_1".to_string(), "reminder_2".to_string()]);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(
            request.body.reminder_guids,
            vec!["reminder_1", "reminder_2"]
        );
    }

    #[test]
    fn test_remove_reminder_single() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = RemoveRemindersRequest::new(config, "task_123")
            .remove_reminder("reminder_1")
            .remove_reminder("reminder_2");

        assert_eq!(
            request.body.reminder_guids,
            vec!["reminder_1", "reminder_2"]
        );
    }

    #[test]
    fn test_task_remove_reminders_api_v2_url() {
        let endpoint = TaskApiV2::TaskRemoveReminders("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/remove_reminders"
        );
    }
}
