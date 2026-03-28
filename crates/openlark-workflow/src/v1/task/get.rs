//! 获取指定任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务信息（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskV1 {
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
    /// 任务优先级（1-5）
    #[serde(default)]
    pub priority: Option<i32>,
    /// 任务是否完成
    pub is_completed: bool,
    /// 创建时间
    pub created_at: String,
    /// 更新时间
    pub updated_at: String,
}

/// 获取任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct GetTaskResponseV1 {
    /// 任务信息
    pub task: TaskV1,
}

/// 获取任务请求（v1）
#[derive(Debug, Clone)]
pub struct GetTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
}

impl GetTaskRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTaskResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskGet(self.task_id);
        let request = ApiRequest::<GetTaskResponseV1>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for GetTaskResponseV1 {
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
    fn test_get_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = GetTaskRequestV1::new(config, "test_task_id");
        assert_eq!(request.task_id, "test_task_id");
    }

    #[test]
    fn test_task_api_v1_get_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskGet("task_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks/task_123");
    }
}
