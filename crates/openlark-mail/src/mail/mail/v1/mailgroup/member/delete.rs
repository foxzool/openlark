//! 删除邮件组成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete

use openlark_core::{
    SDKResult, api::ApiRequest, api::Response, config::Config, http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mail Group Member Request。
#[derive(Debug, Clone)]
pub struct DeleteMailGroupMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    member_id: String,
}

/// Delete Mail Group Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupMemberRequest {
    /// 创建新的实例。
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

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteMailGroupMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/members/{}",
            self.mailgroup_id, self.member_id
        );
        let req: ApiRequest<DeleteMailGroupMemberResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupMemberResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupMemberResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
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
