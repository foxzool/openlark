//! 搜索用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/search-users>

use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport};

use crate::{common::api_utils::extract_response_data, endpoints::SEARCH_V1_USER};

/// 搜索用户请求
pub struct SearchUserRequest {
    config: Config,
    query: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl SearchUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 搜索关键字（查询参数，可选）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/search-users>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(SEARCH_V1_USER);

        if let Some(query) = self.query {
            req = req.query("query", &query);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", &page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索用户")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_search_user_request_builder() {
        let config = Config::default();
        let request = SearchUserRequest::new(config)
            .query("zhangsan")
            .page_size(50)
            .page_token("token_123");

        assert_eq!(request.query, Some("zhangsan".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_search_user_request_default_values() {
        let config = Config::default();
        let request = SearchUserRequest::new(config);
        assert_eq!(request.query, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }
}
