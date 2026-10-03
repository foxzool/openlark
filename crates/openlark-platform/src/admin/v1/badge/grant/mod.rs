//! Badge Grant module

use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

/// badge.grant 资源服务
#[derive(Debug, Clone)]
pub struct BadgeGrantService {
    config: Config,
}

impl BadgeGrantService {
    /// 创建新的 badge.grant 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建勋章授予名单
    pub fn create(&self) -> create::CreateBadgeGrantRequestBuilder {
        create::CreateBadgeGrantRequestBuilder::new(self.config.clone())
    }

    /// 获取勋章授予名单详情
    pub fn get(&self) -> get::GetBadgeGrantRequestBuilder {
        get::GetBadgeGrantRequestBuilder::new(self.config.clone())
    }

    /// 勋章授予名单列表
    pub fn list(&self) -> list::ListBadgeGrantRequestBuilder {
        list::ListBadgeGrantRequestBuilder::new(self.config.clone())
    }

    /// 更新勋章授予名单
    pub fn update(&self) -> update::UpdateBadgeGrantRequestBuilder {
        update::UpdateBadgeGrantRequestBuilder::new(self.config.clone())
    }

    /// 删除勋章授予名单
    pub fn delete(&self) -> delete::DeleteBadgeGrantRequestBuilder {
        delete::DeleteBadgeGrantRequestBuilder::new(self.config.clone())
    }
}
