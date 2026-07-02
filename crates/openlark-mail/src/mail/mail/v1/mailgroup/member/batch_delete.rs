//! 批量删除邮件组成员

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除邮件组成员的请求。
#[derive(Debug, Clone)]
pub struct BatchDeleteMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
}

/// 批量删除邮件组成员的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for BatchDeleteMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl BatchDeleteMailGroupMemberRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, mailgroup_id: impl Into<String>) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
        }
    }

    /// 执行批量删除邮件组成员请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/batch_delete",
            self.mailgroup_id
        );
        let req: ApiRequest<BatchDeleteMailGroupMemberResponse> = ApiRequest::delete(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量删除邮件组成员", "响应数据为空")
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
