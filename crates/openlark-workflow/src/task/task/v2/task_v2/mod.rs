//! task_v2 资源新增接口。

pub mod list_related_task;
pub mod task_subscription;

pub use list_related_task::{ListRelatedTaskRequest, ListRelatedTaskResponse, RelatedTaskItem};
pub use task_subscription::{TaskSubscriptionRequest, TaskSubscriptionResponse};
