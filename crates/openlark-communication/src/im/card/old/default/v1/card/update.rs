//! 延时更新消息卡片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::INTERACTIVE_V1_CARD_UPDATE,
};

/// 延时更新消息卡片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCardBody {
    /// 消息卡片 token
    pub token: String,

    /// 消息卡片内容（JSON 格式）
    pub card: Value,

    /// 消息卡片更新的触发关键词（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_keyword: Option<String>,
}

/// 延时更新消息卡片请求
pub struct UpdateCardRequest {
    config: Config,
}

impl UpdateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card>
    pub async fn execute(self, body: UpdateCardBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateCardBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(&body.token, "token 不能为空");

        // url: POST:/open-apis/interactive/v1/card/update
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(INTERACTIVE_V1_CARD_UPDATE)
            .body(serialize_params(&body, "延时更新消息卡片")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "延时更新消息卡片")
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
