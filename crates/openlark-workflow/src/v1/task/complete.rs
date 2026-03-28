//! 完成任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/complete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 完成任务请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CompleteTaskBodyV1 {
    /// 完成时间（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// 完成任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CompleteTaskResponseV1 {
    /// 任务 ID
    pub id: String,
    /// 任务标题
    pub summary: String,
    /// 任务是否完成
    pub is_completed: bool,
    /// 完成时间
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// 完成任务请求（v1）
#[derive(Debug, Clone)]
pub struct CompleteTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CompleteTaskBodyV1,
}

impl CompleteTaskRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CompleteTaskBodyV1::default(),
        }
    }

    /// 设置完成时间
    pub fn completed_at(mut self, completed_at: impl Into<String>) -> Self {
        self.body.completed_at = Some(completed_at.into());
        self
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
        let mut request = ApiRequest::<CompleteTaskResponseV1>::post(api_endpoint.to_url());

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

        let request =
            CompleteTaskRequestV1::new(config, "test_task_id").completed_at("2024-01-15T10:00:00Z");

        assert_eq!(request.task_id, "test_task_id");
        assert_eq!(
            request.body.completed_at,
            Some("2024-01-15T10:00:00Z".to_string())
        );
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
