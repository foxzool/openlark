//! 安全合规管理 (Security & Compliance) - Project
//!
//! 提供设备记录管理、审计日志、合规检查等安全功能。
//!
//! **迁移说明（见 GitHub issue #210）**：security_and_compliance 已从
//! `SecurityConfig` + 原始 reqwest 迁移到 `openlark_core::Config` + Transport，
//! 与 acs 子模块一致。整个 crate 现在统一使用 core `Config`。

use openlark_core::config::Config;

/// 安全合规项目服务
#[derive(Debug)]
pub struct SecurityAndComplianceProject {
    config: Config,
    v1: SecurityAndComplianceV1Service,
    v2: SecurityAndComplianceV2Service,
}

impl SecurityAndComplianceProject {
    /// 创建新的安全合规项目实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: SecurityAndComplianceV1Service::new(config.clone()),
            v2: SecurityAndComplianceV2Service::new(config.clone()),
            config,
        }
    }

    /// 获取 v1 版本服务（审计日志）
    pub fn v1(&self) -> &SecurityAndComplianceV1Service {
        &self.v1
    }

    /// 获取 v2 版本服务（设备记录管理）
    pub fn v2(&self) -> &SecurityAndComplianceV2Service {
        &self.v2
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 安全合规 v1 版本服务
///
/// 主要提供审计日志功能
#[derive(Debug)]
pub struct SecurityAndComplianceV1Service {
    openapi_logs: crate::security::security_and_compliance::v1::openapi_logs::OpenApiLogsService,
}

impl SecurityAndComplianceV1Service {
    /// 创建新的 v1 服务实例
    pub fn new(config: Config) -> Self {
        Self {
            openapi_logs:
                crate::security::security_and_compliance::v1::openapi_logs::OpenApiLogsService::new(
                    config,
                ),
        }
    }

    /// 获取 OpenAPI 审计日志服务
    pub fn openapi_logs(
        &self,
    ) -> &crate::security::security_and_compliance::v1::openapi_logs::OpenApiLogsService {
        &self.openapi_logs
    }
}

/// 安全合规 v2 版本服务
///
/// 主要提供设备记录管理和设备申报审批功能
#[derive(Debug)]
pub struct SecurityAndComplianceV2Service {
    device_records: crate::security::security_and_compliance::v2::device_records::DeviceRecordsService,
    device_apply_records:
        crate::security::security_and_compliance::v2::device_apply_records::DeviceApplyRecordsService,
}

impl SecurityAndComplianceV2Service {
    /// 创建新的 v2 服务实例
    pub fn new(config: Config) -> Self {
        Self {
            device_records: crate::security::security_and_compliance::v2::device_records::DeviceRecordsService::new(config.clone()),
            device_apply_records: crate::security::security_and_compliance::v2::device_apply_records::DeviceApplyRecordsService::new(config),
        }
    }

    /// 获取设备记录管理服务
    pub fn device_records(
        &self,
    ) -> &crate::security::security_and_compliance::v2::device_records::DeviceRecordsService {
        &self.device_records
    }

    /// 获取设备申报审批服务
    pub fn device_apply_records(
        &self,
    ) -> &crate::security::security_and_compliance::v2::device_apply_records::DeviceApplyRecordsService{
        &self.device_apply_records
    }
}

// v1 模块
pub mod v1;

// v2 模块
pub mod v2;
