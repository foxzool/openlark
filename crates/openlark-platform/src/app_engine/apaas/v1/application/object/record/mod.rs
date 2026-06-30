//! 记录相关 API

pub mod batch_create;
pub mod batch_delete;
pub mod batch_query;
pub mod batch_update;
pub mod create;
pub mod delete;
pub mod patch;
pub mod query;

use openlark_core::config::Config;

/// object.record 资源服务（叶子级，持 owned Config + namespace + object_api_name）
#[derive(Debug, Clone)]
pub struct RecordService {
    config: Config,
    namespace: String,
    object_api_name: String,
}

impl RecordService {
    /// 创建新的 record 服务
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
        }
    }
    /// 新增记录
    pub fn create(&self) -> create::RecordCreateRequestBuilder {
        create::RecordCreateRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
    /// 批量新增记录
    pub fn batch_create(&self) -> batch_create::RecordBatchCreateRequestBuilder {
        batch_create::RecordBatchCreateRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
    /// 查询记录
    pub fn query(&self, record_id: impl Into<String>) -> query::RecordQueryRequestBuilder {
        query::RecordQueryRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
            record_id,
        )
    }
    /// 更新记录
    pub fn patch(&self, record_id: impl Into<String>) -> patch::RecordPatchRequestBuilder {
        patch::RecordPatchRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
            record_id,
        )
    }
    /// 删除记录
    pub fn delete(&self, record_id: impl Into<String>) -> delete::RecordDeleteRequestBuilder {
        delete::RecordDeleteRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
            record_id,
        )
    }
    /// 批量删除记录
    pub fn batch_delete(&self) -> batch_delete::RecordBatchDeleteRequestBuilder {
        batch_delete::RecordBatchDeleteRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
    /// 批量更新记录
    pub fn batch_update(&self) -> batch_update::RecordBatchUpdateRequestBuilder {
        batch_update::RecordBatchUpdateRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
    /// 批量查询记录
    pub fn batch_query(&self) -> batch_query::RecordBatchQueryRequestBuilder {
        batch_query::RecordBatchQueryRequestBuilder::new(
            self.config.clone(),
            self.namespace.clone(),
            self.object_api_name.clone(),
        )
    }
}
