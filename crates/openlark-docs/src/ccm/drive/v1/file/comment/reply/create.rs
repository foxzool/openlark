//! 添加回复
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

use crate::common::api_endpoints::DriveApi;

/// 添加回复请求。
#[derive(Debug, Clone)]
pub struct FileCommentReplyCreateRequest {
    config: Arc<Config>,
    file_token: String,
    comment_id: String,
}

impl FileCommentReplyCreateRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            file_token: String::new(),
            comment_id: String::new(),
        }
    }

    /// 设置路径参数 `file_token`。
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = file_token.into();
        self
    }

    /// 设置路径参数 `comment_id`。
    pub fn comment_id(mut self, comment_id: impl Into<String>) -> Self {
        self.comment_id = comment_id.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.comment_id, "comment_id 不能为空");
        let req: ApiRequest<serde_json::Value> =
            DriveApi::CreateCommentReply(self.file_token, self.comment_id)
                .to_request()
                .body(body);
        Transport::request_typed(req, &self.config, Some(option), "添加回复").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = FileCommentReplyCreateRequest::new(config);
    }
}
