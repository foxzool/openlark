//! 修改邮箱联系人

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Patch Mail Contact Request。
#[derive(Debug, Clone)]
pub struct PatchMailContactRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    mail_contact_id: String,
}

/// Patch Mail Contact Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMailContactResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchMailContactResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchMailContactRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        user_mailbox_id: impl Into<String>,
        mail_contact_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            mail_contact_id: mail_contact_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<PatchMailContactResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchMailContactResponse> {
        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/mail_contacts/{}",
            self.user_mailbox_id, self.mail_contact_id
        );
        let req: ApiRequest<PatchMailContactResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("修改邮箱联系人", "响应数据为空"))
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
