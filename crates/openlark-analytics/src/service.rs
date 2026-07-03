//! 数据分析服务
//!
//! 提供数据分析相关的服务入口

use crate::AnalyticsConfig;
use openlark_core::SDKResult;
use std::sync::Arc;

/// 数据分析服务
///
/// 提供搜索、数据分析等功能的统一入口。
#[derive(Debug, Clone)]
pub struct AnalyticsService {
    /// 客户端配置
    config: Arc<AnalyticsConfig>,
}

impl AnalyticsService {
    /// 创建新的数据分析服务实例
    ///
    /// # 参数
    ///
    /// * `config` - 数据分析服务配置
    ///
    /// # 返回
    ///
    /// 返回数据分析服务实例或错误
    pub fn new(config: AnalyticsConfig) -> SDKResult<Self> {
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// 获取客户端配置
    pub fn config(&self) -> Arc<AnalyticsConfig> {
        self.config.clone()
    }

    /// 搜索服务
    ///
    /// 提供全文搜索、智能搜索等功能。
    #[deprecated(
        note = "search() 导航死胡同；直接用 crate::search::search::v2 子模块的 XxxRequest::new(config)，如 v2::query::SearchRequest / v2::user::SearchUserRequest"
    )]
    #[cfg(feature = "search")]
    #[allow(deprecated)]
    pub fn search(&self) -> crate::search::search::Search {
        crate::search::search::Search::new(self.config.clone())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = AnalyticsConfig::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();

        let service = AnalyticsService::new(config);
        assert!(service.is_ok());
    }
}
