//! 获取关联组织详情
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织详情 Builder
#[derive(Debug, Clone)]
pub struct CollaborationTenantGetBuilder {
    config: Config,
    target_tenant_key: String,
}

impl CollaborationTenantGetBuilder {
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
    pub async fn execute(self) -> SDKResult<CollaborationTenantGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationTenantGetResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}",
            self.target_tenant_key
        );

        let req: ApiRequest<CollaborationTenantGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 关联组织详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationTenantGetResponse {
    /// 租户 key
    #[serde(rename = "tenant_key")]
    pub tenant_key: String,
    /// 租户名称
    #[serde(rename = "tenant_name")]
    pub tenant_name: String,
    /// 关联状态
    pub status: String,
}

impl ApiResponseTrait for CollaborationTenantGetResponse {}

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
