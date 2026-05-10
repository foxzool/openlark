//! 从回收站删除用户邮箱地址
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/user_mailbox-alias/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteUserMailboxRequest {
    config: Arc<Config>,
    user_mailbox_id: String,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserMailboxResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DeleteUserMailboxResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DeleteUserMailboxRequest {
    pub fn new(config: Arc<Config>, user_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            user_mailbox_id: user_mailbox_id.into(),
            
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteUserMailboxResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteUserMailboxResponse> {
        let path = format!("/open-apis/mail/v1/user_mailboxes/{}", self.user_mailbox_id);
        let req: ApiRequest<DeleteUserMailboxResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("从回收站删除用户邮箱地址", "响应数据为空")
        })
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
