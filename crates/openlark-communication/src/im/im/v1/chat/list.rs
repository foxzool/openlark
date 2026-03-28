//! 获取用户或机器人所在的群列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/group/chat/list>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::{chat::models::ChatSortType, message::models::UserIdType},
};

/// 获取用户或机器人所在的群列表请求
///
/// 用于获取当前用户或机器人所在的所有群聊列表。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
/// - `sort_type`: 群组排序方式（可选，默认 ByCreateTimeAsc）
/// - `page_token`: 分页标记（可选）
/// - `page_size`: 分页大小（可选，默认 20，最大 100）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::chat::ListChatsRequest;
/// use openlark_communication::im::im::v1::message::models::UserIdType;
/// use openlark_communication::im::im::v1::chat::models::ChatSortType;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = ListChatsRequest::new(config)
///     .user_id_type(UserIdType::OpenId)
///     .sort_type(ChatSortType::ByCreateTimeAsc)
///     .page_size(50);
/// let response = request.execute().await?;
/// ```
pub struct ListChatsRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    sort_type: Option<ChatSortType>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListChatsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            sort_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 群组排序方式（查询参数，可选，默认 ByCreateTimeAsc）
    pub fn sort_type(mut self, sort_type: ChatSortType) -> Self {
        self.sort_type = Some(sort_type);
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
    /// docPath: <https://open.feishu.cn/document/server-docs/group/chat/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(IM_V1_CHATS);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(sort_type) = self.sort_type {
            req = req.query("sort_type", sort_type.as_str());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取用户或机器人所在的群列表")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_chats_request_builder() {
        let config = Config::default();
        let request = ListChatsRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.sort_type, None);
    }

    #[test]
    fn test_list_chats_request_with_user_id_type() {
        let config = Config::default();
        let request = ListChatsRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_list_chats_request_with_sort_type() {
        let config = Config::default();
        let request = ListChatsRequest::new(config).sort_type(ChatSortType::ByCreateTimeAsc);
        assert_eq!(request.sort_type, Some(ChatSortType::ByCreateTimeAsc));
    }

    #[test]
    fn test_list_chats_request_with_page_size() {
        let config = Config::default();
        let request = ListChatsRequest::new(config).page_size(50);
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_chats_request_with_page_token() {
        let config = Config::default();
        let request = ListChatsRequest::new(config).page_token("token123");
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_chats_request_with_all_options() {
        let config = Config::default();
        let request = ListChatsRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .sort_type(ChatSortType::ByActiveTimeDesc)
            .page_size(100)
            .page_token("token456");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(request.sort_type, Some(ChatSortType::ByActiveTimeDesc));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token456".to_string()));
    }
}
