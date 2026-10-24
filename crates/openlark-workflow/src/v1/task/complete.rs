//! 完成任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/complete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::Deserialize;
use std::sync::Arc;

/// 完成任务响应（v1）
///
/// 官方文档响应示例中 `data` 为空对象。
#[derive(Debug, Clone, Deserialize)]
/// 完成任务响应。
pub struct CompleteTaskResponseV1 {}

/// 完成任务请求（v1）
///
/// 官方接口不接受请求体。
#[derive(Debug, Clone)]
/// 完成任务请求构建器。
pub struct CompleteTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
}

impl CompleteTaskRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CompleteTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CompleteTaskResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskComplete(self.task_id);
        let request = ApiRequest::<CompleteTaskResponseV1>::post(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for CompleteTaskResponseV1 {
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
    fn test_complete_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CompleteTaskRequestV1::new(config, "test_task_id");

        assert_eq!(request.task_id, "test_task_id");
    }

    #[test]
    fn test_task_api_v1_complete_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskComplete("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/complete"
        );
    }
}
