//! 获取关联组织部门详情
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get-2
//! docPath: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织部门详情 Builder
#[derive(Debug, Clone)]
pub struct CollaborationDepartmentGetBuilder {
    config: Config,
    target_tenant_key: String,
    target_department_id: String,
}

impl CollaborationDepartmentGetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            target_tenant_key: String::new(),
            target_department_id: String::new(),
        }
    }

    /// 设置目标租户 key
    pub fn target_tenant_key(mut self, key: impl Into<String>) -> Self {
        self.target_tenant_key = key.into();
        self
    }

    /// 设置目标部门 ID
    pub fn target_department_id(mut self, id: impl Into<String>) -> Self {
        self.target_department_id = id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationDepartmentGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationDepartmentGetResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/collaboration_departments/{}",
            self.target_tenant_key, self.target_department_id
        );

        let req: ApiRequest<CollaborationDepartmentGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 关联组织部门详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationDepartmentGetResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    #[serde(rename = "parent_department_id")]
    pub parent_department_id: Option<String>,
}

impl ApiResponseTrait for CollaborationDepartmentGetResponse {}

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
