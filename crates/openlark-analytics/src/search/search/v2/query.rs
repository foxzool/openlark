//! 查询搜索 API
//!
//! 当前仍是 runtime stub。
//!
//! 该模块历史上暴露了查询 / 搜索建议入口，但并没有对应的已接线服务端实现。
//! 为避免继续返回占位 JSON，本模块现在会显式返回未接线错误。

use crate::AnalyticsConfig;
use openlark_core::{SDKResult, error::business_error, req_option::RequestOption};
use std::sync::Arc;

/// 查询搜索 API
#[derive(Debug, Clone)]
pub struct QueryApi {
    config: Arc<AnalyticsConfig>,
}

impl QueryApi {
    /// 创建新的查询搜索 facade。
    pub fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self { config }
    }

    /// 搜索
    pub fn search(&self) -> SearchRequest {
        SearchRequest::new(self.config.clone())
    }

    /// 获取搜索建议
    pub fn suggest(&self) -> SuggestRequest {
        SuggestRequest::new(self.config.clone())
    }
}

/// 搜索请求
pub struct SearchRequest {
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<AnalyticsConfig>,
    search_term: Option<String>,
    search_type: Option<String>,
    page_size: Option<u32>,
}

impl SearchRequest {
    fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self {
            _config: config,
            search_term: None,
            search_type: None,
            page_size: None,
        }
    }

    /// 设置搜索词
    pub fn search_term(mut self, term: impl Into<String>) -> Self {
        self.search_term = Some(term.into());
        self
    }

    /// 设置搜索类型
    pub fn search_type(mut self, search_type: impl Into<String>) -> Self {
        self.search_type = Some(search_type.into());
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, size: u32) -> Self {
        self.page_size = Some(size);
        self
    }

    fn unsupported(operation: &str) -> openlark_core::error::CoreError {
        business_error(format!(
            "{operation}: openlark-analytics 尚未接入该 search runtime API，请改用已实现的 doc_wiki/schema/app/message 路径"
        ))
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
        Err(Self::unsupported("query.search"))
    }
}

/// 搜索建议请求
pub struct SuggestRequest {
    // reserved：待装访问器/execute（见 #274，不完整脚手架）
    _config: Arc<AnalyticsConfig>,
    query: Option<String>,
}

impl SuggestRequest {
    fn new(config: Arc<AnalyticsConfig>) -> Self {
        Self {
            _config: config,
            query: None,
        }
    }

    /// 设置查询词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    fn unsupported(operation: &str) -> openlark_core::error::CoreError {
        business_error(format!(
            "{operation}: openlark-analytics 尚未接入该 search runtime API，请改用已实现的 doc_wiki/schema/app/message 路径"
        ))
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
        Err(Self::unsupported("query.suggest"))
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
    async fn test_query_stub_returns_explicit_error() {
        let config = Arc::new(AnalyticsConfig::default());
        let err = QueryApi::new(config)
            .search()
            .search_term("项目文档")
            .execute()
            .await
            .expect_err("query search should now fail explicitly");
        assert!(err.to_string().contains("尚未接入"));
    }
}
