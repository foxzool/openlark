//! 更新任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新任务请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateTaskBodyV1 {
    /// 任务标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,

    /// 任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,

    /// 任务优先级（1-5）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 更新任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateTaskResponseV1 {
    /// 任务 ID
    pub id: String,
    /// 任务标题
    pub summary: String,
    /// 任务描述
    #[serde(default)]
    pub description: Option<String>,
    /// 任务开始时间
    #[serde(default)]
    pub start: Option<String>,
    /// 任务截止时间
    #[serde(default)]
    pub due: Option<String>,
    /// 任务优先级
    #[serde(default)]
    pub priority: Option<i32>,
    /// 任务是否完成
    pub is_completed: bool,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 更新任务请求（v1）
#[derive(Debug, Clone)]
pub struct UpdateTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: UpdateTaskBodyV1,
}

impl UpdateTaskRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: UpdateTaskBodyV1::default(),
        }
    }

    /// 设置任务标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = Some(summary.into());
        self
    }

    /// 设置任务描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置任务开始时间
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.body.start = Some(start.into());
        self
    }

    /// 设置任务截止时间
    pub fn due(mut self, due: impl Into<String>) -> Self {
        self.body.due = Some(due.into());
        self
    }

    /// 设置任务优先级（1-5）
    pub fn priority(mut self, priority: i32) -> Self {
        self.body.priority = Some(priority);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTaskResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskUpdate(self.task_id);
        let mut request = ApiRequest::<UpdateTaskResponseV1>::patch(api_endpoint.to_url());

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

impl ApiResponseTrait for UpdateTaskResponseV1 {
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
    fn test_update_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateTaskRequestV1::new(config, "test_task_id")
            .summary("更新后的任务标题")
            .description("更新后的描述")
            .priority(4);

        assert_eq!(request.task_id, "test_task_id");
        assert_eq!(request.body.summary, Some("更新后的任务标题".to_string()));
        assert_eq!(request.body.description, Some("更新后的描述".to_string()));
        assert_eq!(request.body.priority, Some(4));
    }

    #[test]
    fn test_task_api_v1_update_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskUpdate("task_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks/task_123");
    }
}
