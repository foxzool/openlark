//! 批量删除任务关注者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskfollower/batch_delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除任务关注者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
/// 批量删除任务关注者请求体。
pub struct BatchDeleteTaskFollowerBodyV1 {
    /// 要删除的关注人 ID 列表
    pub id_list: Vec<String>,
}

/// 批量删除任务关注者响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 批量删除任务关注者响应。
pub struct BatchDeleteTaskFollowerResponseV1 {
    /// 实际删除的关注人用户 ID 列表
    #[serde(default)]
    pub followers: Vec<String>,
}

/// 批量删除任务关注者请求（v1）
#[derive(Debug, Clone)]
/// 批量删除任务关注者请求构建器。
pub struct BatchDeleteTaskFollowerRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: BatchDeleteTaskFollowerBodyV1,
}

impl BatchDeleteTaskFollowerRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: BatchDeleteTaskFollowerBodyV1::default(),
        }
    }

    /// 设置要删除的关注人 ID 列表
    pub fn id_list(mut self, id_list: Vec<impl Into<String>>) -> Self {
        self.body.id_list = id_list.into_iter().map(|v| v.into()).collect();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchDeleteTaskFollowerResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteTaskFollowerResponseV1> {
        validate_required!(self.body.id_list, "要删除的关注人 ID 列表不能为空");

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerBatchDelete(self.task_id.clone());
        let mut request =
            ApiRequest::<BatchDeleteTaskFollowerResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for BatchDeleteTaskFollowerResponseV1 {
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
    fn test_batch_delete_task_follower_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = BatchDeleteTaskFollowerRequestV1::new(config.clone(), "task_123")
            .id_list(vec!["user_1", "user_2"]);

        assert_eq!(request.body.id_list, vec!["user_1", "user_2"]);
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_batch_delete_task_follower_v1_body_serialization() {
        let body = BatchDeleteTaskFollowerBodyV1 {
            id_list: vec![
                "ou_13585843f02bc94923ed17a007cbc9b1".to_string(),
                "ou_f4506885e436763c36e03c05ef1bd6f8".to_string(),
            ],
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "id_list": [
                    "ou_13585843f02bc94923ed17a007cbc9b1",
                    "ou_f4506885e436763c36e03c05ef1bd6f8"
                ]
            })
        );
    }

    #[test]
    fn test_task_follower_batch_delete_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskFollowerBatchDelete(
            "task_123".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/batch_delete_follower"
        );
    }
}
