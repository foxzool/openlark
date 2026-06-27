//! 用户偏好 V1 API
//!
//! 当前仍是 runtime stub。
//!
//! 这些请求构建器已经对外暴露，但尚未接入真实的服务端实现。
//! 为避免继续返回占位 JSON，本模块现在会显式返回未接线错误。

use crate::UserConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 用户偏好 V1 API
#[derive(Debug, Clone)]
pub struct PreferencesV1 {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl PreferencesV1 {
    /// 创建新的用户偏好 V1 实例
    pub fn new(config: Arc<UserConfig>) -> Self {
        Self { config }
    }

    /// 获取偏好
    pub fn get(&self) -> GetPreferenceRequest {
        GetPreferenceRequest::new(self.config.clone())
    }

    /// 更新偏好
    pub fn update(&self) -> UpdatePreferenceRequest {
        UpdatePreferenceRequest::new(self.config.clone())
    }

    /// 删除偏好
    pub fn delete(&self) -> DeletePreferenceRequest {
        DeletePreferenceRequest::new(self.config.clone())
    }

    /// 获取所有偏好
    pub fn list(&self) -> ListPreferencesRequest {
        ListPreferencesRequest::new(self.config.clone())
    }
}

/// 获取单个偏好的请求构建器。
pub struct GetPreferenceRequest {
    config: Arc<UserConfig>,
    key: Option<String>,
}

impl GetPreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self { config, key: None }
    }

    /// 设置要查询的偏好键。
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求并返回偏好内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "preferences.v1.get: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

/// 更新单个偏好的请求构建器。
pub struct UpdatePreferenceRequest {
    config: Arc<UserConfig>,
    key: Option<String>,
    value: Option<String>,
    category: Option<String>,
}

impl UpdatePreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            config,
            key: None,
            value: None,
            category: None,
        }
    }

    /// 设置要更新的偏好键。
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置新的偏好值。
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置偏好所属类别。
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 执行请求并返回更新结果。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "preferences.v1.update: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

/// 删除单个偏好的请求构建器。
pub struct DeletePreferenceRequest {
    config: Arc<UserConfig>,
    key: Option<String>,
}

impl DeletePreferenceRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self { config, key: None }
    }

    /// 设置要删除的偏好键。
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求并返回删除结果。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "preferences.v1.delete: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

/// 获取偏好列表的请求构建器。
pub struct ListPreferencesRequest {
    config: Arc<UserConfig>,
    category: Option<String>,
}

impl ListPreferencesRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            config,
            category: None,
        }
    }

    /// 按类别过滤偏好列表。
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// 执行请求并返回偏好列表。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "preferences.v1.list: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
    async fn test_preferences_stub_returns_explicit_error() {
        let config = Arc::new(UserConfig::default());
        let err = PreferencesV1::new(config)
            .list()
            .category("shortcuts")
            .execute()
            .await
            .expect_err("preferences stub should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
