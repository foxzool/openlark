//! 撤回消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::{api_utils::extract_response_data, models::EmptyData};
use crate::endpoints::IM_V1_MESSAGES;

/// 撤回消息请求
///
/// 用于撤回已发送的消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 待撤回的消息 ID，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::DeleteMessageRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = DeleteMessageRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute().await?;
/// ```
pub struct DeleteMessageRequest {
    config: Config,
    message_id: String,
}

impl DeleteMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待撤回的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");

        // url: DELETE:/open-apis/im/v1/messages/:message_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", IM_V1_MESSAGES, self.message_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "撤回消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_message_request_builder() {
        let config = Config::default();
        let request = DeleteMessageRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = DeleteMessageRequest::new(config);
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_message_id_with_str() {
        let config = Config::default();
        let request = DeleteMessageRequest::new(config).message_id("msg_12345");
        assert_eq!(request.message_id, "msg_12345");
    }

    #[test]
    fn test_message_id_with_string() {
        let config = Config::default();
        let request = DeleteMessageRequest::new(config).message_id(String::from("msg_67890"));
        assert_eq!(request.message_id, "msg_67890");
    }
}
