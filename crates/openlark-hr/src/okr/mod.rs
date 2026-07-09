/// OKR 模块
///
/// 按照bizTag/project/version/resource/name.rs模式组织
use openlark_core::config::Config;

/// okr 项目模块。
/// okr 子模块。
#[allow(clippy::module_inception)]
pub mod okr;

/// OKR 服务
/// Okr 服务入口。
#[derive(Debug, Clone)]
pub struct Okr {
    config: Config,
}

impl Okr {
    /// 创建新的服务入口实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 返回共享配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取 okr 项目 v2 版本服务
    pub fn v2(&self) -> okr::OkrV2 {
        okr::OkrV2::new(self.config.clone())
    }
}
