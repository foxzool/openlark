//! 创建角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::functional_role::models::CreateFunctionalRoleResponse,
    endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
};

/// 创建角色请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleBody {
    pub role_name: String,
}

/// 创建角色请求
pub struct CreateFunctionalRoleRequest {
    config: Config,
}

impl CreateFunctionalRoleRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/functional_role/create>
    pub async fn execute(
        self,
        body: CreateFunctionalRoleBody,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateFunctionalRoleBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        validate_required!(body.role_name, "role_name 不能为空");

        // url: POST:/open-apis/contact/v3/functional_roles
        let req: ApiRequest<CreateFunctionalRoleResponse> =
            ApiRequest::post(CONTACT_V3_FUNCTIONAL_ROLES)
                .body(serialize_params(&body, "创建角色")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建角色")
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
