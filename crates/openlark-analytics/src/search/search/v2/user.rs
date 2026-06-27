//! 用户搜索 API
//!
//! 当前仍是 runtime stub。
//!
//! 该入口没有对应的已接线服务端实现，因此现在会显式返回未接线错误，
//! 而不再伪装成成功返回占位 JSON。

use crate::AnalyticsConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 用户搜索 API
#[derive(Debug, Clone)]
pub struct UserSearchApi {
    config: Arc<AnalyticsConfig>,
}

impl UserSearchApi {
    /// 创建新的用户搜索 facade。
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 搜索用户
    pub fn search(&self) -> SearchUserRequest {
        SearchUserRequest::new(self.config.clone())
    }
}

/// 搜索用户请求
pub struct SearchUserRequest {
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<AnalyticsConfig>,
    query: Option<String>,
    page_size: Option<u32>,
}

impl SearchUserRequest {
    fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self {
            _config: config,
            query: None,
            page_size: None,
        }
    }

    /// 设置查询词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    fn unsupported() -> openlark_core::error::CoreError {
        business_error(
            "user.search: openlark-analytics 尚未接入用户搜索 runtime API，请等待后续真实端点支持",
        )
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求并传入请求选项。
    pub async fn execute_with_options(
        self,
        _option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        Err(Self::unsupported())
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
    async fn test_user_search_stub_returns_explicit_error() {
        let config = Arc::new(AnalyticsConfig::default());
        let err = UserSearchApi::new(config)
            .search()
            .query("zool")
            .execute()
            .await
            .expect_err("user search should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
