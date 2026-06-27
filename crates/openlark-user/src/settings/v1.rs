//! 个人设置 V1 API
//!
//! 当前仍是 runtime stub。
//!
//! 这些请求构建器已经对外暴露，但尚未接入真实的服务端实现。
//! 为避免继续返回占位 JSON，本模块现在会显式返回未接线错误。

use crate::UserConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 个人设置 V1 API
#[derive(Debug, Clone)]
pub struct SettingsV1 {
    /// 客户端配置
    config: Arc<UserConfig>,
}

impl SettingsV1 {
    /// 创建新的个人设置 V1 实例
    pub fn new(config: Arc<UserConfig>) -> Self {
        Self { config }
    }

    /// 获取设置
    pub fn get(&self) -> GetSettingRequest {
        GetSettingRequest::new(self.config.clone())
    }

    /// 更新设置
    pub fn update(&self) -> UpdateSettingRequest {
        UpdateSettingRequest::new(self.config.clone())
    }

    /// 获取所有设置
    pub fn list(&self) -> ListSettingsRequest {
        ListSettingsRequest::new(self.config.clone())
    }
}

/// 获取单个设置的请求构建器。
pub struct GetSettingRequest {
    // reserved：config 无读取者（不完整脚手架，见 #274/#276）
    _config: Arc<UserConfig>,
    key: Option<String>,
}

impl GetSettingRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            _config: config,
            key: None,
        }
    }

    /// 设置要查询的配置键。
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 执行请求并返回设置内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "settings.v1.get: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

/// 更新单个设置的请求构建器。
pub struct UpdateSettingRequest {
    // reserved：config 无读取者（不完整脚手架，见 #274/#276）
    _config: Arc<UserConfig>,
    key: Option<String>,
    value: Option<String>,
}

impl UpdateSettingRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self {
            _config: config,
            key: None,
            value: None,
        }
    }

    /// 设置要更新的配置键。
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// 设置新的配置值。
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
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
            "settings.v1.update: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
        ))
    }
}

/// 获取所有设置的请求构建器。
pub struct ListSettingsRequest {
    // reserved：config 无读取者（不完整脚手架，见 #274/#276）
    _config: Arc<UserConfig>,
}

impl ListSettingsRequest {
    fn new(config: Arc<UserConfig>) -> Self {
        Self { _config: config }
    }

    /// 执行请求并返回设置列表。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(business_error(
            "settings.v1.list: openlark-user 尚未接入该 runtime API，请等待后续真实端点支持",
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
    async fn test_settings_stub_returns_explicit_error() {
        let config = Arc::new(UserConfig::default());
        let err = SettingsV1::new(config)
            .get()
            .key("language")
            .execute()
            .await
            .expect_err("settings stub should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
