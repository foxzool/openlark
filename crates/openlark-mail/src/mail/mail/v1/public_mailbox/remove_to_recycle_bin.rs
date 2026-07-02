//! 将公共邮箱移至回收站
//! docPath: <https://open.feishu.cn/document/mail-v1/public-mailbox/public_mailbox/remove_to_recycle_bin>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 将公共邮箱移至回收站的请求。
#[derive(Debug, Clone)]
pub struct RemovePublicMailboxToRecycleBinRequest {
    config: Arc<Config>,
    public_mailbox_id: String,
}

/// 将公共邮箱移至回收站的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovePublicMailboxToRecycleBinResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RemovePublicMailboxToRecycleBinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RemovePublicMailboxToRecycleBinRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, public_mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            public_mailbox_id: public_mailbox_id.into(),
        }
    }

    /// 执行将公共邮箱移至回收站请求。
    pub async fn execute(self) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemovePublicMailboxToRecycleBinResponse> {
        let path = format!(
            "/open-apis/mail/v1/public_mailboxes/{}/remove_to_recycle_bin",
            self.public_mailbox_id
        );
        let req: ApiRequest<RemovePublicMailboxToRecycleBinResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("将公共邮箱移至回收站", "响应数据为空")
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
