//! app module

/// 应用列表查询。
pub mod list;

use openlark_core::config::Config;

/// app 资源服务
#[derive(Debug, Clone)]
pub struct AppService {
    config: Config,
}

impl AppService {
    /// 创建新的 app 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 应用列表
    pub fn list(&self) -> list::ListAppRequestBuilder {
        list::ListAppRequestBuilder::new(self.config.clone())
    }
}
