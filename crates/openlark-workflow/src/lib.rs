//! # OpenLark 工作流模块
//!
//! OpenLark SDK 的工作流模块，提供飞书任务、审批和看板 API 的完整访问。
//! Communication / Workflow helper 分层边界见 `docs/communication-workflow-helper-boundaries.md`。
//!
//! ## 功能特性
//!
//! - **任务管理**: 创建、更新、删除、查询待办事项
//! - **审批流程**: 审批定义、审批实例管理，以及高频审批任务 helper
//! - **看板管理**: 看板创建、任务卡片管理
//! - **协作支持**: 添加执行者、关注者、提醒
//! - **版本支持**: 支持 task v1/v2，以及 approval v4 helper 场景
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use openlark_workflow::{
//!     ApprovalTaskAction, ApprovalTaskQuery, WorkflowService, WorkflowTaskListQuery,
//!     WorkflowTaskMutation,
//! };
//! use openlark_core::config::Config;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = Config::builder()
//!     .app_id("your_app_id")
//!     .app_secret("your_app_secret")
//!     .build();
//!
//! let workflow_service = WorkflowService::new(config);
//!
//! // 列取任务清单中的任务
//! let tasks = workflow_service
//!     .list_tasks_all(WorkflowTaskListQuery::for_tasklist("tasklist_guid"))
//!     .await?;
//!
//! // 更新任务
//! let result = workflow_service
//!     .mutate_task(
//!         "task_guid",
//!         WorkflowTaskMutation::new()
//!             .summary("完成项目文档")
//!             .priority(3),
//!     )
//!     .await?;
//!
//! // 处理待审批任务
//! let approval_tasks = workflow_service
//!     .query_approval_tasks(
//!         ApprovalTaskQuery::new("ou_example_user", "1")
//!             .user_id_type("open_id")
//!             .status("Todo"),
//!     )
//!     .await?;
//! if let Some(task) = approval_tasks.first() {
//!     let _ = workflow_service
//!         .approve_task(
//!             ApprovalTaskAction::new(
//!                 task.approval_code.clone(),
//!                 task.instance_code.clone(),
//!                 "ou_example_user",
//!                 task.task_id.clone(),
//!             )
//!             .user_id_type("open_id")
//!             .comment("同意"),
//!         )
//!         .await?;
//! }
//! # Ok(())
//! # }
//! ```

mod service;

// 通用模块
/// 工作流通用工具、端点与模型。
pub mod common;

// 版本模块
#[cfg(feature = "v1")]
/// 任务 v1 API 模块。
pub mod v1;

#[cfg(feature = "v2")]
/// 任务 v2 API 模块。
pub mod v2;

// 看板模块
#[cfg(feature = "board")]
/// 白板/看板模块。
pub mod board;

// 审批模块（v4 审批定义/实例/任务、外部审批）
/// 审批 API 模块。
pub mod approval;

// Prelude 模块
/// 常用工作流类型预导出模块。
pub mod prelude;

// 重新导出核心服务
pub use service::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowService, WorkflowTaskListQuery,
    WorkflowTaskMutation,
};

// 重新导出 approval v4 用户级接口类型（用户态，需 user_access_token）
// 用户可直接 new() + builder + execute_with_options(option) 调用
pub use service::{
    AddCcInstanceBodyV4, AddCcInstanceRequestV4, AddCcInstanceResponseV4, AddSignTaskBodyV4,
    AddSignTaskRequestV4, AddSignTaskResponseV4, DetailInstanceRequestV4, DetailInstanceResponseV4,
    DetailInstanceTaskV4, ForwardTaskBodyV4, ForwardTaskRequestV4, ForwardTaskResponseV4,
    InitiatedInstanceItemV4, InitiatedInstanceRequestV4, InitiatedInstanceResponseV4,
    InstanceSummaryV4, ListTaskItemV4, ListTaskRequestV4, ListTaskResponseV4, PassTaskBodyV4,
    PassTaskRequestV4, PassTaskResponseV4, RecallInstanceBodyV4, RecallInstanceRequestV4,
    RecallInstanceResponseV4, RefuseTaskBodyV4, RefuseTaskRequestV4, RefuseTaskResponseV4,
    RemindInstanceBodyV4, RemindInstanceRequestV4, RemindInstanceResponseV4, RollbackTaskBodyV4,
    RollbackTaskRequestV4, RollbackTaskResponseV4, TaskSummaryV4,
};

/// 工作流服务客户端类型别名（统一命名为 `XxxClient`）。
pub type WorkflowClient = WorkflowService;

/// 工作流模块版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::VERSION;

    #[test]
    fn test_version() {
        assert_ne!(VERSION, "");
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;
    use openlark_core::config::Config;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_workflow_service_creation() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        // Service created successfully
        let _ = service;
    }

    #[test]
    fn test_workflow_service_clone() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        let _cloned = service.clone();
    }

    #[cfg(feature = "v1")]
    #[test]
    fn test_workflow_service_v1() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        let _v1 = service.v1();
    }

    #[cfg(feature = "v2")]
    #[test]
    fn test_workflow_service_v2() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        let _v2 = service.v2();
    }

    #[cfg(feature = "v2")]
    #[test]
    fn test_workflow_service_task() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        let _task = service.task();
    }

    #[cfg(feature = "v2")]
    #[test]
    fn test_workflow_service_tasklist() {
        let config = create_test_config();
        let service = WorkflowService::new(config);
        let _tasklist = service.tasklist();
    }
}
