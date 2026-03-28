//! 删除任务关注者（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskfollower/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 删除任务关注者响应（v1）
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteTaskFollowerResponseV1 {
    /// 是否成功删除
    pub success: bool,
}

/// 删除任务关注者请求（v1）
#[derive(Debug, Clone)]
pub struct DeleteTaskFollowerRequestV1 {
    config: Arc<Config>,
    task_id: String,
    follower_id: String,
}

impl DeleteTaskFollowerRequestV1 {
    pub fn new(
        config: Arc<Config>,
        task_id: impl Into<String>,
        follower_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            follower_id: follower_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteTaskFollowerResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteTaskFollowerResponseV1> {
        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskFollowerDelete(
            self.task_id.clone(),
            self.follower_id.clone(),
        );
        let request = ApiRequest::<DeleteTaskFollowerResponseV1>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for DeleteTaskFollowerResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_delete_task_follower_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskFollowerDelete(
            "task_123".to_string(),
            "user_456".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/followers/user_456"
        );
    }
}
