//! 设备记录管理 API（门面）
//!
//! [`DeviceRecordsService`] 是轻量门面，返回 `mine|create|list|get|update|delete` 子模块
//! 下的端点构建器。`mine` 为通过用户授权获取当前设备信息。

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod mine;
pub mod update;

use openlark_core::config::Config;

/// 设备记录管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct DeviceRecordsService {
    config: Config,
}

impl DeviceRecordsService {
    /// 创建新的设备记录管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 通过授权获取设备信息（GET `/device_records/mine`）。
    pub fn mine(&self) -> mine::MineDeviceRecordRequest {
        mine::MineDeviceRecordRequest::new(self.config.clone())
    }

    /// 新增设备记录（POST `/device_records`）。
    pub fn create(&self) -> create::CreateDeviceRecordRequest {
        create::CreateDeviceRecordRequest::new(self.config.clone())
    }

    /// 查询设备信息列表（GET `/device_records`）。
    pub fn list(&self) -> list::ListDeviceRecordsRequest {
        list::ListDeviceRecordsRequest::new(self.config.clone())
    }

    /// 获取单个设备信息（GET `/device_records/{device_record_id}`）。
    pub fn get(&self, device_record_id: impl Into<String>) -> get::GetDeviceRecordRequest {
        get::GetDeviceRecordRequest::new(self.config.clone(), device_record_id)
    }

    /// 更新设备信息（PUT `/device_records/{device_record_id}`，查询参数 `version` 必填）。
    pub fn update(
        &self,
        device_record_id: impl Into<String>,
        version: impl Into<String>,
    ) -> update::UpdateDeviceRecordRequest {
        update::UpdateDeviceRecordRequest::new(self.config.clone(), device_record_id, version)
    }

    /// 删除设备（DELETE `/device_records/{device_record_id}`）。
    pub fn delete(&self, device_record_id: impl Into<String>) -> delete::DeleteDeviceRecordRequest {
        delete::DeleteDeviceRecordRequest::new(self.config.clone(), device_record_id)
    }
}
