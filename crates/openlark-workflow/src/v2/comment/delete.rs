//! 删除评论
//!
//! docPath: <https://open.feishu.cn/document/task-v2/comment/delete>

use crate::common::api_endpoints::TaskApiV2;
use crate::v2::comment::models::DeleteCommentResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除评论请求
#[derive(Debug, Clone)]
pub struct DeleteCommentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 评论 GUID
    comment_guid: String,
}

impl DeleteCommentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, comment_guid: String) -> Self {
        Self {
            config,
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
        validate_required!(self.comment_guid.trim(), "评论GUID不能为空");

        let api_endpoint = TaskApiV2::CommentDelete(self.comment_guid.clone());
        let request = ApiRequest::<DeleteCommentResponse>::delete(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除评论",
        )
        .await
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

        let request = DeleteCommentRequest::new(Arc::new(config), "comment_456".to_string());

        assert_eq!(request.comment_guid, "comment_456");
    }
}
