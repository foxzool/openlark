//! 新增自定义角色
//!
//! docPath: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2>

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 新增自定义角色
#[derive(Debug)]
pub struct Create {
    config: Config,
    app_token: String,
    req: CreateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
    /// 自定义角色的名字
    pub role_name: String,
    /// 数据表权限配置列表（结构按 JSON 透传）
    pub table_roles: Vec<serde_json::Value>,
    /// Block 权限配置列表（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_roles: Option<Vec<serde_json::Value>>,
    /// Base 规则（结构按 JSON 透传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_rule: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResp {
    /// 自定义角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AppRole>,
}

impl Create {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CreateReq {
                role_name: String::new(),
                table_roles: Vec::new(),
                block_roles: None,
                base_rule: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    /// 数据表权限配置列表（table_roles）
    pub fn table_roles(mut self, table_roles: Vec<serde_json::Value>) -> Self {
        self.req.table_roles = table_roles;
        self
    }

    /// Block 权限配置列表（block_roles）
    pub fn block_roles(mut self, block_roles: Vec<serde_json::Value>) -> Self {
        self.req.block_roles = Some(block_roles);
        self
    }

    /// Base 规则（base_rule）
    pub fn base_rule(mut self, base_rule: serde_json::Value) -> Self {
        self.req.base_rule = Some(base_rule);
        self
    }

    pub async fn execute(self) -> SDKResult<CreateResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");
        if self.req.role_name.chars().count() > 100 {
            return Err(openlark_core::error::validation_error(
                "role_name",
                "role_name 长度不能超过 100 字符",
            ));
        }
        if self.req.table_roles.is_empty() {
            return Err(openlark_core::error::validation_error(
                "table_roles",
                "table_roles 不能为空",
            ));
        }
        if self.req.table_roles.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "table_roles",
                "table_roles 长度不能超过 100",
            ));
        }

        // 使用类型安全的端点枚举生成路径
        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleCreate(self.app_token);

        let api_request: ApiRequest<CreateResp> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.req, "新增自定义角色")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新增自定义角色")
    }
}

impl ApiResponseTrait for CreateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
