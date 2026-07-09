/// 绩效模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// performance 项目模块。
/// performance 子模块。
#[allow(clippy::module_inception)]
pub mod performance;

/// 绩效服务
/// Performance 服务入口。
#[derive(Debug, Clone)]
pub struct Performance {
    config: Config,
}

impl Performance {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
