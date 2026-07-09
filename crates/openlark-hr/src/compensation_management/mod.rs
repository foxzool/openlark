/// 薪酬管理模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// compensation 子模块。
pub mod compensation;

/// 薪酬管理服务
/// CompensationManagement 服务入口。
#[derive(Debug, Clone)]
pub struct CompensationManagement {
    config: Config,
}

impl CompensationManagement {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
