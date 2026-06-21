//! 设备申报审批 API（门面）
//!
//! [`DeviceApplyRecordsService`] 是轻量门面，返回 [`approve`] 下的端点构建器。

pub mod approve;

use openlark_core::config::Config;

/// 设备申报审批服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct DeviceApplyRecordsService {
    config: Config,
}

impl DeviceApplyRecordsService {
    /// 创建新的设备申报审批服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 审批设备申报（PUT `/device_apply_records/{device_apply_record_id}`）。
    pub fn approve(
        &self,
        device_apply_record_id: impl Into<String>,
    ) -> approve::ApproveDeviceApplyRecordRequest {
        approve::ApproveDeviceApplyRecordRequest::new(self.config.clone(), device_apply_record_id)
    }
}
