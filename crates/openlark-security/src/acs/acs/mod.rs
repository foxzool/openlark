//! 访问控制系统 (ACS) - Project
//!
//! 智能门禁访问控制系统，提供用户、设备、权限和访客管理功能。
//!
//! **迁移说明（见 docs/superpowers/specs/2026-06-20-acs-transport-migration-design.md）**：
//! acs 正在从 SecurityConfig + 原始 reqwest 迁移到 openlark_core::Config + Transport。
//! Task 1 仅做边界：`AcsProject`/`AcsV1Service` 切到 `Arc<openlark_core::Config>`，6 个资源
//! Service 暂时下线（注释），在 Task 2-5 中逐个以门面形式恢复（持 `Arc<Config>`）。

use openlark_core::config::Config;
use std::sync::Arc;

/// ACS 项目服务
#[derive(Debug)]
pub struct AcsProject {
    config: Arc<Config>,
    v1: AcsV1Service,
}

impl AcsProject {
    /// 创建新的 ACS 项目实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            v1: AcsV1Service::new(config.clone()),
            config,
        }
    }

    /// 获取 v1 版本服务
    pub fn v1(&self) -> &AcsV1Service {
        &self.v1
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// ACS v1 版本服务
///
/// **迁移进行中**：6 个资源 Service（users/user_faces/rule_external/visitors/devices/
/// access_records）及其访问器方法在 Task 2-5 中逐个恢复。当前暂仅持有核心配置。
#[derive(Debug)]
pub struct AcsV1Service {
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl AcsV1Service {
    /// 创建新的 v1 服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

// v1 模块
pub mod v1;
