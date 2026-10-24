//! 创建任务协作者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskcollaborator/create>

use crate::v1::task::models::TaskCollaboratorV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务协作者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
/// 创建任务协作者请求体。
pub struct CreateTaskCollaboratorBodyV1 {
    /// 任务执行者 ID（官方已废弃，为向前兼容保留，推荐使用 id_list）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 执行者用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}

/// 创建任务协作者响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 创建任务协作者响应。
pub struct CreateTaskCollaboratorResponseV1 {
    /// 创建成功后的任务执行者
    pub collaborator: TaskCollaboratorV1,
}

/// 创建任务协作者请求（v1）
#[derive(Debug, Clone)]
/// 创建任务协作者请求构建器。
pub struct CreateTaskCollaboratorRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskCollaboratorBodyV1,
}

impl CreateTaskCollaboratorRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskCollaboratorBodyV1::default(),
        }
    }

    /// 设置任务执行者 ID（官方已废弃，推荐使用 id_list）
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.body.id = Some(id.into());
        self
    }

    /// 设置执行者用户 ID 列表
    pub fn id_list(mut self, id_list: Vec<impl Into<String>>) -> Self {
        self.body.id_list = Some(id_list.into_iter().map(|v| v.into()).collect());
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
        let id_blank = self.body.id.as_deref().is_none_or(|v| v.trim().is_empty());
        let id_list_empty = self.body.id_list.as_ref().is_none_or(|l| l.is_empty());
        if id_blank && id_list_empty {
            return Err(openlark_core::error::CoreError::validation_msg(
                "id 与 id_list 至少传一个",
            ));
        }

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskCollaboratorCreate(self.task_id.clone());
        let mut request =
            ApiRequest::<CreateTaskCollaboratorResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
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
            .id("user_456")
            .id_list(vec!["user_789"]);

        assert_eq!(request.body.id, Some("user_456".to_string()));
        assert_eq!(request.body.id_list, Some(vec!["user_789".to_string()]));
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_create_task_collaborator_v1_body_serialization() {
        let body = CreateTaskCollaboratorBodyV1 {
            id: Some("ou_99e1a581b36ecc4862cbfbce473f1234".to_string()),
            id_list: Some(vec!["ou_99e1a581b36ecc4862cbfbce473f3123".to_string()]),
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "id": "ou_99e1a581b36ecc4862cbfbce473f1234",
                "id_list": ["ou_99e1a581b36ecc4862cbfbce473f3123"]
            })
        );
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
