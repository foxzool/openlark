//! 创建任务协作者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcollaborator/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务协作者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskCollaboratorBodyV1 {
    /// 协作者用户 ID
    pub collaborator_id: String,
}

/// 创建任务协作者响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskCollaboratorResponseV1 {
    /// 协作者用户 ID
    pub collaborator_id: String,
}

/// 创建任务协作者请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskCollaboratorRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskCollaboratorBodyV1,
}

impl CreateTaskCollaboratorRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskCollaboratorBodyV1::default(),
        }
    }

    /// 设置协作者用户 ID
    pub fn collaborator_id(mut self, collaborator_id: impl Into<String>) -> Self {
        self.body.collaborator_id = collaborator_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskCollaboratorResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskCollaboratorResponseV1> {
        validate_required!(self.body.collaborator_id.trim(), "协作者用户 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCollaboratorCreate(self.task_id.clone());
        let mut request =
            ApiRequest::<CreateTaskCollaboratorResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateTaskCollaboratorResponseV1 {
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
    fn test_create_task_collaborator_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskCollaboratorRequestV1::new(config.clone(), "task_123")
            .collaborator_id("user_456");

        assert_eq!(request.body.collaborator_id, "user_456");
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_task_collaborator_create_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCollaboratorCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/collaborators"
        );
    }
}
