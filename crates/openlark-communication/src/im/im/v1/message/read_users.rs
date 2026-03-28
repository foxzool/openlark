//! 查询消息已读信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/read_users>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::UserIdType,
};

/// 查询消息已读信息请求
///
/// 用于查询指定消息的已读用户信息，支持分页查询。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
/// - `page_size`: 分页大小（可选，范围 1~100）
/// - `page_token`: 分页标记（可选）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::ReadMessageUsersRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = ReadMessageUsersRequest::new(config)
///     .message_id("om_xxx")
///     .user_id_type(UserIdType::OpenId)
///     .page_size(50);
/// let response = request.execute().await?;
/// ```
pub struct ReadMessageUsersRequest {
    config: Config,
    message_id: String,
    user_id_type: Option<UserIdType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ReadMessageUsersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 分页大小（查询参数，可选，范围 1~100）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/read_users>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");

        // url: GET:/open-apis/im/v1/messages/:message_id/read_users
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}/read_users", IM_V1_MESSAGES, self.message_id));

        // 文档虽标注必填，但同时给出默认值 open_id，这里采用默认值以降低使用成本。
        let user_id_type = self.user_id_type.unwrap_or(UserIdType::OpenId);
        req = req.query("user_id_type", user_id_type.as_str());

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询消息已读信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_read_message_users_request_builder() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::OpenId)
            .page_size(50);
        assert_eq!(request.message_id, "om_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_read_message_users_request_with_pagination() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config)
            .message_id("om_xxx")
            .page_size(20)
            .page_token("token_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_read_message_users_request_with_user_id() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_read_message_users_request_with_union_id() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_read_message_users_request_builder_chaining() {
        let config = Config::default();
        let request = ReadMessageUsersRequest::new(config)
            .message_id("om_test_123")
            .user_id_type(UserIdType::OpenId)
            .page_size(100)
            .page_token("page_token_456");
        assert_eq!(request.message_id, "om_test_123");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("page_token_456".to_string()));
    }
}
