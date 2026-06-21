//! 访客管理 API（门面）
//!
//! [`VisitorsService`] 是轻量门面，返回 `super::visitor::*` 下的端点构建器。

use openlark_core::config::Config;

/// 访客管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct VisitorsService {
    config: Config,
}

impl VisitorsService {
    /// 创建新的访客管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加访客。
    pub fn create(&self) -> super::visitor::create::CreateVisitorRequest {
        super::visitor::create::CreateVisitorRequest::new(self.config.clone())
    }

    /// 删除访客。
    pub fn delete(
        &self,
        visitor_id: impl Into<String>,
    ) -> super::visitor::delete::DeleteVisitorRequest {
        super::visitor::delete::DeleteVisitorRequest::new(self.config.clone(), visitor_id)
    }
}
