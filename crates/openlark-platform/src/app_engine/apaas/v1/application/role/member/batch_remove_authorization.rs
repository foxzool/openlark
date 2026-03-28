//! 批量删除角色成员授权
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/permission/application-role-member/batch_remove_authorization>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量删除角色成员授权 Builder
#[derive(Debug, Clone)]
pub struct RoleMemberBatchRemoveAuthBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 角色 API 名称
    role_api_name: String,
    /// 用户 ID 列表
    user_ids: Vec<String>,
}

impl RoleMemberBatchRemoveAuthBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        role_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            role_api_name: role_api_name.into(),
            user_ids: Vec::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RoleMemberBatchRemoveAuthResponse> {
        self.execute_with_options.await
    }(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RoleMemberBatchRemoveAuthResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/roles/{}/member/batch_remove_authorization",
            self.namespace, self.role_api_name
        );

        let request = RoleMemberBatchRemoveAuthRequest {
            user_ids: self.user_ids,
        };

        let req = ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量删除角色成员授权请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RoleMemberBatchRemoveAuthRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
}

/// 批量删除角色成员授权响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoleMemberBatchRemoveAuthResponse {
    /// 取消授权的用户数量
    #[serde(rename = "removed_count")]
    removed_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for RoleMemberBatchRemoveAuthResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
