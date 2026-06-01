//! task_v2 资源入口。

#[path = "../../task/task/v2/task_v2/list_related_task.rs"]
pub mod list_related_task;
#[path = "../../task/task/v2/task_v2/task_subscription.rs"]
pub mod task_subscription;

use openlark_core::config::Config;
use std::sync::Arc;

/// TaskV2Resource：task_v2 资源。
#[derive(Clone)]
pub struct TaskV2Resource {
    config: Arc<Config>,
}

impl TaskV2Resource {
    /// 创建新的 task_v2 资源入口。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建列取与我相关的任务请求。
    pub fn list_related_task(&self) -> list_related_task::ListRelatedTaskRequest {
        list_related_task::ListRelatedTaskRequest::new(self.config.clone())
    }

    /// 创建订阅任务变更事件请求。
    pub fn task_subscription(&self) -> task_subscription::TaskSubscriptionRequest {
        task_subscription::TaskSubscriptionRequest::new(self.config.clone())
    }
}

pub use list_related_task::{ListRelatedTaskRequest, ListRelatedTaskResponse, RelatedTaskItem};
pub use task_subscription::{TaskSubscriptionRequest, TaskSubscriptionResponse};
