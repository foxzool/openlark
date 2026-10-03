//! application 资源模块

pub mod favourite;
pub mod recommend;

use openlark_core::config::Config;
use std::sync::Arc;

/// Application：v5 application 资源（favourite / recommend）。
#[derive(Clone)]
pub struct Application {
    config: Arc<Config>,
}

impl Application {
    /// 创建新的 Application 资源实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取推荐应用列表。
    pub fn favourite(&self) -> favourite::GetFavouriteAppsRequest {
        favourite::GetFavouriteAppsRequest::new(self.config.clone())
    }

    /// 获取推荐应用。
    pub fn recommend(&self) -> recommend::GetRecommendedAppsRequest {
        recommend::GetRecommendedAppsRequest::new(self.config.clone())
    }
}
