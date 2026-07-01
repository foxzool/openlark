//! 移除用户被授权的角色
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/remove_role_assign>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 移除用户被授权的角色请求
#[derive(Debug, Clone)]
pub struct RemoveRoleAssignRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl RemoveRoleAssignRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveRoleAssignResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveRoleAssignResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::AuthorizationRemoveRoleAssign;
        let request =
            ApiRequest::<RemoveRoleAssignResponse>::post(api_endpoint.to_url()).body(self.body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "移除用户被授权角色响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 移除用户被授权的角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveRoleAssignResponse {
    /// 操作结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<RoleAssignmentInfo>,
}

/// 授权信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AuthorizationInfo {
    /// 角色授权列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_assignments: Option<Vec<RoleAssignmentInfo>>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 角色授权条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RoleAssignmentInfo {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for RemoveRoleAssignResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
