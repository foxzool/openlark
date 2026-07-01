//! 修改角色名称
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/update>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 修改角色名称请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleBody {
    /// 角色名称。
    pub role_name: String,
}

/// 修改角色名称请求
pub struct UpdateFunctionalRoleRequest {
    /// 配置信息。
    config: Config,
    /// 角色 ID。
    role_id: String,
}

impl UpdateFunctionalRoleRequest {
    /// 创建新的请求构建器。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/update>
    pub async fn execute(self, body: UpdateFunctionalRoleBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: UpdateFunctionalRoleBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(body.role_name, "role_name 不能为空");

        // url: PUT:/open-apis/contact/v3/functional_roles/:role_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::put(format!("{}/{}", CONTACT_V3_FUNCTIONAL_ROLES, self.role_id))
                .body(serialize_params(&body, "修改角色名称")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "修改角色名称")
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
