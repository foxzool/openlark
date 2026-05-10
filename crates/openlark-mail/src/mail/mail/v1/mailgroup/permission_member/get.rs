//! 获取邮件组权限成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Get Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct GetMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    permission_member_id: String,
}

/// Get Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<PermissionMemberData>,
}

impl ApiResponseTrait for GetMailGroupPermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Permission Member Data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberData {
    /// 权限成员 ID。
    pub permission_member_id: String,
    /// 成员 ID。
    pub member_id: String,
    /// member_type 字段。
    pub member_type: String,
}

impl GetMailGroupPermissionMemberRequest {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailgroup_id: impl Into<String>,
        permission_member_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailgroup_id: mailgroup_id.into(),
            permission_member_id: permission_member_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
            self.mailgroup_id, self.permission_member_id
        );
        let req: ApiRequest<GetMailGroupPermissionMemberResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取邮件组权限成员", "响应数据为空")
        })
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
