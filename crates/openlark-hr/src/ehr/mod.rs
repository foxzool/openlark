/// EHR 模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// ehr 项目模块。
/// ehr 子模块。
#[allow(clippy::module_inception)]
pub mod ehr;

/// EHR 服务
/// Ehr 服务入口。
#[derive(Debug, Clone)]
pub struct Ehr {
    config: Config,
}

impl Ehr {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
