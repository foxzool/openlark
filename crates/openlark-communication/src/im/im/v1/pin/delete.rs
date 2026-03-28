//! 移除 Pin 消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/pin/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::IM_V1_PINS,
};

/// 移除 Pin 消息请求
///
/// 用于移除群组中的 Pin 消息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeletePinRequest::new(config)
///     .message_id("msg_xxx")
///     .execute().await?;
/// ```
pub struct DeletePinRequest {
    config: Config,
    message_id: String,
}

impl DeletePinRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/pin/delete>
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

        // url: DELETE:/open-apis/im/v1/pins/:message_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", IM_V1_PINS, self.message_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "移除 Pin 消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_pin_request_builder() {
        let config = Config::default();
        let request = DeletePinRequest::new(config).message_id("msg_xxx");
        assert_eq!(request.message_id, "msg_xxx");
    }

    #[test]
    fn test_delete_pin_request_default_values() {
        let config = Config::default();
        let request = DeletePinRequest::new(config);
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_delete_pin_request_with_different_ids() {
        let config = Config::default();
        let request1 = DeletePinRequest::new(config.clone()).message_id("msg_111");
        let request2 = DeletePinRequest::new(config).message_id("msg_222");
        assert_eq!(request1.message_id, "msg_111");
        assert_eq!(request2.message_id, "msg_222");
    }
}
