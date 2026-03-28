//! 删除评论
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/comment/delete>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::DeleteCommentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 删除评论请求
#[derive(Debug, Clone)]
pub struct DeleteCommentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 评论 GUID
    comment_guid: String,
}

impl DeleteCommentRequest {
    pub fn new(config: Arc<Config>, task_guid: String, comment_guid: String) -> Self {
        Self {
            config,
            task_guid,
            comment_guid,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteCommentResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.comment_guid.trim(), "评论GUID不能为空");

        let api_endpoint =
            TaskApiV2::CommentDelete(self.task_guid.clone(), self.comment_guid.clone());
        let request = ApiRequest::<DeleteCommentResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除评论")
    }
}

impl ApiResponseTrait for DeleteCommentResponse {
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
    fn test_delete_comment_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = DeleteCommentRequest::new(
            Arc::new(config),
            "task_123".to_string(),
            "comment_456".to_string(),
        );

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.comment_guid, "comment_456");
    }
}
