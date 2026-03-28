//! 创建评论
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/comment/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::comment::models::{CreateCommentBody, CreateCommentResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建评论请求
#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: CreateCommentBody,
}

impl CreateCommentRequest {
    pub fn new(config: Arc<Config>, task_guid: String) -> Self {
        Self {
            config,
            task_guid,
            body: CreateCommentBody::default(),
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCommentResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.content.trim(), "评论内容不能为空");

        let api_endpoint = TaskApiV2::CommentCreate(self.task_guid.clone());
        let mut request = ApiRequest::<CreateCommentResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建评论")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建评论")
    }
}

impl ApiResponseTrait for CreateCommentResponse {
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
    fn test_create_comment_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            CreateCommentRequest::new(config, "task_123".to_string()).content("这是一条评论");

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.content, "这是一条评论");
    }
}
