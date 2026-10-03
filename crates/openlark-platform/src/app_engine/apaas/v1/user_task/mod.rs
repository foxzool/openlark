//! 人工任务模块

pub mod cc;
pub mod chat_group;
pub mod expediting;
pub mod query;
pub mod rollback;
pub mod rollback_points;

use openlark_core::config::Config;

/// user_task 资源服务
#[derive(Debug, Clone)]
pub struct UserTaskService {
    config: Config,
}

impl UserTaskService {
    /// 创建新的 user_task 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查询用户任务
    pub fn query(&self) -> query::UserTaskQueryRequestBuilder {
        query::UserTaskQueryRequestBuilder::new(self.config.clone())
    }

    /// 抄送用户任务（接收 task_id 路径参数）
    pub fn cc(&self, task_id: impl Into<String>) -> cc::CcTaskRequestBuilder {
        cc::CcTaskRequestBuilder::new(self.config.clone(), task_id)
    }

    /// 催办用户任务（接收 task_id 路径参数）
    pub fn expediting(&self, task_id: impl Into<String>) -> expediting::ExpeditingRequestBuilder {
        expediting::ExpeditingRequestBuilder::new(self.config.clone(), task_id)
    }

    /// 创建群聊（接收 task_id 路径参数）
    pub fn chat_group(&self, task_id: impl Into<String>) -> chat_group::ChatGroupRequestBuilder {
        chat_group::ChatGroupRequestBuilder::new(self.config.clone(), task_id)
    }

    /// 获取退回节点列表（接收 task_id 路径参数）
    pub fn rollback_points(
        &self,
        task_id: impl Into<String>,
    ) -> rollback_points::RollbackPointsRequestBuilder {
        rollback_points::RollbackPointsRequestBuilder::new(self.config.clone(), task_id)
    }

    /// 退回用户任务（接收 task_id 与 node_id 两个路径参数）
    pub fn rollback(
        &self,
        task_id: impl Into<String>,
        node_id: impl Into<String>,
    ) -> rollback::RollbackTaskRequestBuilder {
        rollback::RollbackTaskRequestBuilder::new(self.config.clone(), task_id, node_id)
    }
}
