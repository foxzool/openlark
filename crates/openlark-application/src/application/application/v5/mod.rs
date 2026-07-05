//! v5 资源模块

pub mod application;

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV5：应用 API v5 访问入口。
#[derive(Clone)]
pub struct ApplicationV5 {
    config: Arc<Config>,
}

impl ApplicationV5 {
    /// 创建新的 ApplicationV5 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 application 资源（favourite / recommend）。
    pub fn application(&self) -> application::Application {
        application::Application::new(self.config.clone())
    }
}
