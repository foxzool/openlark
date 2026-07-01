//! 搜索任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task/search>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::sync::Arc;

/// 搜索任务请求
pub struct SearchTaskRequest {
    config: Arc<Config>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl SearchTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小（可选）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（可选）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
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
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(TaskApiV2::TaskSearch.to_url())
                .body(serialize_params(&body, "搜索任务")?);

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索任务")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let request = SearchTaskRequest::new(config);
        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn builder_sets_params() {
        let config = Arc::new(Config::default());
        let request = SearchTaskRequest::new(config)
            .page_size(50)
            .page_token("token_abc")
            .user_id_type("open_id");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_abc".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_url_construction() {
        use crate::common::api_endpoints::TaskApiV2;
        let url = TaskApiV2::TaskSearch.to_url();
        assert_eq!(url, "/open-apis/task/v2/tasks/search");
    }
}
