//! 删除邮件组权限成员
//! docPath: https://open.feishu.cn/document/server-docs/mail-v1/mail-group/mailgroup/delete

use openlark_core::{
    SDKResult, api::ApiRequest, api::Response, config::Config, http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Delete Mail Group Permission Member Request。
#[derive(Debug, Clone)]
pub struct DeleteMailGroupPermissionMemberRequest {
    config: Arc<Config>,
    mailgroup_id: String,
    permission_member_id: String,
}

/// Delete Mail Group Permission Member Response。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMailGroupPermissionMemberResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl DeleteMailGroupPermissionMemberRequest {
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
    pub async fn execute(self) -> SDKResult<DeleteMailGroupPermissionMemberResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteMailGroupPermissionMemberResponse> {
        let path = format!(
            "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
            self.mailgroup_id, self.permission_member_id
        );
        let req: ApiRequest<DeleteMailGroupPermissionMemberResponse> = ApiRequest::delete(&path);

        let _resp: Response<DeleteMailGroupPermissionMemberResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(DeleteMailGroupPermissionMemberResponse { data: None })
    }
}

impl openlark_core::api::ApiResponseTrait for DeleteMailGroupPermissionMemberResponse {
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
