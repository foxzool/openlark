//! 工作台 V1 API

pub mod custom_workplace_access_data;
pub mod workplace_access_data;
pub mod workplace_block_access_data;

use openlark_core::config::Config;
use std::sync::Arc;

/// WorkplaceV1：工作台 API v1 访问入口。
///
/// 每个 access_data 资源仅一个 search 端点，故直接返回请求构建器（不引入单方法 resource 中间层）。
#[derive(Clone)]
pub struct WorkplaceV1 {
    config: Arc<Config>,
}

impl WorkplaceV1 {
    /// 创建新的 WorkplaceV1 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 搜索自定义工作台访问数据。
    pub fn custom_workplace_access_data(
        &self,
    ) -> custom_workplace_access_data::search::AccessDataSearchCustomRequestBuilder {
        custom_workplace_access_data::search::AccessDataSearchCustomRequestBuilder::new(
            self.config.as_ref().clone(),
        )
    }

    /// 搜索工作台访问数据。
    pub fn workplace_access_data(
        &self,
    ) -> workplace_access_data::search::AccessDataSearchWorkplaceRequestBuilder {
        workplace_access_data::search::AccessDataSearchWorkplaceRequestBuilder::new(
            self.config.as_ref().clone(),
        )
    }

    /// 搜索工作台小组件访问数据。
    pub fn workplace_block_access_data(
        &self,
    ) -> workplace_block_access_data::search::AccessDataSearchBlockRequestBuilder {
        workplace_block_access_data::search::AccessDataSearchBlockRequestBuilder::new(
            self.config.as_ref().clone(),
        )
    }
}
