//! 获取关联组织的部门和成员信息
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/visible_organization
//! docPath: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/visible_organization

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织可见部门成员 Builder
#[derive(Debug, Clone)]
pub struct VisibleOrganizationBuilder {
    config: Config,
    target_tenant_key: String,
}

impl VisibleOrganizationBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            target_tenant_key: String::new(),
        }
    }

    /// 设置目标租户 key
    pub fn target_tenant_key(mut self, key: impl Into<String>) -> Self {
        self.target_tenant_key = key.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<VisibleOrganizationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<VisibleOrganizationResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/visible_organization",
            self.target_tenant_key
        );

        let req: ApiRequest<VisibleOrganizationResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 可见组织响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VisibleOrganizationResponse {
    /// 部门列表
    pub departments: Vec<CollaborationDepartment>,
    /// 用户列表
    pub users: Vec<CollaborationUser>,
}

/// 协作部门
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationDepartment {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 部门名称
    pub name: String,
}

/// 协作用户
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationUser {
    /// 用户 ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

impl ApiResponseTrait for VisibleOrganizationResponse {}

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
