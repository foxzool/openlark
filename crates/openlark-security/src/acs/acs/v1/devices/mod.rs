//! 设备管理 API（门面）
//!
//! [`DevicesService`] 是轻量门面，返回 `super::device::*` 下的端点构建器。
//! 客户端设备认证（`client_device`）是独立端点，由 [`AcsV1Service::client_device`]
//! 直接暴露。

use openlark_core::config::Config;

/// 设备管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct DevicesService {
    config: Config,
}

impl DevicesService {
    /// 创建新的设备管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取单个设备信息。
    pub fn get(&self, device_id: impl Into<String>) -> super::device::get::GetDeviceRequest {
        super::device::get::GetDeviceRequest::new(self.config.clone(), device_id)
    }

    /// 获取设备列表。
    pub fn list(&self) -> super::device::list::ListDevicesRequest {
        super::device::list::ListDevicesRequest::new(self.config.clone())
    }

    /// 新增设备。
    pub fn create(&self) -> super::device::create::CreateDeviceRequest {
        super::device::create::CreateDeviceRequest::new(self.config.clone())
    }

    /// 更新设备。
    pub fn update(
        &self,
        device_id: impl Into<String>,
    ) -> super::device::update::UpdateDeviceRequest {
        super::device::update::UpdateDeviceRequest::new(self.config.clone(), device_id)
    }

    /// 删除设备。
    pub fn delete(
        &self,
        device_id: impl Into<String>,
    ) -> super::device::delete::DeleteDeviceRequest {
        super::device::delete::DeleteDeviceRequest::new(self.config.clone(), device_id)
    }

    /// 审批设备申报。
    pub fn approve(
        &self,
        device_id: impl Into<String>,
    ) -> super::device::approve::ApproveDeviceRequest {
        super::device::approve::ApproveDeviceRequest::new(self.config.clone(), device_id)
    }

    /// 查询设备信息。
    pub fn query(&self, device_id: impl Into<String>) -> super::device::query::QueryDeviceRequest {
        super::device::query::QueryDeviceRequest::new(self.config.clone(), device_id)
    }
}
