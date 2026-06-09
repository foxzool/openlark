/// 动态订阅模块。
pub mod activity_subscription;
/// 添加成员接口。
pub mod add_members;
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
/// 移除成员接口。
pub mod remove_members;
#[path = "../../task/task/v2/tasklist/search.rs"]
pub mod search;
/// 任务列表接口。
pub mod tasks;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Tasklist：任务清单资源
#[derive(Clone)]
pub struct Tasklist {
    config: Arc<Config>,
}

impl Tasklist {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateTasklistRequest {
        create::CreateTasklistRequest::new(self.config.clone())
    }

    /// 创建获取详情请求。
    pub fn get(&self, tasklist_guid: impl Into<String>) -> get::GetTasklistRequest {
        get::GetTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 创建更新请求。
    pub fn update(&self, tasklist_guid: impl Into<String>) -> update::UpdateTasklistRequest {
        update::UpdateTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, tasklist_guid: impl Into<String>) -> delete::DeleteTasklistRequest {
        delete::DeleteTasklistRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListTasklistsRequest {
        list::ListTasklistsRequest::new(self.config.clone())
    }

    /// 获取动态订阅资源
    pub fn activity_subscription(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> activity_subscription::ActivitySubscriptionResource {
        activity_subscription::ActivitySubscriptionResource::new(
            self.config.clone(),
            tasklist_guid.into(),
        )
    }

    /// 获取清单任务列表
    pub fn tasks(&self, tasklist_guid: impl Into<String>) -> tasks::GetTasklistTasksRequest {
        tasks::GetTasklistTasksRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 添加清单成员
    pub fn add_members(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> add_members::AddTasklistMembersRequest {
        add_members::AddTasklistMembersRequest::new(self.config.clone(), tasklist_guid.into())
    }

    /// 移除清单成员
    pub fn remove_members(
        &self,
        tasklist_guid: impl Into<String>,
    ) -> remove_members::RemoveTasklistMembersRequest {
        remove_members::RemoveTasklistMembersRequest::new(self.config.clone(), tasklist_guid.into())
    }
    /// 创建搜索清单请求。
    pub fn search(&self) -> search::SearchTasklistRequest {
        search::SearchTasklistRequest::new(self.config.clone())
    }
}

// 重新导出请求类型
pub use add_members::AddTasklistMembersRequest;
pub use create::CreateTasklistRequest;
pub use delete::DeleteTasklistRequest;
pub use get::GetTasklistRequest;
pub use list::ListTasklistsRequest;
pub use patch::UpdateTasklistRequest;
pub use remove_members::RemoveTasklistMembersRequest;
pub use search::SearchTasklistRequest;
pub use tasks::GetTasklistTasksRequest;

// 重新导出响应类型
pub use add_members::{AddTasklistMembersBody, AddTasklistMembersResponse, TasklistMember};
pub use models::{
    CreateTasklistBody, CreateTasklistResponse, DeleteTasklistResponse, GetTasklistResponse,
    ListTasklistsResponse, TasklistIcon, TasklistItem, UpdateTasklistBody, UpdateTasklistResponse,
};
pub use remove_members::{RemoveTasklistMembersBody, RemoveTasklistMembersResponse};

#[cfg(test)]
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
    fn test_tasklist_new() {
        let config = create_test_config();
        let _tasklist = Tasklist::new(config);
    }

    #[test]
    fn test_tasklist_create() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.create();
    }

    #[test]
    fn test_tasklist_get() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.get("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_update() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.update("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_delete() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.delete("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_list() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.list();
    }

    #[test]
    fn test_tasklist_activity_subscription() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _resource = tasklist.activity_subscription("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_tasks() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.tasks("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_add_members() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.add_members("tasklist_guid_123");
    }

    #[test]
    fn test_tasklist_remove_members() {
        let config = create_test_config();
        let tasklist = Tasklist::new(config);
        let _request = tasklist.remove_members("tasklist_guid_123");
    }
}
