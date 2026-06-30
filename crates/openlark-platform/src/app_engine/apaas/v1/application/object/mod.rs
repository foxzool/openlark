//! 应用对象相关 API

pub mod oql_query;
pub mod record;
pub mod search;

pub use oql_query::OqlQueryRequestBuilder;
pub use record::batch_create as record_batch_create;
pub use record::batch_delete as record_batch_delete;
pub use record::batch_query as record_batch_query;
pub use record::batch_update as record_batch_update;
pub use record::create as record_create;
pub use record::delete as record_delete;
pub use record::patch as record_patch;
pub use record::query as record_query;
pub use search::RecordSearchRequestBuilder;

use crate::PlatformConfig;
use std::sync::Arc;

/// application.object 资源服务（中间级，绑 namespace + object_api_name）
#[derive(Debug, Clone)]
pub struct ObjectService {
    config: Arc<PlatformConfig>,
    namespace: String,
    object_api_name: String,
}

impl ObjectService {
    /// 创建新的 object 服务
    pub fn new(
        config: Arc<PlatformConfig>,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
        }
    }
    /// object.record 子资源（叶子级）
    pub fn record(&self) -> record::RecordService {
        record::RecordService::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
    /// 记录搜索
    pub fn search(&self, search: impl Into<String>) -> search::RecordSearchRequestBuilder {
        search::RecordSearchRequestBuilder::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            search,
        )
    }
    /// OQL 查询
    pub fn oql_query(&self, oql: impl Into<String>) -> oql_query::OqlQueryRequestBuilder {
        oql_query::OqlQueryRequestBuilder::new(
            self.config.as_ref().clone(),
            self.namespace.clone(),
            oql,
        )
    }
}
