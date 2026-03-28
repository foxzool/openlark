//! 合并转发消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/merge_forward>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::ReceiveIdType,
};

/// 合并转发消息请求体
///
/// 表示合并转发消息所需的请求参数。
///
/// # 字段说明
///
/// - `receive_id`: 消息接收者 ID，类型与 receive_id_type 一致
/// - `message_id_list`: 待转发的消息 ID 列表，至少包含 1 条消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeForwardMessageBody {
    /// 消息接收者 ID
    pub receive_id: String,
    /// 待转发的消息 ID 列表
    pub message_id_list: Vec<String>,
}

/// 合并转发消息请求
///
/// 用于将多条消息合并后转发给指定接收者。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `receive_id_type`: 消息接收者 ID 类型，必填
/// - `uuid`: 幂等 uuid，可选
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{MergeForwardMessageRequest, MergeForwardMessageBody};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = MergeForwardMessageBody {
///     receive_id: "ou_xxx".to_string(),
///     message_id_list: vec!["om_xxx1".to_string(), "om_xxx2".to_string()],
/// };
/// let request = MergeForwardMessageRequest::new(config)
///     .receive_id_type(ReceiveIdType::OpenId);
/// let response = request.execute(body).await?;
/// ```
pub struct MergeForwardMessageRequest {
    config: Config,
    receive_id_type: Option<ReceiveIdType>,
    uuid: Option<String>,
}

impl MergeForwardMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            receive_id_type: None,
            uuid: None,
        }
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
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/merge_forward>
    pub async fn execute(self, body: MergeForwardMessageBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: MergeForwardMessageBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(body.receive_id, "receive_id 不能为空");
        if body.message_id_list.is_empty() {
            return Err(error::validation_error(
                "message_id_list 不能为空".to_string(),
                "合并转发消息需要至少 1 条消息".to_string(),
            ));
        }
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "合并转发消息需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/messages/merge_forward
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/merge_forward", IM_V1_MESSAGES))
                .query("receive_id_type", receive_id_type.as_str())
                .body(serialize_params(&body, "合并转发消息")?);

        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "合并转发消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_forward_message_request_builder() {
        let config = Config::default();
        let request =
            MergeForwardMessageRequest::new(config).receive_id_type(ReceiveIdType::OpenId);
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
    }

    #[test]
    fn test_merge_forward_message_body() {
        let body = MergeForwardMessageBody {
            receive_id: "ou_xxx".to_string(),
            message_id_list: vec!["om_xxx1".to_string(), "om_xxx2".to_string()],
        };
        assert_eq!(body.receive_id, "ou_xxx");
        assert_eq!(body.message_id_list.len(), 2);
    }

    #[test]
    fn test_merge_forward_message_body_with_single_message() {
        let body = MergeForwardMessageBody {
            receive_id: "ou_xxx".to_string(),
            message_id_list: vec!["om_xxx1".to_string()],
        };
        assert_eq!(body.message_id_list.len(), 1);
    }

    #[test]
    fn test_merge_forward_message_request_with_uuid() {
        let config = Config::default();
        let request = MergeForwardMessageRequest::new(config)
            .receive_id_type(ReceiveIdType::OpenId)
            .uuid("uuid-123");
        assert_eq!(request.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_merge_forward_message_body_empty_list() {
        let body = MergeForwardMessageBody {
            receive_id: "ou_xxx".to_string(),
            message_id_list: vec![],
        };
        assert_eq!(body.message_id_list.len(), 0);
    }

    #[test]
    fn test_merge_forward_message_request_with_chat_id() {
        let config = Config::default();
        let request =
            MergeForwardMessageRequest::new(config).receive_id_type(ReceiveIdType::ChatId);
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::ChatId));
    }
}
