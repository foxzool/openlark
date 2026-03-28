//! 获取指定消息的内容
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::UserIdType,
};

/// 获取指定消息的内容请求
///
/// 用于获取指定消息的详细内容。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选，默认 open_id）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::GetMessageRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = GetMessageRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute().await?;
/// ```
pub struct GetMessageRequest {
    config: Config,
    message_id: String,
    user_id_type: Option<UserIdType>,
}

impl GetMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            user_id_type: None,
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

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/get>
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

        // url: GET:/open-apis/im/v1/messages/:message_id
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}", IM_V1_MESSAGES, self.message_id));
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取指定消息的内容")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_request_builder() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.message_id, "om_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_get_message_request_without_user_id_type() {
        let config = Config::default();
        let request = GetMessageRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = GetMessageRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_get_message_request_with_user_id() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::UserId);
        assert_eq!(request.user_id_type, Some(UserIdType::UserId));
    }

    #[test]
    fn test_get_message_request_with_union_id() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .message_id("om_xxx")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }

    #[test]
    fn test_get_message_request_builder_chaining() {
        let config = Config::default();
        let request = GetMessageRequest::new(config)
            .message_id("om_test_123")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.message_id, "om_test_123");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }
}
