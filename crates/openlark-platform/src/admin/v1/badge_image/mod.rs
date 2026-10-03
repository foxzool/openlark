//! Badge image module

use openlark_core::config::Config;

pub mod create;

/// badge_image 资源服务
#[derive(Debug, Clone)]
pub struct BadgeImageService {
    config: Config,
}

impl BadgeImageService {
    /// 创建新的 badge_image 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传勋章图片
    pub fn create(&self) -> create::CreateBadgeImageRequestBuilder {
        create::CreateBadgeImageRequestBuilder::new(self.config.clone())
    }
}
