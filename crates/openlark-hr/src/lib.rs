//! Open-Lark HR Module
//!
//! 飞书人力资源服务模块，提供完整的人力资源管理功能。
//!
//! ## 主要功能
//!
//! - **考勤管理**: 考勤组、班次、用户任务、统计数据、请假加班管理
//! - **核心人力资源**: 员工、岗位、部门、合同、工作地点管理
//! - **OKR管理**: 目标与关键成果的制定、跟踪和评估
//! - **薪资管理**: 薪资组、薪资调整、工资单、薪资条目管理
//! - **绩效管理**: 绩效周期、评估、反馈、目标管理
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use openlark_hr::prelude::*;
//! use openlark_hr::HrClient;
//!
//! let client = HrClient::new(config);
//! // 推荐：字段式 meta 入口
//! client.attendance.v1().group().create()
//!     .group_name("技术部".to_string())
//!     .execute()
//!     .await?;
//!
//! // 字段访问是当前推荐方式；旧方法式入口只保留兼容职责。
//! ```
//!
//! ## API 端点
//!
//! 推荐使用枚举类型的端点系统（位于 `common::api_endpoints`）：
//! - `AttendanceApiV1` - 考勤管理
//! - `HireApiV1` - 招聘管理
//! - `FeishuPeopleApiV1` / `FeishuPeopleApiV2` - 核心人力资源
//! - `OkrApiV1` - OKR管理
//! - `PayrollApiV1` - 薪资管理
//! - `PerformanceApiV1` - 绩效管理

/// 通用宏、端点与共享模型。
pub mod common;

#[cfg(feature = "attendance")]
/// 考勤模块。
pub mod attendance;
#[cfg(feature = "compensation")]
/// 薪酬管理模块。
pub mod compensation_management;
#[cfg(feature = "ehr")]
/// 员工档案模块。
pub mod ehr;
#[cfg(feature = "corehr")]
/// 核心人力模块。
pub mod feishu_people;
#[cfg(feature = "hire")]
/// 招聘模块。
pub mod hire;
#[cfg(feature = "okr")]
/// OKR 模块。
pub mod okr;
#[cfg(feature = "payroll")]
/// 薪资模块。
pub mod payroll;
#[cfg(feature = "performance")]
/// 绩效模块。
pub mod performance;

// 端点保留（已废弃，请使用 common::api_endpoints 中的枚举系统）
#[allow(deprecated)]
mod endpoints;

/// 常用类型预导出模块。
pub mod prelude {
    pub use openlark_core::{SDKResult, config::Config};
}

use openlark_core::config::Config;
use std::sync::Arc;

/// HRClient：统一入口，提供 project-version-resource 链式访问
#[derive(Debug, Clone)]
pub struct HrClient {
    config: Arc<Config>,

    #[cfg(feature = "attendance")]
    /// 考勤入口。
    pub attendance: attendance::Attendance,

    #[cfg(feature = "corehr")]
    /// 核心人力入口。
    pub corehr: feishu_people::Corehr,

    #[cfg(feature = "compensation")]
    /// 薪酬管理入口。
    pub compensation: compensation_management::CompensationManagement,

    #[cfg(feature = "payroll")]
    /// 薪资入口。
    pub payroll: payroll::Payroll,

    #[cfg(feature = "performance")]
    /// 绩效入口。
    pub performance: performance::Performance,

    #[cfg(feature = "okr")]
    /// OKR 入口。
    pub okr: okr::Okr,

    #[cfg(feature = "hire")]
    /// 招聘入口。
    pub hire: hire::Hire,

    #[cfg(feature = "ehr")]
    /// 员工档案入口。
    pub ehr: ehr::Ehr,
}

impl HrClient {
    /// 创建新的 HR 客户端。
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            #[cfg(feature = "attendance")]
            attendance: attendance::Attendance::new((*config).clone()),
            #[cfg(feature = "corehr")]
            corehr: feishu_people::Corehr::new((*config).clone()),
            #[cfg(feature = "compensation")]
            compensation: compensation_management::CompensationManagement::new((*config).clone()),
            #[cfg(feature = "payroll")]
            payroll: payroll::Payroll::new((*config).clone()),
            #[cfg(feature = "performance")]
            performance: performance::Performance::new((*config).clone()),
            #[cfg(feature = "okr")]
            okr: okr::Okr::new((*config).clone()),
            #[cfg(feature = "hire")]
            hire: hire::Hire::new((*config).clone()),
            #[cfg(feature = "ehr")]
            ehr: ehr::Ehr::new((*config).clone()),
            config,
        }
    }

    /// 返回底层配置引用。
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_hr_client_creation() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert!(client.config().app_id() == "test_app");
    }

    #[test]
    fn test_hr_client_clone() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let cloned = client.clone();
        assert!(cloned.config().app_id() == "test_app");
    }

    #[cfg(feature = "attendance")]
    #[test]
    fn test_hr_client_attendance_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.attendance.config().app_id(), "test_app");
    }

    #[cfg(feature = "attendance")]
    #[test]
    #[allow(deprecated)]
    fn test_hr_client_attendance_method() {
        let config = create_test_config();
        let client = HrClient::new(config);
        let attendance = client.attendance.clone();
        assert_eq!(attendance.config().app_id(), "test_app");
    }

    #[cfg(feature = "corehr")]
    #[test]
    fn test_hr_client_corehr_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.corehr.config().app_id(), "test_app");
    }

    #[cfg(feature = "compensation")]
    #[test]
    fn test_hr_client_compensation_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.compensation.config().app_id(), "test_app");
    }

    #[cfg(feature = "payroll")]
    #[test]
    fn test_hr_client_payroll_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.payroll.config().app_id(), "test_app");
    }

    #[cfg(feature = "performance")]
    #[test]
    fn test_hr_client_performance_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.performance.config().app_id(), "test_app");
    }

    #[cfg(feature = "okr")]
    #[test]
    fn test_hr_client_okr_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.okr.config().app_id(), "test_app");
    }

    #[cfg(feature = "hire")]
    #[test]
    fn test_hr_client_hire_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.hire.config().app_id(), "test_app");
    }

    #[cfg(feature = "ehr")]
    #[test]
    fn test_hr_client_ehr_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.ehr.config().app_id(), "test_app");
    }
}
