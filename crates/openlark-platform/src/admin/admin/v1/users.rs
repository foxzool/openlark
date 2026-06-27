//! 用户管理 API
//!
//! 当前仍是 runtime stub。
//!
//! 平台 admin 已接入的真实用户统计接口是 `admin_user_stat/list.rs`，
//! 但这里的 `users` facade 并没有对应的已接线服务端端点。
//! 为避免继续返回占位 JSON，本模块现在会显式返回未接线错误。

use crate::PlatformConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 用户管理 API
#[derive(Debug, Clone)]
pub struct UsersApi {
    config: Arc<PlatformConfig>,
}

impl UsersApi {
    /// 创建新的用户管理 facade。
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 获取用户列表
    pub fn list(&self) -> ListAdminUsersRequest {
        ListAdminUsersRequest::new(self.config.clone())
    }

    /// 禁用用户
    pub fn disable(&self) -> DisableUserRequest {
        DisableUserRequest::new(self.config.clone())
    }

    /// 启用用户
    pub fn enable(&self) -> EnableUserRequest {
        EnableUserRequest::new(self.config.clone())
    }
}

/// 获取用户列表请求
pub struct ListAdminUsersRequest {
    config: Arc<PlatformConfig>,
    page_size: Option<u32>,
}

impl ListAdminUsersRequest {
    fn new(config: Arc<PlatformConfig>) -> Self {
        Self {
            config,
            page_size: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "admin.users.list: openlark-platform 尚未接入该 facade，请改用已实现的 admin_user_stat.list 等真实端点",
        ))
    }
}

/// 禁用用户请求
pub struct DisableUserRequest {
    config: Arc<PlatformConfig>,
    user_id: Option<String>,
}

impl DisableUserRequest {
    fn new(config: Arc<PlatformConfig>) -> Self {
        Self {
            config,
            user_id: None,
        }
    }

    /// 设置用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "admin.users.disable: openlark-platform 尚未接入该 facade，请等待后续真实端点支持",
        ))
    }
}

/// 启用用户请求
pub struct EnableUserRequest {
    config: Arc<PlatformConfig>,
    user_id: Option<String>,
}

impl EnableUserRequest {
    fn new(config: Arc<PlatformConfig>) -> Self {
        Self {
            config,
            user_id: None,
        }
    }

    /// 设置用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "admin.users.enable: openlark-platform 尚未接入该 facade，请等待后续真实端点支持",
        ))
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

    #[tokio::test]
    async fn test_users_stub_returns_explicit_error() {
        let config = Arc::new(PlatformConfig::default());
        let err = UsersApi::new(config)
            .list()
            .page_size(20)
            .execute()
            .await
            .expect_err("users stub should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
