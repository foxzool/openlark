//! 批量删除任务关注者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskfollower/batch_delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除任务关注者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeleteTaskFollowerBodyV1 {
    /// 需要删除的关注者用户 ID 列表
    pub follower_ids: Vec<String>,
}

/// 批量删除任务关注者响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteTaskFollowerResponseV1 {
    /// 删除失败的用户 ID 列表
    pub failed_follower_ids: Option<Vec<String>>,
}

/// 批量删除任务关注者请求（v1）
#[derive(Debug, Clone)]
pub struct BatchDeleteTaskFollowerRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: BatchDeleteTaskFollowerBodyV1,
}

impl BatchDeleteTaskFollowerRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: BatchDeleteTaskFollowerBodyV1::default(),
        }
    }

    /// 设置需要删除的关注者用户 ID 列表
    pub fn follower_ids(mut self, follower_ids: Vec<impl Into<String>>) -> Self {
        self.body.follower_ids = follower_ids.into_iter().map(|v| v.into()).collect();
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
        if self.body.follower_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "关注者用户 ID 列表不能为空",
                "",
            ));
        }

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerBatchDelete(self.task_id.clone());
        let mut request =
            ApiRequest::<BatchDeleteTaskFollowerResponseV1>::post(api_endpoint.to_url());

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
            .follower_ids(vec!["user_1", "user_2"]);

        assert_eq!(request.body.follower_ids.len(), 2);
        assert_eq!(request.task_id, "task_123");
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
