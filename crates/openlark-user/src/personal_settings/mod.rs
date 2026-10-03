//! Personal Settings 模块
//!
//! `SystemStatusResource` 由 `UserService::system_status()` 直接返回（ADR 0001：砍
//! `PersonalSettingsResource` 单转发中间层），收敛 system_status 6 个真实请求构建器，
//! 避免写 `personal_settings::personal_settings::v1::system_status::*` 三重嵌套模块全路径。

pub mod personal_settings;

use openlark_core::config::Config;
use std::sync::Arc;

use crate::personal_settings::personal_settings::v1::system_status::{
    batch_close::BatchCloseSystemStatusRequest, batch_open::SystemStatusBatchOpenRequest,
    create::SystemStatusCreateRequest, delete::SystemStatusDeleteRequest,
    list::SystemStatusListRequest, patch::SystemStatusPatchRequest,
};

/// system_status 资源（6 个真实请求构建器）。
#[derive(Debug, Clone)]
pub struct SystemStatusResource {
    config: Arc<Config>,
}

impl SystemStatusResource {
    /// 创建新的 system_status 资源实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客户端配置。
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    /// 获取系统状态列表。
    pub fn list(&self) -> SystemStatusListRequest {
        SystemStatusListRequest::new(self.config.clone())
    }

    /// 创建系统状态。
    pub fn create(&self) -> SystemStatusCreateRequest {
        SystemStatusCreateRequest::new(self.config.clone())
    }

    /// 更新系统状态。
    pub fn patch(&self) -> SystemStatusPatchRequest {
        SystemStatusPatchRequest::new(self.config.clone())
    }

    /// 删除系统状态。
    pub fn delete(&self) -> SystemStatusDeleteRequest {
        SystemStatusDeleteRequest::new(self.config.clone())
    }

    /// 批量开启系统状态。
    ///
    /// `status_id` 为目标系统状态 ID（飞书 API 限定单次操作一个状态）。
    pub fn batch_open(&self, status_id: impl Into<String>) -> SystemStatusBatchOpenRequest {
        SystemStatusBatchOpenRequest::new(self.config.clone(), status_id)
    }

    /// 批量关闭系统状态。
    ///
    /// `status_id` 为目标系统状态 ID（飞书 API 限定单次操作一个状态）。
    pub fn batch_close(&self, status_id: impl Into<String>) -> BatchCloseSystemStatusRequest {
        BatchCloseSystemStatusRequest::new(self.config.clone(), status_id)
    }
}
