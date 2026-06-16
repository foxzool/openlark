//! 安全服务预导出模块
//!
//! 提供最常用的类型和功能的便捷导入。

// 重新导出主要类型
pub use crate::{AcsProject, SecurityAndComplianceProject, SecurityError, SecurityResult, SecurityServices};

// 重新导出配置和基础类型
pub use crate::models::{
    ExtensionMap, GeoLocation, KeyValue, OperationResponse, PageRequest, PageResponse,
    SecurityConfig, SortCondition, SortDirection, Status, TimeRange, Timestamp,
};

// 重新导出 ACS 相关类型
pub use crate::models::acs::*;

// 重新导出 Security & Compliance 相关类型
pub use crate::models::security_and_compliance::*;

// 重新导出通用模型
pub use crate::models::common::{
    BatchOperationError, BatchOperationRequest, BatchOperationResponse, DeviceBase, FileUploadResponse,
    PermissionBase, QueryCondition, UserBase,
};

// 重新导出项目服务
pub use crate::acs::{AcsV1Service};
pub use crate::security::security_and_compliance::{SecurityAndComplianceV1Service, SecurityAndComplianceV2Service};