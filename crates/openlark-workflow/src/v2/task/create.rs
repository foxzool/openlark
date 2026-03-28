//! 创建任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::{CreateTaskBody, CreateTaskResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建任务请求
#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateTaskBody,
}

impl CreateTaskRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTaskBody::default(),
        }
    }

    /// 设置任务标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
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

    /// 设置任务所属的任务清单 GUID
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.body.tasklist_guid = Some(tasklist_guid.into());
        self
    }

    /// 设置任务所属的分组 GUID
    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.body.section_guid = Some(section_guid.into());
        self
    }

    /// 设置任务优先级（1-5）
    pub fn priority(mut self, priority: i32) -> Self {
        self.body.priority = Some(priority);
        self
    }

    /// 设置自定义字段
    pub fn custom_fields(mut self, custom_fields: serde_json::Value) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置任务关注者
    pub fn followers(mut self, followers: Vec<String>) -> Self {
        self.body.followers = Some(followers);
        self
    }

    /// 设置子任务
    pub fn subtasks(mut self, subtasks: Vec<serde_json::Value>) -> Self {
        self.body.subtasks = Some(subtasks);
        self
    }

    /// 设置任务执行者
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.body.assignee = Some(assignee.into());
        self
    }

    /// 设置任务提醒时间
    pub fn remind_time(mut self, remind_time: impl Into<String>) -> Self {
        self.body.remind_time = Some(remind_time.into());
        self
    }

    /// 设置重复规则
    pub fn repeat_rule(mut self, repeat_rule: serde_json::Value) -> Self {
        self.body.repeat_rule = Some(repeat_rule);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskResponse> {
        // 验证必填字段
        validate_required!(self.body.summary.trim(), "任务标题不能为空");

        let api_endpoint = TaskApiV2::TaskCreate;
        let mut request = ApiRequest::<CreateTaskResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建任务")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建任务")
    }
}

impl ApiResponseTrait for CreateTaskResponse {
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
    fn test_create_task_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskRequest::new(config)
            .summary("测试任务")
            .description("任务描述")
            .priority(3);

        assert_eq!(request.body.summary, "测试任务");
        assert_eq!(request.body.description, Some("任务描述".to_string()));
        assert_eq!(request.body.priority, Some(3));
    }

    #[test]
    fn test_task_api_v2_url() {
        let endpoint = TaskApiV2::TaskCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v2/tasks");

        let endpoint = TaskApiV2::TaskGet("task_123".to_string());
        assert!(endpoint.to_url().contains("task_123"));
    }
}
