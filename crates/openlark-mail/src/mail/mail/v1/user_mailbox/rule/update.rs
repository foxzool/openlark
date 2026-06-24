//! 更新收信规则

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
/// 待补充文档。
pub struct UpdateMailboxRuleRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    rule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct UpdateMailboxRuleResponse {
    /// 待补充文档。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for UpdateMailboxRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateMailboxRuleRequest {
    /// 待补充文档。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        rule_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            rule_id: rule_id.into(),
        }
    }

    /// 待补充文档。
    pub async fn execute(self) -> SDKResult<UpdateMailboxRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 待补充文档。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpdateMailboxRuleResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/rules/{}",
            self.user_mailbox_id, self.rule_id
        );
        let req: ApiRequest<UpdateMailboxRuleResponse> = ApiRequest::put(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("更新收信规则", "响应数据为空"))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
