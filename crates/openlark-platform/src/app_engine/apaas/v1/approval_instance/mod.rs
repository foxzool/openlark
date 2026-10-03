//! Approval Instance module

pub mod cancel;
pub mod get;
pub mod list;

use openlark_core::config::Config;

/// approval_instance 资源服务
#[derive(Debug, Clone)]
pub struct ApprovalInstanceService {
    config: Config,
}

impl ApprovalInstanceService {
    /// 创建新的 approval_instance 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 审批实例列表
    pub fn list(&self) -> list::ListInstanceRequestBuilder {
        list::ListInstanceRequestBuilder::new(self.config.clone())
    }

    /// 取消审批实例
    pub fn cancel(&self) -> cancel::CancelInstanceRequestBuilder {
        cancel::CancelInstanceRequestBuilder::new(self.config.clone())
    }

    /// 获取审批实例详情
    pub fn get(&self) -> get::GetInstanceRequestBuilder {
        get::GetInstanceRequestBuilder::new(self.config.clone())
    }
}
