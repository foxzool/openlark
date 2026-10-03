//! 数据表相关 API

pub mod list;
pub mod records_batch_update;
pub mod records_delete;
pub mod records_get;
pub mod records_patch;
pub mod records_post;
pub mod table_get;

use openlark_core::config::Config;

/// workspace.table 资源服务（叶子级，持 owned Config + workspace_id + table_name）
#[derive(Debug, Clone)]
pub struct TableService {
    config: Config,
    workspace_id: String,
    table_name: String,
}

impl TableService {
    /// 创建新的 table 服务
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
        }
    }
    /// 数据表列表
    pub fn list(&self) -> list::TableListRequestBuilder {
        list::TableListRequestBuilder::new(self.config.clone(), self.workspace_id.clone())
    }
    /// 获取数据表详情
    pub fn table_get(&self) -> table_get::TableGetRequestBuilder {
        table_get::TableGetRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }
    /// 新增记录
    pub fn records_post(&self) -> records_post::TableRecordsPostRequestBuilder {
        records_post::TableRecordsPostRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }
    /// 查询记录
    pub fn records_get(&self) -> records_get::TableRecordsGetRequestBuilder {
        records_get::TableRecordsGetRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }
    /// 删除记录
    pub fn records_delete(&self) -> records_delete::TableRecordsDeleteRequestBuilder {
        records_delete::TableRecordsDeleteRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }
    /// 批量更新记录
    pub fn records_batch_update(
        &self,
    ) -> records_batch_update::TableRecordsBatchUpdateRequestBuilder {
        records_batch_update::TableRecordsBatchUpdateRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
        )
    }
    /// 按过滤条件更新记录
    pub fn records_patch(
        &self,
        filter: impl Into<String>,
    ) -> records_patch::TableRecordsPatchRequestBuilder {
        records_patch::TableRecordsPatchRequestBuilder::new(
            self.config.clone(),
            self.workspace_id.clone(),
            self.table_name.clone(),
            filter,
        )
    }
}
