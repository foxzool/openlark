/// 薪资模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// payroll 项目模块。
/// payroll 子模块。
#[allow(clippy::module_inception)]
pub mod payroll;

/// 薪资服务
/// Payroll 服务入口。
#[derive(Debug, Clone)]
pub struct Payroll {
    config: Config,
}

impl Payroll {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
