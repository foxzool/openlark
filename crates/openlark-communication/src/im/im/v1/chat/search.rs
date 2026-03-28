//! 搜索对用户或机器人可见的群列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/search>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 搜索对用户或机器人可见的群列表请求
///
/// 用于根据关键词搜索用户或机器人可见的群聊列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
/// - `query`: 搜索关键词（可选）
/// - `page_token`: 分页标记（可选）
/// - `page_size`: 分页大小（可选，默认 20，最大 100）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::chat::SearchChatsRequest;
/// use openlark_communication::im::im::v1::message::models::UserIdType;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = SearchChatsRequest::new(config)
///     .query("项目群")
///     .user_id_type(UserIdType::OpenId)
///     .page_size(20);
/// let response = request.execute().await?;
/// ```
pub struct SearchChatsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    query: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl SearchChatsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            query: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 关键词（查询参数，可选）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/search>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/search", IM_V1_CHATS));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(query) = self.query {
            req = req.query("query", query);
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "搜索对用户或机器人可见的群列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_search_chats_request_builder() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config);
        assert_eq!(request.query, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_search_chats_request_with_query() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config).query("测试群");
        assert_eq!(request.query, Some("测试群".to_string()));
    }

    #[test]
    fn test_search_chats_request_with_user_id_type() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config)
            .query("项目群")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_search_chats_request_with_page_size() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config).page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_search_chats_request_with_page_token() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config).page_token("token789");
        assert_eq!(request.page_token, Some("token789".to_string()));
    }

    #[test]
    fn test_search_chats_request_with_all_options() {
        let config = Config::default();
        let request = SearchChatsRequest::new(config)
            .query("开发群")
            .user_id_type(UserIdType::UnionId)
            .page_size(100)
            .page_token("token999");
        assert_eq!(request.query, Some("开发群".to_string()));
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token999".to_string()));
    }
}
