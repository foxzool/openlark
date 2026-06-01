//! 任务 v2 资源模块
//!
//! 提供任务 v2 版本的资源操作集合，覆盖任务生命周期、成员与依赖管理、提醒管理与任务清单关联。
//!
//! ## 主要功能
//! - 任务基础操作：创建、查询、更新、删除、完成与取消完成
//! - 关系管理：成员、依赖、子任务与任务清单关联
//! - 扩展能力：提醒与响应模型统一导出

/// 添加依赖接口。
pub mod add_dependencies;
/// 添加成员接口。
pub mod add_members;
/// 添加提醒接口。
pub mod add_reminders;
/// 加入任务清单接口。
pub mod add_tasklist;
/// 完成任务接口。
pub mod complete;
/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 更新接口。
pub mod patch;
/// 移除依赖接口。
pub mod remove_dependencies;
/// 移除成员接口。
pub mod remove_members;
/// 移除提醒接口。
pub mod remove_reminders;
/// 移出任务清单接口。
pub mod remove_tasklist;
/// 设置父任务接口。
#[path = "../../task/task/v2/task/set_ancestor_task.rs"]
pub mod set_ancestor_task;
/// 子任务模块。
pub mod subtask;
/// tasklists 模块。
pub mod tasklists;
/// 取消完成接口。
pub mod uncomplete;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Task：任务资源（v2）
#[derive(Clone)]
pub struct Task {
    config: Arc<Config>,
}

impl Task {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateTaskRequest {
        create::CreateTaskRequest::new(self.config.clone())
    }

    /// 创建更新请求。
    pub fn update(&self, task_guid: impl Into<String>) -> update::UpdateTaskRequest {
        update::UpdateTaskRequest::new(self.config.clone(), task_guid.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, task_guid: impl Into<String>) -> delete::DeleteTaskRequest {
        delete::DeleteTaskRequest::new(self.config.clone(), task_guid.into())
    }

    /// 创建获取详情请求。
    pub fn get(&self, task_guid: impl Into<String>) -> get::GetTaskRequest {
        get::GetTaskRequest::new(self.config.clone(), task_guid.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListTasksRequest {
        list::ListTasksRequest::new(self.config.clone())
    }

    /// 创建完成任务请求。
    pub fn complete(&self, task_guid: impl Into<String>) -> complete::CompleteTaskRequest {
        complete::CompleteTaskRequest::new(self.config.clone(), task_guid.into())
    }

    /// 创建取消完成请求。
    pub fn uncomplete(&self, task_guid: impl Into<String>) -> uncomplete::UncompleteTaskRequest {
        uncomplete::UncompleteTaskRequest::new(self.config.clone(), task_guid.into())
    }

    /// 获取子任务资源
    pub fn subtask(&self, task_guid: impl Into<String>) -> subtask::Subtask {
        subtask::Subtask::new(self.config.clone(), task_guid.into())
    }

    /// 任务加入清单
    pub fn add_tasklist(&self, task_guid: impl Into<String>) -> add_tasklist::AddTasklistRequest {
        add_tasklist::AddTasklistRequest::new(self.config.clone(), task_guid.into())
    }

    /// 任务移出清单
    pub fn remove_tasklist(
        &self,
        task_guid: impl Into<String>,
    ) -> remove_tasklist::RemoveTasklistRequest {
        remove_tasklist::RemoveTasklistRequest::new(self.config.clone(), task_guid.into())
    }

    /// 列取任务所在清单
    pub fn tasklists(&self, task_guid: impl Into<String>) -> tasklists::GetTaskTasklistsRequest {
        tasklists::GetTaskTasklistsRequest::new(self.config.clone(), task_guid.into())
    }

    /// 添加任务成员
    pub fn add_members(&self, task_guid: impl Into<String>) -> add_members::AddMembersRequest {
        add_members::AddMembersRequest::new(self.config.clone(), task_guid.into())
    }

    /// 移除任务成员
    pub fn remove_members(
        &self,
        task_guid: impl Into<String>,
    ) -> remove_members::RemoveMembersRequest {
        remove_members::RemoveMembersRequest::new(self.config.clone(), task_guid.into())
    }

    /// 添加任务提醒
    pub fn add_reminders(
        &self,
        task_guid: impl Into<String>,
    ) -> add_reminders::AddRemindersRequest {
        add_reminders::AddRemindersRequest::new(self.config.clone(), task_guid.into())
    }

    /// 移除任务提醒
    pub fn remove_reminders(
        &self,
        task_guid: impl Into<String>,
    ) -> remove_reminders::RemoveRemindersRequest {
        remove_reminders::RemoveRemindersRequest::new(self.config.clone(), task_guid.into())
    }

    /// 添加任务依赖
    pub fn add_dependencies(
        &self,
        task_guid: impl Into<String>,
    ) -> add_dependencies::AddDependenciesRequest {
        add_dependencies::AddDependenciesRequest::new(self.config.clone(), task_guid.into())
    }

    /// 移除任务依赖
    pub fn remove_dependencies(
        &self,
        task_guid: impl Into<String>,
    ) -> remove_dependencies::RemoveDependenciesRequest {
        remove_dependencies::RemoveDependenciesRequest::new(self.config.clone(), task_guid.into())
    }

    /// 设置父任务。
    pub fn set_ancestor_task(
        &self,
        task_guid: impl Into<String>,
    ) -> set_ancestor_task::SetAncestorTaskRequest {
        set_ancestor_task::SetAncestorTaskRequest::new(self.config.clone(), task_guid.into())
    }
}

// 重新导出请求类型
pub use add_dependencies::AddDependenciesRequest;
pub use add_members::AddMembersRequest;
pub use add_reminders::AddRemindersRequest;
pub use add_tasklist::AddTasklistRequest;
pub use complete::CompleteTaskRequest;
pub use create::CreateTaskRequest;
pub use delete::DeleteTaskRequest;
pub use get::GetTaskRequest;
pub use list::ListTasksRequest;
pub use patch::UpdateTaskRequest;
pub use remove_dependencies::RemoveDependenciesRequest;
pub use remove_members::RemoveMembersRequest;
pub use remove_reminders::RemoveRemindersRequest;
pub use remove_tasklist::RemoveTasklistRequest;
pub use set_ancestor_task::SetAncestorTaskRequest;
pub use tasklists::GetTaskTasklistsRequest;
pub use uncomplete::UncompleteTaskRequest;

// 重新导出响应类型
pub use add_dependencies::{AddDependenciesResponse, TaskDependency};
pub use add_members::{AddMembersResponse, TaskMember};
pub use add_reminders::{AddRemindersResponse, TaskReminder};
pub use add_tasklist::{AddTasklistBody, AddTasklistResponse};
pub use models::{
    CompleteTaskResponse, CreateTaskBody, CreateTaskResponse, DeleteTaskResponse, GetTaskResponse,
    ListTasksResponse, TaskItem, UncompleteTaskResponse, UpdateTaskBody, UpdateTaskResponse,
};
pub use remove_dependencies::{RemoveDependenciesBody, RemoveDependenciesResponse};
pub use remove_members::{RemoveMembersBody, RemoveMembersResponse};
pub use remove_reminders::{RemoveRemindersBody, RemoveRemindersResponse};
pub use remove_tasklist::{RemoveTasklistBody, RemoveTasklistResponse};
pub use set_ancestor_task::{SetAncestorTaskBody, SetAncestorTaskResponse};
pub use tasklists::{GetTaskTasklistsResponse, TaskTasklistItem};

#[cfg(test)]
#[allow(unused_variables)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_task_v2_new() {
        let config = create_test_config();
        let _ = Task::new(config);
    }

    #[test]
    fn test_task_v2_create() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.create();
    }

