//! 发送消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::ReceiveIdType,
};

/// 发送消息请求体
///
/// 表示发送消息所需的请求参数。
///
/// # 字段说明
///
/// - `receive_id`: 消息接收者 ID，类型与 receive_id_type 一致
/// - `msg_type`: 消息类型（如 text、post、card 等）
/// - `content`: 消息内容，JSON 字符串格式
/// - `uuid`: 幂等 uuid，用于防止重复发送（可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageBody {
    /// 消息接收者 ID，类型与 receive_id_type 一致
    pub receive_id: String,
    /// 消息类型
    pub msg_type: String,
    /// 消息内容（JSON 字符串）
    pub content: String,
    /// 幂等 uuid（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// 发送消息请求
///
/// 用于向指定接收者发送消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `receive_id_type`: 接收者 ID 类型，必填
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{CreateMessageRequest, CreateMessageBody};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = CreateMessageBody {
///     receive_id: "ou_xxx".to_string(),
///     msg_type: "text".to_string(),
///     content: r#"{"text":"Hello"}"#.to_string(),
///     uuid: None,
/// };
/// let request = CreateMessageRequest::new(config)
///     .receive_id_type(ReceiveIdType::OpenId);
/// let response = request.execute(body).await?;
/// ```
pub struct CreateMessageRequest {
    config: Config,
    receive_id_type: Option<ReceiveIdType>,
}

impl CreateMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            receive_id_type: None,
        }
    }

    /// 接收者 ID 类型（查询参数，必填）
    pub fn receive_id_type(mut self, receive_id_type: ReceiveIdType) -> Self {
        self.receive_id_type = Some(receive_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/create>
    pub async fn execute(self, body: CreateMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(body.receive_id, "receive_id 不能为空");
        validate_required!(body.msg_type, "msg_type 不能为空");
        validate_required!(body.content, "content 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            openlark_core::error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "发送消息需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/messages
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_V1_MESSAGES)
            .query("receive_id_type", receive_id_type.as_str())
            .body(serialize_params(&body, "发送消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "发送消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message_request_builder() {
        let config = Config::default();
        let request = CreateMessageRequest::new(config).receive_id_type(ReceiveIdType::OpenId);
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
    }

    #[test]
    fn test_create_message_body() {
        let body = CreateMessageBody {
            receive_id: "ou_xxx".to_string(),
            msg_type: "text".to_string(),
            content: r#"{"text":"Hello"}"#.to_string(),
            uuid: Some("uuid-123".to_string()),
        };
        assert_eq!(body.receive_id, "ou_xxx");
        assert_eq!(body.msg_type, "text");
        assert_eq!(body.content, r#"{"text":"Hello"}"#);
        assert_eq!(body.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_create_message_body_without_uuid() {
        let body = CreateMessageBody {
            receive_id: "ou_xxx".to_string(),
            msg_type: "text".to_string(),
            content: r#"{"text":"Hello"}"#.to_string(),
            uuid: None,
        };
        assert_eq!(body.uuid, None);
    }

    #[test]
    fn test_empty_receive_id() {
        let body = CreateMessageBody {
            receive_id: "".to_string(),
            msg_type: "text".to_string(),
            content: r#"{"text":"Hello"}"#.to_string(),
            uuid: None,
        };
        assert_eq!(body.receive_id, "");
    }

    #[test]
    fn test_empty_msg_type() {
        let body = CreateMessageBody {
            receive_id: "ou_xxx".to_string(),
            msg_type: "".to_string(),
            content: r#"{"text":"Hello"}"#.to_string(),
            uuid: None,
        };
        assert_eq!(body.msg_type, "");
    }

    #[test]
    fn test_empty_content() {
        let body = CreateMessageBody {
            receive_id: "ou_xxx".to_string(),
            msg_type: "text".to_string(),
            content: "".to_string(),
            uuid: None,
        };
        assert_eq!(body.content, "");
    }
}
