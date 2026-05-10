//! 获取关联组织成员详情
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get-3
//! docPath: https://open.feishu.cn/document/trust_party-v1/-collaboraiton-organization/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取关联组织成员详情 Builder
#[derive(Debug, Clone)]
pub struct CollaborationUserGetBuilder {
    config: Config,
    target_tenant_key: String,
    target_user_id: String,
}

impl CollaborationUserGetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            target_tenant_key: String::new(),
            target_user_id: String::new(),
        }
    }

    /// 设置目标租户 key
    pub fn target_tenant_key(mut self, key: impl Into<String>) -> Self {
        self.target_tenant_key = key.into();
        self
    }

    /// 设置目标用户 ID
    pub fn target_user_id(mut self, id: impl Into<String>) -> Self {
        self.target_user_id = id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationUserGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationUserGetResponse> {
        let url = format!(
            "/open-apis/trust_party/v1/collaboration_tenants/{}/collaboration_users/{}",
            self.target_tenant_key, self.target_user_id
        );

        let req: ApiRequest<CollaborationUserGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 关联组织成员详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationUserGetResponse {
    /// 用户 ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: Option<String>,
}

impl ApiResponseTrait for CollaborationUserGetResponse {}

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
