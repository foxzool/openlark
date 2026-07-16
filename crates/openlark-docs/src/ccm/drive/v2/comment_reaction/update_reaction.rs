//! 添加/取消表情回应
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};
use std::sync::Arc;

use crate::common::api_endpoints::DriveApi;

/// 添加/取消表情回应请求。
#[derive(Debug, Clone)]
pub struct CommentReactionUpdateReactionRequest {
    config: Arc<Config>,
    file_token: String,
}

impl CommentReactionUpdateReactionRequest {
    /// 创建请求。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            file_token: String::new(),
        }
    }

    /// 设置路径参数 `file_token`。
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = file_token.into();
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
        let req: ApiRequest<serde_json::Value> = DriveApi::UpdateCommentReaction(self.file_token)
            .to_request()
            .body(body);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("添加/取消表情回应", "响应数据为空")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _request = CommentReactionUpdateReactionRequest::new(config);
    }
}
