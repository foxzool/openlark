//! Badge module

use openlark_core::config::Config;

pub mod create;
pub mod get;
pub mod grant;
pub mod list;
pub mod update;

/// badge 资源服务
#[derive(Debug, Clone)]
pub struct BadgeService {
    config: Config,
}

impl BadgeService {
    /// 创建新的 badge 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建勋章
    pub fn create(&self) -> create::CreateBadgeRequestBuilder {
        create::CreateBadgeRequestBuilder::new(self.config.clone())
    }

    /// 获取勋章详情
    pub fn get(&self) -> get::GetBadgeRequestBuilder {
        get::GetBadgeRequestBuilder::new(self.config.clone())
    }

    /// 勋章列表
    pub fn list(&self) -> list::ListBadgeRequestBuilder {
        list::ListBadgeRequestBuilder::new(self.config.clone())
    }

    /// 更新勋章
    pub fn update(&self) -> update::UpdateBadgeRequestBuilder {
        update::UpdateBadgeRequestBuilder::new(self.config.clone())
    }

    /// badge.grant 子资源
    pub fn grant(&self) -> grant::BadgeGrantService {
        grant::BadgeGrantService::new(self.config.clone())
    }
}