    #[test]
    fn test_task_v2_update() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.update("task_guid_123");
    }

    #[test]
    fn test_task_v2_delete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.delete("task_guid_123");
    }

    #[test]
    fn test_task_v2_get() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.get("task_guid_123");
    }

    #[test]
    fn test_task_v2_list() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.list();
    }

    #[test]
    fn test_task_v2_complete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.complete("task_guid_123");
    }

    #[test]
    fn test_task_v2_uncomplete() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.uncomplete("task_guid_123");
    }

    #[test]
    fn test_task_v2_subtask() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.subtask("task_guid_123");
    }

    #[test]
    fn test_task_v2_add_tasklist() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.add_tasklist("task_guid_123");
    }

    #[test]
    fn test_task_v2_remove_tasklist() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.remove_tasklist("task_guid_123");
    }

    #[test]
    fn test_task_v2_tasklists() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.tasklists("task_guid_123");
    }

    #[test]
    fn test_task_v2_add_members() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.add_members("task_guid_123");
    }

    #[test]
    fn test_task_v2_remove_members() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.remove_members("task_guid_123");
    }

    #[test]
    fn test_task_v2_add_reminders() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.add_reminders("task_guid_123");
    }

    #[test]
    fn test_task_v2_remove_reminders() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.remove_reminders("task_guid_123");
    }

    #[test]
    fn test_task_v2_add_dependencies() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.add_dependencies("task_guid_123");
    }

    #[test]
    fn test_task_v2_remove_dependencies() {
        let config = create_test_config();
        let task = Task::new(config);
        let _ = task.remove_dependencies("task_guid_123");
    }
}
