//! 搜索消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/search>

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::IM_V1_MESSAGES_SEARCH;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 搜索消息请求。
#[derive(Debug, Clone)]
pub struct SearchMessagesRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl SearchMessagesRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页大小（查询参数）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（查询参数）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（查询参数）。
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
            ApiRequest::post(IM_V1_MESSAGES_SEARCH).body(serialize_params(&body, "搜索消息")?);

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
        extract_response_data(resp, "搜索消息")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_query_params() {
        let config = Config::default();
        let request = SearchMessagesRequest::new(config)
            .page_size(20)
            .page_token("token_1")
            .user_id_type("open_id");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token_1".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_search_messages_request_url() {
        use crate::endpoints::im::IM_V1_MESSAGES_SEARCH;
        assert_eq!(IM_V1_MESSAGES_SEARCH, "/open-apis/im/v1/messages/search");
    }
}
