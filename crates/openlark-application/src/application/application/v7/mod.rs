/// app_avatar 模块。
pub mod app_avatar;
/// application 模块。
pub mod application;

use openlark_core::config::Config;
use std::sync::Arc;

/// ApplicationV7：应用 API v7 访问入口。
#[derive(Clone)]
pub struct ApplicationV7 {
    config: Arc<Config>,
}

impl ApplicationV7 {
    /// 创建新的 ApplicationV7 实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 app_avatar 资源。
    pub fn app_avatar(&self) -> app_avatar::AppAvatar {
        app_avatar::AppAvatar::new(self.config.clone())
    }

    /// 访问 application 资源（ability / base / config / publish）。
    pub fn application(&self) -> application::Application {
        application::Application::new(self.config.clone())
    }
}
