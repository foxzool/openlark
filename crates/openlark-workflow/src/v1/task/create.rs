//! 创建任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskBodyV1 {
    /// 任务标题
    pub summary: String,

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

/// 创建任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskResponseV1 {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,
}

/// 创建任务请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskRequestV1 {
    config: Arc<Config>,
    body: CreateTaskBodyV1,
}

impl CreateTaskRequestV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTaskBodyV1::default(),
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

    /// 设置任务优先级（1-5）
    pub fn priority(mut self, priority: i32) -> Self {
        self.body.priority = Some(priority);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskResponseV1> {
        validate_required!(self.body.summary.trim(), "任务标题不能为空");

        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCreate;
        let mut request = ApiRequest::<CreateTaskResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateTaskResponseV1 {
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
    fn test_create_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskRequestV1::new(config)
            .summary("测试任务")
            .description("任务描述")
            .priority(3);

        assert_eq!(request.body.summary, "测试任务");
        assert_eq!(request.body.description, Some("任务描述".to_string()));
        assert_eq!(request.body.priority, Some(3));
    }

    #[test]
    fn test_task_api_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks");
    }
}
