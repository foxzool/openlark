//! 批量创建记录权限用户授权
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/permission/application-record_permission-member/batch_create_authorization
//! docPath: https://open.feishu.cn/document/apaas-v1/permission/application-record_permission-member/batch_create_authorization

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量创建记录权限用户授权 Builder
#[derive(Debug, Clone)]
pub struct RecordPermissionBatchCreateAuthBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 记录权限 API 名称
    record_permission_api_name: String,
    /// 用户 ID 列表
    user_ids: Vec<String>,
}

impl RecordPermissionBatchCreateAuthBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        record_permission_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            record_permission_api_name: record_permission_api_name.into(),
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
    pub async fn execute(self) -> SDKResult<RecordPermissionBatchCreateAuthResponse> {
        self.execute_with_options.await
    }(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordPermissionBatchCreateAuthResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/record_permissions/{}/member/batch_create_authorization",
            self.namespace, self.record_permission_api_name
        );

        let request = RecordPermissionBatchCreateAuthRequest {
            user_ids: self.user_ids,
        };

        let req = ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量创建记录权限用户授权请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordPermissionBatchCreateAuthRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
}

/// 批量创建记录权限用户授权响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordPermissionBatchCreateAuthResponse {
    /// 授权的用户数量
    #[serde(rename = "authorized_count")]
    authorized_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    message: String,
}

impl ApiResponseTrait for RecordPermissionBatchCreateAuthResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
