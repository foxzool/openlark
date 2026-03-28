//! 创建任务关注者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskfollower/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务关注者请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateTaskFollowerBodyV1 {
    /// 关注者用户 ID
    pub follower_id: String,
}

/// 创建任务关注者响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateTaskFollowerResponseV1 {
    /// 关注者用户 ID
    pub follower_id: String,
}

/// 创建任务关注者请求（v1）
#[derive(Debug, Clone)]
pub struct CreateTaskFollowerRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: CreateTaskFollowerBodyV1,
}

impl CreateTaskFollowerRequestV1 {
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: CreateTaskFollowerBodyV1::default(),
        }
    }

    /// 设置关注者用户 ID
    pub fn follower_id(mut self, follower_id: impl Into<String>) -> Self {
        self.body.follower_id = follower_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskFollowerResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskFollowerResponseV1> {
        validate_required!(self.body.follower_id.trim(), "关注者用户 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerCreate(self.task_id.clone());
        let mut request = ApiRequest::<CreateTaskFollowerResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateTaskFollowerResponseV1 {
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
    fn test_create_task_follower_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            CreateTaskFollowerRequestV1::new(config.clone(), "task_123").follower_id("user_456");

        assert_eq!(request.body.follower_id, "user_456");
        assert_eq!(request.task_id, "task_123");
    }

    #[test]
    fn test_task_follower_create_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/followers"
        );
    }
}
