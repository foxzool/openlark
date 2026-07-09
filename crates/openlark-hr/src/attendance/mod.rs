/// 考勤模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// attendance 项目模块。
/// attendance 子模块。
#[allow(clippy::module_inception)]
pub mod attendance;

/// 考勤服务
/// Attendance 服务入口。
#[derive(Debug, Clone)]
pub struct Attendance {
    config: Config,
}

impl Attendance {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}
