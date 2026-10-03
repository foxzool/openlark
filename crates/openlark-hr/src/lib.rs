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
//! ```no_run
//! use openlark_hr::HrClient;
//! use openlark_hr::attendance::attendance::v1::group::CreateGroupRequest;
//! # let config: openlark_core::config::Config = unimplemented!();
//! let client = HrClient::new(config);
//! // 真实资源直达：Config-direct Request 自带 builder + execute()
//! let _request = CreateGroupRequest::new(client.config().clone())
//!     .group_name("技术部".to_string());
//! // .execute().await?  // 发送请求（需 async runtime，示例省略）
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

/// 常用类型预导出模块。
pub mod prelude {
    pub use openlark_core::{SDKResult, config::Config};
}

use openlark_core::config::Config;
use std::sync::Arc;

/// HRClient：统一入口。
///
/// - `config()` 直达底层 `Config`，用于构造各域 leaf 请求
///   （attendance/corehr/payroll/performance/compensation/hire/ehr）。
/// - `okr` 保留 fluent v2 路由（`client.okr.v2()`）。
#[derive(Debug, Clone)]
pub struct HrClient {
    config: Arc<Config>,

    #[cfg(feature = "okr")]
    /// OKR 入口。
    pub okr: okr::Okr,
}

impl HrClient {
    /// 创建新的 HR 客户端。
    pub fn new(config: Config) -> Self {
        let config = Arc::new(config);
        Self {
            #[cfg(feature = "okr")]
            okr: okr::Okr::new((*config).clone()),
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

    #[cfg(feature = "okr")]
    #[test]
    fn test_hr_client_okr_field() {
        let config = create_test_config();
        let client = HrClient::new(config);
        assert_eq!(client.okr.config().app_id(), "test_app");
    }
}
