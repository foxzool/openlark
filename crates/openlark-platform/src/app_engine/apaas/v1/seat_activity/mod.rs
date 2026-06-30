//! 席位活跃详情模块

pub mod list;

use openlark_core::config::Config;

/// seat_activity 资源服务
#[derive(Debug, Clone)]
pub struct SeatActivityService {
    config: Config,
}

impl SeatActivityService {
    /// 创建新的 seat_activity 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 席位活跃列表
    pub fn list(&self) -> list::SeatActivityListRequestBuilder {
        list::SeatActivityListRequestBuilder::new(self.config.clone())
    }
}
