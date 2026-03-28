//! 创建子任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-subtask/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建子任务请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateSubtaskBody {
    /// 子任务标题
    pub summary: String,

    /// 子任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 子任务执行者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,

    /// 子任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<String>,

    /// 子任务完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// 创建子任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSubtaskResponse {
    /// 子任务 GUID
    pub task_guid: String,

    /// 子任务标题
    pub summary: String,

    /// 子任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 子任务状态
    pub status: String,

    /// 父任务 GUID
    pub parent_task_guid: String,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 创建子任务请求
#[derive(Debug, Clone)]
pub struct CreateSubtaskRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 父任务 GUID
    task_guid: String,
    /// 请求体
    body: CreateSubtaskBody,
}

impl CreateSubtaskRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: CreateSubtaskBody::default(),
        }
    }

    /// 设置子任务标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
        self
    }

    /// 设置子任务描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置子任务执行者
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.body.assignee = Some(assignee.into());
        self
    }

    /// 设置子任务截止时间
    pub fn due(mut self, due: impl Into<String>) -> Self {
        self.body.due = Some(due.into());
        self
    }

    /// 设置子任务完成时间
    pub fn completed_at(mut self, completed_at: impl Into<String>) -> Self {
        self.body.completed_at = Some(completed_at.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateSubtaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateSubtaskResponse> {
        // 验证必填字段
        validate_required!(self.body.summary.trim(), "子任务标题不能为空");
        validate_required!(self.task_guid.trim(), "父任务 GUID 不能为空");

        let api_endpoint = TaskApiV2::SubtaskCreate(self.task_guid.clone());
        let mut request = ApiRequest::<CreateSubtaskResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建子任务")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建子任务")
    }
}

impl ApiResponseTrait for CreateSubtaskResponse {
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
    fn test_create_subtask_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateSubtaskRequest::new(config, "parent_task_123")
            .summary("测试子任务")
            .description("子任务描述")
            .assignee("user_123");

        assert_eq!(request.task_guid, "parent_task_123");
        assert_eq!(request.body.summary, "测试子任务");
        assert_eq!(request.body.description, Some("子任务描述".to_string()));
        assert_eq!(request.body.assignee, Some("user_123".to_string()));
    }

    #[test]
    fn test_subtask_api_v2_url() {
        let endpoint = TaskApiV2::SubtaskCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/subtasks"
        );
    }
}
