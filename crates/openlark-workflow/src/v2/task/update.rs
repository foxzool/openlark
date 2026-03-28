//! 更新任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task/update>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::{UpdateTaskBody, UpdateTaskResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 更新任务请求
#[derive(Debug, Clone)]
pub struct UpdateTaskRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: UpdateTaskBody,
}

impl UpdateTaskRequest {
    pub fn new(config: Arc<Config>, task_guid: String) -> Self {
        Self {
            config,
            task_guid,
            body: UpdateTaskBody::default(),
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

    /// 设置任务执行者
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.body.assignee = Some(assignee.into());
        self
    }

    /// 设置任务状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.body.status = Some(status.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTaskResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");

        let api_endpoint = TaskApiV2::TaskUpdate(self.task_guid.clone());
        let mut request = ApiRequest::<UpdateTaskResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新任务")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新任务")
    }
}

impl ApiResponseTrait for UpdateTaskResponse {
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
    fn test_update_task_builder() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = UpdateTaskRequest::new(Arc::new(config), "task_123".to_string())
            .summary("更新后的标题")
            .priority(4);

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.summary, Some("更新后的标题".to_string()));
    }
}
