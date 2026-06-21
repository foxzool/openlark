//! OpenLark 安全服务模块
//!
//! 提供飞书开放平台的完整安全服务，包括访问控制(ACS)和安全合规管理。
//!
//! ## 架构设计
//!
//! 采用 Project-Version-Resource (PVR) 三层架构：
//!
//! ```text
//! openlark-security/src/
//! ├── models/           # 共享数据模型
//! ├── acs/              # 访问控制系统 (Project)
//! │   └── v1/          # API版本v1 (Version)
//! └── security_and_compliance/  # 安全合规管理 (Project)
//!     ├── v1/          # API版本v1 (Version) - 审计日志
//!     └── v2/          # API版本v2 (Version) - 设备记录管理
//! ```
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! // 注：迁移期间（acs Service 门面化进行中）此示例暂不编译，Task 8 恢复。
//! use openlark_security::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = SecurityConfig::new("app_id", "app_secret");
//!     let security = SecurityServices::new(config);
//!
//!     // 获取门禁用户列表
//!     let users = security.acs.v1().users().list()
//!         .page_size(20)
//!         .send()
//!         .await?;
//!
//!     println!("用户数量: {}", users.users.len());
//!     Ok(())
//! }
//! ```
//!
//! ## API覆盖
//!
//! ### acs (v1) - 访问控制系统
//! #### 用户管理 (user)
//! - `users.get()` - 获取单个用户信息
//! - `users.list()` - 获取用户列表
//! - `users.patch()` - 修改用户部分信息
//!
//! #### 人脸识别 (user.face)
//! - `user_faces.get()` - 下载人脸图片
//! - `user_faces.update()` - 上传人脸图片
//!
//! #### 权限规则 (rule_external)
//! - `rule_external.create()` - 创建或更新权限组
//! - `rule_external.get()` - 获取权限组信息
//! - `rule_external.delete()` - 删除权限组
//! - `rule_external.device_bind()` - 设备绑定权限组
//!
//! #### 访客管理 (visitor)
//! - `visitors.create()` - 添加访客
//! - `visitors.delete()` - 删除访客
//!
//! #### 设备管理 (device)
//! - `devices.list()` - 获取门禁设备列表
//!
//! ### security_and_compliance (v2/v1) - 安全合规管理
//! #### 设备记录管理 (device_record - v2)
//! - `device_records.mine()` - 获取客户端设备认证信息
//! - `device_records.create()` - 新增设备
//! - `device_records.list()` - 查询设备信息
//! - `device_records.get()` - 获取设备信息
//! - `device_records.update()` - 更新设备
//! - `device_records.delete()` - 删除设备
//!
//! #### 设备申报审批 (device_apply_record - v2)
//! - `device_apply_records.approve()` - 审批设备申报
//!
//! #### 审计日志管理 (openapi_log - v1)
//! - `openapi_logs.list_data()` - 获取OpenAPI审计日志数据

#![deny(missing_docs)]
#![warn(clippy::all)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// 错误处理模块
pub mod error;

// 共享数据模型
pub mod models;

// Project: acs - 访问控制系统
pub mod acs;
pub mod security;

// 重新导出主要类型
pub use acs::acs::{AcsProject, AcsV1Service};
pub use security::security_and_compliance::{
    SecurityAndComplianceProject, SecurityAndComplianceV1Service, SecurityAndComplianceV2Service,
};

// 重新导出错误类型
pub use crate::error::SecurityError;

/// 安全服务统一入口
#[derive(Debug)]
pub struct SecurityServices {
    /// 安全配置
    pub config: std::sync::Arc<crate::models::SecurityConfig>,
    /// ACS门禁控制项目
    pub acs: AcsProject,
    /// 安全合规项目
    pub security_and_compliance: SecurityAndComplianceProject,
}

impl SecurityServices {
    /// 创建新的安全服务实例
    ///
    /// 内部为 acs 构造一份 `openlark_core::Config`（由 SecurityConfig 字段转换而来），
    /// 供 acs 走 SDK 标准的 Transport 路径；security_and_compliance 仍直接吃 SecurityConfig。
    pub fn new(config: crate::models::SecurityConfig) -> Self {
        let config = std::sync::Arc::new(config);

        // Approach A 边界转换：SecurityConfig → openlark_core::Config
        // （一次性 shim，待 security_and_compliance 也迁移到 Transport 后整体删除）
        let acs_core_config = std::sync::Arc::new(
            openlark_core::config::Config::builder()
                .app_id(&config.app_id)
                .app_secret(&config.app_secret)
                .base_url(&config.base_url)
                .build(),
        );

        Self {
            acs: AcsProject::new(acs_core_config),
            security_and_compliance: SecurityAndComplianceProject::new(config.clone()),
            config,
        }
    }

    /// 获取配置信息
    pub fn config(&self) -> &crate::models::SecurityConfig {
        &self.config
    }
}

/// 安全服务客户端 — Arc 包装的 [`SecurityServices`]，支持零成本克隆。
///
/// 用法：`client.security.acs...`
#[derive(Debug, Clone)]
pub struct SecurityClient {
    inner: std::sync::Arc<SecurityServices>,
}

impl SecurityClient {
    /// 从安全配置创建客户端实例。
    pub fn new(config: crate::models::SecurityConfig) -> Self {
        Self {
            inner: std::sync::Arc::new(SecurityServices::new(config)),
        }
    }
}

impl std::ops::Deref for SecurityClient {
    type Target = SecurityServices;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for SecurityServices {
    fn default() -> Self {
        Self::new(crate::models::SecurityConfig::default())
    }
}

/// 结果类型别名
pub type SecurityResult<T> = Result<T, crate::error::SecurityError>;

/// 预导出模块
pub mod prelude {
    pub use super::{
        AcsProject, SecurityAndComplianceProject, SecurityClient, SecurityResult, SecurityServices,
    };

    // 避免v1命名空间冲突，明确导出需要的类型
    pub use super::acs::acs::{AcsProject as Acs, AcsV1Service};
    pub use super::models::*;
    pub use super::security::security_and_compliance::{
        SecurityAndComplianceV1Service, SecurityAndComplianceV2Service,
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
