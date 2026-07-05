/// ability 模块。
pub mod ability;
/// base 模块。
pub mod base;
/// config 模块。
pub mod config;
/// publish 模块。
pub mod publish;

use openlark_core::config::Config;
use std::sync::Arc;

/// Application：v7 application 资源（ability / base / config / publish）。
#[derive(Clone)]
pub struct Application {
    config: Arc<Config>,
}

impl Application {
    /// 创建新的 Application 资源实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 更新应用能力。
    pub fn ability_patch(&self) -> ability::patch::ApplicationAbilityPatchRequest {
        ability::patch::ApplicationAbilityPatchRequest::new(self.config.clone())
    }

    /// 更新应用基础信息。
    pub fn base_patch(&self) -> base::patch::ApplicationBasePatchRequest {
        base::patch::ApplicationBasePatchRequest::new(self.config.clone())
    }

    /// 更新应用配置。
    pub fn config_patch(&self) -> config::patch::ApplicationConfigPatchRequest {
        config::patch::ApplicationConfigPatchRequest::new(self.config.clone())
    }

    /// 创建应用发布。
    pub fn publish_create(&self) -> publish::create::ApplicationPublishCreateRequest {
        publish::create::ApplicationPublishCreateRequest::new(self.config.clone())
    }
}
