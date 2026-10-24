//! 创建任务提醒（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskreminder/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务提醒请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
/// 创建任务提醒请求体。
pub struct CreateTaskReminderBodyV1 {
    /// 相对截止时间的提醒时间（分钟数；提前 30 分钟为 30，截止时间后 30 分钟为 -30）
    pub relative_fire_minute: i64,
}

/// 任务提醒对象（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskReminderItemV1 {
    /// 提醒时间设置的 ID（删除时需使用）
    #[serde(default)]
    pub id: Option<String>,

    /// 相对截止时间的提醒时间（分钟数）
    #[serde(default)]
    pub relative_fire_minute: Option<i64>,
}

/// 创建任务提醒响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 创建任务提醒响应。
pub struct CreateTaskReminderResponseV1 {
    /// 创建成功的提醒时间
    pub reminder: TaskReminderItemV1,
}

/// 创建任务提醒请求（v1）
#[derive(Debug, Clone)]
/// 创建任务提醒请求构建器。
pub struct CreateTaskReminderRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskReminderBodyV1,
}

impl CreateTaskReminderRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskReminderBodyV1::default(),
        }
    }

    /// 设置相对截止时间的提醒时间（分钟数；提前 30 分钟为 30，截止时间后 30 分钟为 -30）
    pub fn relative_fire_minute(mut self, relative_fire_minute: i64) -> Self {
        self.body.relative_fire_minute = relative_fire_minute;
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

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
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

        let request =
            CreateTaskReminderRequestV1::new(config.clone(), "task_123").relative_fire_minute(30);

        assert_eq!(request.body.relative_fire_minute, 30);
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_create_task_reminder_v1_body_serialization() {
        let body = CreateTaskReminderBodyV1 {
            relative_fire_minute: 30,
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json, serde_json::json!({"relative_fire_minute": 30}));
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
