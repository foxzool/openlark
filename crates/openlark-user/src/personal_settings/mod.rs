//! Personal Settings 模块
//!
//! 门面 [`PersonalSettingsResource`] 是 `UserService::personal_settings()` 返回的薄 resource，
//! 收敛 system_status 等真实请求构建器，缩短调用路径（避免写
//! `personal_settings::personal_settings::v1::system_status::*` 三重嵌套模块全路径）。

pub mod personal_settings;

use openlark_core::config::Config;
use std::sync::Arc;

use crate::personal_settings::personal_settings::v1::system_status::{
    batch_close::BatchCloseSystemStatusRequest, batch_open::SystemStatusBatchOpenRequest,
    create::SystemStatusCreateRequest, delete::SystemStatusDeleteRequest,
    get::SystemStatusGetRequest, list::SystemStatusListRequest, patch::SystemStatusPatchRequest,
};

/// 个人设置资源（门面 accessor 返回的薄 resource）。
///
/// 经 `UserService::personal_settings()` 获得，提供 system_status 资源入口。
#[derive(Debug, Clone)]
pub struct PersonalSettingsResource {
    /// 客户端配置
    config: Arc<Config>,
}

impl PersonalSettingsResource {
    /// 创建新的个人设置资源实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客户端配置。
    pub fn config(&self) -> Arc<Config> {
        self.config.clone()
    }

    /// system_status 资源入口（7 个真实请求构建器）。
    pub fn system_status(&self) -> SystemStatusResource {
        SystemStatusResource::new(self.config.clone())
    }
}

/// system_status 资源（7 个真实请求构建器）。
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

    /// 获取系统状态详情。
    pub fn get(&self) -> SystemStatusGetRequest {
        SystemStatusGetRequest::new(self.config.clone())
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
