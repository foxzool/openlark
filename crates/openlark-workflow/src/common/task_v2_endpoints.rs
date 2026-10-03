//! Task v2 新增端点。

/// Task v2 API 端点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskV2Endpoint {
    /// 设置父任务。
    TaskSetAncestorTask(String),
    /// 列取与我相关的任务。
    ListRelatedTask,
    /// 订阅任务变更事件。
    TaskSubscription,
}

impl TaskV2Endpoint {
    /// 生成对应的 URL。
    pub fn to_url(&self) -> String {
        match self {
            TaskV2Endpoint::TaskSetAncestorTask(task_guid) => {
                format!("/open-apis/task/v2/tasks/{task_guid}/set_ancestor_task")
            }
            TaskV2Endpoint::ListRelatedTask => {
                "/open-apis/task/v2/task_v2/list_related_task".to_string()
            }
            TaskV2Endpoint::TaskSubscription => {
                "/open-apis/task/v2/task_v2/task_subscription".to_string()
            }
        }
    }
}
