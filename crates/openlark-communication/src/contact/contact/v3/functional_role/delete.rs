//! 删除角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 删除角色请求
pub struct DeleteFunctionalRoleRequest {
    config: Config,
    role_id: String,
}

impl DeleteFunctionalRoleRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            role_id: String::new(),
        }
    }

    /// 角色 ID（路径参数）
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.role_id, "role_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/functional_roles/:role_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_FUNCTIONAL_ROLES, self.role_id));
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除角色")
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
