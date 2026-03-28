//! 删除任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 删除任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTaskResponseV1 {
    /// 删除结果
    pub success: bool,
}

/// 删除任务请求（v1）
#[derive(Debug, Clone)]
pub struct DeleteTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
}

impl DeleteTaskRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteTaskResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskDelete(self.task_id);
        let request = ApiRequest::<DeleteTaskResponseV1>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for DeleteTaskResponseV1 {
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
    fn test_delete_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = DeleteTaskRequestV1::new(config, "test_task_id");
        assert_eq!(request.task_id, "test_task_id");
    }

    #[test]
    fn test_task_api_v1_delete_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskDelete("task_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks/task_123");
    }
}
