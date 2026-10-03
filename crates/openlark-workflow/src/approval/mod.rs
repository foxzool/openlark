//! 审批模块
//!
//! 提供审批相关 API 的版本化入口，包含审批定义、审批实例、审批任务及外部审批能力。
//!
//! ## 主要功能
//! - `v4`: 审批相关 v4 版本接口入口
//! - 审批流程：审批定义、实例与任务管理
//! - 外部审批：外部审批单据、实例与任务对接

pub mod approval;

// v4 模块显式导出
pub use approval::v4::{external_approval, external_instance, external_task, instance, task};
