//! Pin 消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/pin/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_PINS,
    im::im::v1::pin::models::{CreatePinBody, CreatePinResponse},
};

/// Pin 消息请求
///
///用于在群组中 Pin 一条消息，使其固定显示在聊天界面顶部。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreatePinBody::new("msg_xxx", "oc_xxx");
/// let request = CreatePinRequest::new(config)
///     .execute(body).await?;
/// ```
pub struct CreatePinRequest {
    config: Config,
}

impl CreatePinRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/pin/create>
    pub async fn execute(self, body: CreatePinBody) -> SDKResult<CreatePinResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreatePinBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePinResponse> {
        // === 必填字段验证 ===
        validate_required!(body.message_id, "message_id 不能为空");

        // url: POST:/open-apis/im/v1/pins
        let req: ApiRequest<CreatePinResponse> =
            ApiRequest::post(IM_V1_PINS).body(serialize_params(&body, "Pin 消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "Pin 消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pin_request_builder() {
        let config = Config::default();
        let request = CreatePinRequest::new(config);
        // Just verify the request can be created
        assert_eq!(request.config.app_id(), "");
    }

    #[test]
    fn test_create_pin_request_new() {
        let config = Config::default();
        let request = CreatePinRequest::new(config);
        // Verify the request struct is properly initialized
        assert_eq!(request.config.app_id(), "");
    }
}
