//! 创建任务提醒（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskreminder/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务提醒请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskReminderBodyV1 {
    /// 提醒时间（Unix 时间戳）
    pub trigger_time: i64,
    /// 提醒类型
    pub type_: Option<String>,
}

/// 创建任务提醒响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskReminderResponseV1 {
    /// 提醒 ID
    pub reminder_id: String,
}

/// 创建任务提醒请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskReminderRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskReminderBodyV1,
}

impl CreateTaskReminderRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskReminderBodyV1::default(),
        }
    }

    /// 设置提醒时间（Unix 时间戳）
    pub fn trigger_time(mut self, trigger_time: i64) -> Self {
        self.body.trigger_time = trigger_time;
        self
    }

    /// 设置提醒类型
    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.body.type_ = Some(type_.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskReminderResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskReminderResponseV1> {
        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskReminderCreate(self.task_id.clone());
        let mut request = ApiRequest::<CreateTaskReminderResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for CreateTaskReminderResponseV1 {
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
    fn test_create_task_reminder_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskReminderRequestV1::new(config.clone(), "task_123")
            .trigger_time(1704067200)
            .type_("absolute");

        assert_eq!(request.body.trigger_time, 1704067200);
        assert_eq!(request.body.type_, Some("absolute".to_string()));
    }

    #[test]
    fn test_task_reminder_create_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskReminderCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/reminders"
        );
    }
}
