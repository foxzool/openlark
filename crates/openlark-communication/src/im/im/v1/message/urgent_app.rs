//! 发送应用内加急
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_app>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::UserIdType,
};

/// 发送应用内加急请求体
///
/// # 字段说明
///
/// - `user_id_list`: 用户 ID 列表，必填，至少包含 1 项
///
/// # 示例
///
/// ```rust,ignore
/// let body = UrgentAppBody::new()
///     .user_id_list(vec!["ou_xxx".to_string()]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentAppBody {
    user_id_list: Vec<String>,
}

impl Default for UrgentAppBody {
    fn default() -> Self {
        Self::new()
    }
}

impl UrgentAppBody {
    pub fn new() -> Self {
        Self {
            user_id_list: Vec::new(),
        }
    }

    pub fn user_id_list(mut self, user_id_list: Vec<String>) -> Self {
        self.user_id_list = user_id_list;
        self
    }
}

/// 发送应用内加急响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrgentAppResponse {
    #[serde(default)]
    pub invalid_user_id_list: Vec<String>,
}

impl ApiResponseTrait for UrgentAppResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 发送应用内加急请求
///
/// 用于对指定消息发送应用内加急通知。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待加急的消息 ID，必填
/// - `user_id_type`: 用户 ID 类型，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = UrgentAppBody::new()
///     .user_id_list(vec!["ou_xxx".to_string()]);
/// let request = UrgentAppRequest::new(config)
///     .message_id("msg_xxx")
///     .user_id_type(UserIdType::OpenId)
///     .execute(body).await?;
/// ```
pub struct UrgentAppRequest {
    config: Config,
    message_id: String,
    user_id_type: Option<UserIdType>,
}

impl UrgentAppRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            user_id_type: None,
        }
    }

    /// 待加急的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，必填）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/buzz-messages/urgent_app>
    pub async fn execute(self, body: UrgentAppBody) -> SDKResult<UrgentAppResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UrgentAppBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UrgentAppResponse> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        if body.user_id_list.is_empty() {
            return Err(error::validation_error(
                "user_id_list 不能为空".to_string(),
                "加急用户列表至少包含 1 项".to_string(),
            ));
        }
        let user_id_type = self.user_id_type.ok_or_else(|| {
            error::validation_error(
                "user_id_type 不能为空".to_string(),
                "发送应用内加急需要指定 user_id_type".to_string(),
            )
        })?;

        // url: PATCH:/open-apis/im/v1/messages/:message_id/urgent_app
        let req: ApiRequest<UrgentAppResponse> =
            ApiRequest::patch(format!("{}/{}/urgent_app", IM_V1_MESSAGES, self.message_id))
                .query("user_id_type", user_id_type.as_str())
                .body(serialize_params(&body, "发送应用内加急")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "发送应用内加急")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_urgent_app_body_builder() {
        let body =
            UrgentAppBody::new().user_id_list(vec!["ou_xxx".to_string(), "ou_yyy".to_string()]);
        assert_eq!(body.user_id_list.len(), 2);
    }

    #[test]
    fn test_urgent_app_body_default() {
        let body = UrgentAppBody::new();
        assert!(body.user_id_list.is_empty());
    }

    #[test]
    fn test_urgent_app_request_builder() {
        let config = Config::default();
        let request = UrgentAppRequest::new(config)
            .message_id("msg_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_urgent_app_request_default_values() {
        let config = Config::default();
        let request = UrgentAppRequest::new(config);
        assert_eq!(request.message_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_urgent_app_request_with_different_user_id_types() {
        let config = Config::default();
        let request1 = UrgentAppRequest::new(config.clone()).user_id_type(UserIdType::OpenId);
        let request2 = UrgentAppRequest::new(config).user_id_type(UserIdType::UserId);
        assert_eq!(request1.user_id_type, Some(UserIdType::OpenId));
        assert_eq!(request2.user_id_type, Some(UserIdType::UserId));
    }
}
