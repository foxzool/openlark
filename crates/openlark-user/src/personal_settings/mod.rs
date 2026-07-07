//! Personal Settings 模块
//!
//! `SystemStatusResource` 由 `UserService::system_status()` 直接返回（ADR 0001：砍
//! `PersonalSettingsResource` 单转发中间层），收敛 system_status 7 个真实请求构建器，
//! 避免写 `personal_settings::personal_settings::v1::system_status::*` 三重嵌套模块全路径。

pub mod personal_settings;

use openlark_core::config::Config;
use std::sync::Arc;

use crate::personal_settings::personal_settings::v1::system_status::{
    batch_close::BatchCloseSystemStatusRequest, batch_open::SystemStatusBatchOpenRequest,
    create::SystemStatusCreateRequest, delete::SystemStatusDeleteRequest,
    get::SystemStatusGetRequest, list::SystemStatusListRequest, patch::SystemStatusPatchRequest,
};

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
    ///
    /// ⚠️ **幻影 API**：飞书 `personal_settings/v1/system_status` 无 `get` 接口（目录里
    /// "获取系统状态"对应的是 [`list`](SystemStatusResource::list)），此方法请求的是一个不
    /// 存在的端点。将在 v0.18.0 移除（#377）；请改用 [`list`](SystemStatusResource::list)。
    #[deprecated(
        note = "幻影 API：飞书无 system_status get 接口，将在 v0.18.0 移除；请改用 list（SystemStatusListRequest）（#377）"
    )]
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
