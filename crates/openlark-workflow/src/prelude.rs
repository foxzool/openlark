//! 工作流模块预导入 - 包含任务与审批 helper 的常用类型和配置
//!
//! 这个模块重新导出了使用 workflow / approval helper 时最常需要的类型。

pub use crate::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowService, WorkflowTaskCreate,
    WorkflowTaskListQuery, WorkflowTaskMutation,
};
pub use openlark_core::{SDKResult, config::Config};
