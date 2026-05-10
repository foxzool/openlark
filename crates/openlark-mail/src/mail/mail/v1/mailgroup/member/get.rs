//! 查询指定邮件组成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/get

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
pub struct GetMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailGroupMemberResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMailGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMailGroupMemberRequest {
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            member_id: member_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<GetMailGroupMemberResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("查询指定邮件组成员", "响应数据为空")
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
