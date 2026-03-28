//! 转发消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/forward>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::ReceiveIdType,
};

/// 转发消息请求体
///
/// 表示转发消息所需的请求参数。
///
/// # 字段说明
///
/// - `receive_id`: 消息接收者 ID，类型与 receive_id_type 一致
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardMessageBody {
    /// 消息接收者 ID
    pub receive_id: String,
}

/// 转发消息请求
///
/// 用于将指定消息转发给其他接收者。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待转发的消息 ID，必填
/// - `receive_id_type`: 消息接收者 ID 类型，必填
/// - `uuid`: 幂等 uuid，可选
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{ForwardMessageRequest, ForwardMessageBody};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = ForwardMessageBody {
///     receive_id: "ou_xxx".to_string(),
/// };
/// let request = ForwardMessageRequest::new(config)
///     .message_id("om_xxx")
///     .receive_id_type(ReceiveIdType::OpenId);
/// let response = request.execute(body).await?;
/// ```
pub struct ForwardMessageRequest {
    config: Config,
    message_id: String,
    receive_id_type: Option<ReceiveIdType>,
    uuid: Option<String>,
}

impl ForwardMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            receive_id_type: None,
            uuid: None,
        }
    }

    /// 待转发的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 消息接收者 ID 类型（查询参数，必填）
    pub fn receive_id_type(mut self, receive_id_type: ReceiveIdType) -> Self {
        self.receive_id_type = Some(receive_id_type);
        self
    }

    /// 幂等 uuid（查询参数，可选）
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/forward>
    pub async fn execute(self, body: ForwardMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ForwardMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(body.receive_id, "receive_id 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "转发消息需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/messages/:message_id/forward
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/forward", IM_V1_MESSAGES, self.message_id))
                .query("receive_id_type", receive_id_type.as_str())
                .body(serialize_params(&body, "转发消息")?);

        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "转发消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_message_request_builder() {
        let config = Config::default();
        let request = ForwardMessageRequest::new(config)
            .message_id("om_xxx")
            .receive_id_type(ReceiveIdType::OpenId);
        assert_eq!(request.message_id, "om_xxx");
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
    }

    #[test]
    fn test_forward_message_body() {
        let body = ForwardMessageBody {
            receive_id: "ou_xxx".to_string(),
        };
        assert_eq!(body.receive_id, "ou_xxx");
    }

    #[test]
    fn test_forward_message_request_with_uuid() {
        let config = Config::default();
        let request = ForwardMessageRequest::new(config)
            .message_id("om_xxx")
            .receive_id_type(ReceiveIdType::OpenId)
            .uuid("uuid-123");
        assert_eq!(request.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_forward_message_request_with_chat_id() {
        let config = Config::default();
        let request = ForwardMessageRequest::new(config)
            .message_id("om_xxx")
            .receive_id_type(ReceiveIdType::ChatId);
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::ChatId));
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = ForwardMessageRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_forward_message_request_builder_chaining() {
        let config = Config::default();
        let request = ForwardMessageRequest::new(config)
            .message_id("om_test_123")
            .receive_id_type(ReceiveIdType::OpenId)
            .uuid("test-uuid-456");
        assert_eq!(request.message_id, "om_test_123");
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
        assert_eq!(request.uuid, Some("test-uuid-456".to_string()));
    }
}
