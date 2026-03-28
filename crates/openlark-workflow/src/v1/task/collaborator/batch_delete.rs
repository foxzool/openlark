//! 批量删除任务协作者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcollaborator/batch_delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除任务协作者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeleteTaskCollaboratorBodyV1 {
    /// 需要删除的协作者用户 ID 列表
    pub collaborator_ids: Vec<String>,
}

/// 批量删除任务协作者响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteTaskCollaboratorResponseV1 {
    /// 删除失败的用户 ID 列表
    pub failed_collaborator_ids: Option<Vec<String>>,
}

/// 批量删除任务协作者请求（v1）
#[derive(Debug, Clone)]
pub struct BatchDeleteTaskCollaboratorRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: BatchDeleteTaskCollaboratorBodyV1,
}

impl BatchDeleteTaskCollaboratorRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: BatchDeleteTaskCollaboratorBodyV1::default(),
        }
    }

    /// 设置需要删除的协作者用户 ID 列表
    pub fn collaborator_ids(mut self, collaborator_ids: Vec<impl Into<String>>) -> Self {
        self.body.collaborator_ids = collaborator_ids.into_iter().map(|v| v.into()).collect();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteTaskCollaboratorResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteTaskCollaboratorResponseV1> {
        if self.body.collaborator_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "协作者用户 ID 列表不能为空",
                "",
            ));
        }

        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCollaboratorBatchDelete(
            self.task_id.clone(),
        );
        let mut request =
            ApiRequest::<BatchDeleteTaskCollaboratorResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for BatchDeleteTaskCollaboratorResponseV1 {
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
    fn test_batch_delete_task_collaborator_v1_builder() {
        let config = Arc::new(Config::builder().app_id("test").app_secret("test").build());

        let request = BatchDeleteTaskCollaboratorRequestV1::new(config.clone(), "task_123")
            .collaborator_ids(vec!["user_1", "user_2"]);

        assert_eq!(request.body.collaborator_ids.len(), 2);
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_task_collaborator_batch_delete_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCollaboratorBatchDelete(
            "task_123".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/batch_delete_collaborator"
        );
    }
}
