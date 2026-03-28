//! 批量发送消息（旧版）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/send-messages-in-batches>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};

const IM_MESSAGE_V4_BATCH_SEND: &str = "/open-apis/message/v4/batch_send/";

/// 批量发送消息请求
pub struct BatchSendMessagesRequest {
    config: Config,
}

impl BatchSendMessagesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口为旧版批量发送接口，请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/send-messages-in-batches>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/message/v4/batch_send/
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(IM_MESSAGE_V4_BATCH_SEND)
            .body(serialize_params(&body, "批量发送消息")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "批量发送消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
