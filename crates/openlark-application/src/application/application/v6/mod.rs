/// 应用资源接口。
pub mod app;

// app 模块显式导出

// app 模块类型经 app::* 访问

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV6：应用 API v6 访问入口
#[derive(Clone)]
pub struct ApplicationV6 {
    config: Arc<Config>,
}

impl ApplicationV6 {
    /// 创建新的 ApplicationV6 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问应用资源
    pub fn app(&self) -> app::App {
        app::App::new(self.config.clone())
    }
}
pub mod app_badge;
pub mod app_recommend_rule;
pub mod app_usage;
pub mod app_version;
pub mod app_visibility;
pub mod application;
pub mod collaborator;
pub mod contacts_range;
pub mod feedback;
pub mod frequently_used;
pub mod management;
pub mod owner;
pub mod scope;
pub mod usage;
pub mod visibility;
