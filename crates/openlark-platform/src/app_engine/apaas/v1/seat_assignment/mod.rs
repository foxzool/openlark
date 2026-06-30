//! 席位分配详情模块

pub mod list;

use openlark_core::config::Config;

/// seat_assignment 资源服务
#[derive(Debug, Clone)]
pub struct SeatAssignmentService {
    config: Config,
}

impl SeatAssignmentService {
    /// 创建新的 seat_assignment 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 席位分配列表
    pub fn list(&self) -> list::SeatAssignmentListRequestBuilder {
        list::SeatAssignmentListRequestBuilder::new(self.config.clone())
    }
}
