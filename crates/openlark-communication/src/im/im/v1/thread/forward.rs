//! 转发话题
//!
//! docPath: <https://open.feishu.cn/document/im-v1/message/forward-2>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_THREADS,
    im::im::v1::message::models::ReceiveIdType,
};

/// 转发话题请求体
///
/// # 字段说明
///
/// - `receive_id`: 接收者 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = ForwardThreadBody::new("ou_xxx");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardThreadBody {
    receive_id: String,
}

impl ForwardThreadBody {
    /// 创建新的转发话题请求体
    pub fn new(receive_id: impl Into<String>) -> Self {
        Self {
            receive_id: receive_id.into(),
        }
    }

    /// 接收者 ID
    pub fn receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.receive_id = receive_id.into();
        self
    }
}

/// 转发话题请求
///
/// 用于将指定话题转发到其他接收者。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `thread_id`: 待转发的话题 ID，必填
/// - `receive_id_type`: 消息接收者 ID 类型，必填
/// - `uuid`: 幂等 uuid，可选
///
/// # 示例
///
/// ```rust,ignore
/// let body = ForwardThreadBody::new("ou_xxx");
/// let request = ForwardThreadRequest::new(config)
///     .thread_id("thread_xxx")
///     .receive_id_type(ReceiveIdType::OpenId)
///     .execute(body).await?;
/// ```
pub struct ForwardThreadRequest {
    config: Config,
    thread_id: String,
    receive_id_type: Option<ReceiveIdType>,
    uuid: Option<String>,
}

impl ForwardThreadRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            thread_id: String::new(),
            receive_id_type: None,
            uuid: None,
        }
    }

    /// 待转发的话题 ID（路径参数）
    pub fn thread_id(mut self, thread_id: impl Into<String>) -> Self {
        self.thread_id = thread_id.into();
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
    /// docPath: <https://open.feishu.cn/document/im-v1/message/forward-2>
    pub async fn execute(self, body: ForwardThreadBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: ForwardThreadBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.thread_id, "thread_id 不能为空");
        validate_required!(body.receive_id, "receive_id 不能为空");
        let receive_id_type = self.receive_id_type.ok_or_else(|| {
            error::validation_error(
                "receive_id_type 不能为空".to_string(),
                "转发话题需要指定 receive_id_type".to_string(),
            )
        })?;

        // url: POST:/open-apis/im/v1/threads/:thread_id/forward
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/forward", IM_V1_THREADS, self.thread_id))
                .query("receive_id_type", receive_id_type.as_str())
                .body(serialize_params(&body, "转发话题")?);

        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "转发话题")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_thread_body_builder() {
        let body = ForwardThreadBody::new("ou_xxx");
        assert_eq!(body.receive_id, "ou_xxx");
    }

    #[test]
    fn test_forward_thread_body_with_receive_id() {
        let body = ForwardThreadBody::new("ou_123").receive_id("ou_456");
        assert_eq!(body.receive_id, "ou_456");
    }

    #[test]
    fn test_forward_thread_request_builder() {
        let config = Config::default();
        let request = ForwardThreadRequest::new(config)
            .thread_id("thread_xxx")
            .receive_id_type(ReceiveIdType::OpenId);
        assert_eq!(request.thread_id, "thread_xxx");
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
    }

    #[test]
    fn test_forward_thread_request_default_values() {
        let config = Config::default();
        let request = ForwardThreadRequest::new(config);
        assert_eq!(request.thread_id, "");
        assert!(request.receive_id_type.is_none());
        assert!(request.uuid.is_none());
    }

    #[test]
    fn test_forward_thread_request_with_uuid() {
        let config = Config::default();
        let request = ForwardThreadRequest::new(config)
            .thread_id("thread_xxx")
            .receive_id_type(ReceiveIdType::UserId)
            .uuid("uuid-123");
        assert_eq!(request.uuid, Some("uuid-123".to_string()));
    }

    #[test]
    fn test_forward_thread_request_chaining() {
        let config = Config::default();
        let request = ForwardThreadRequest::new(config)
            .thread_id("thread_xxx")
            .receive_id_type(ReceiveIdType::OpenId)
            .uuid("uuid-456");
        assert_eq!(request.thread_id, "thread_xxx");
        assert_eq!(request.receive_id_type, Some(ReceiveIdType::OpenId));
    }
}
