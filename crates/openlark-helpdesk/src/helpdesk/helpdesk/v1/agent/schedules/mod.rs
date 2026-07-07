//! 客服工作日程模块 (agent.schedules)

pub mod delete;
/// 获取接口。
pub mod get;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服工作日程 API
#[derive(Clone)]
pub struct AgentSchedules {
    config: Arc<Config>,
    agent_id: String,
}

impl AgentSchedules {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, agent_id: impl Into<String>) -> Self {
        Self {
            config,
            agent_id: agent_id.into(),
        }
    }

    /// 创建删除请求。
    pub fn delete(self) -> delete::DeleteAgentScheduleRequest {
        delete::DeleteAgentScheduleRequest::new(self.config, self.agent_id)
    }

    /// 创建获取详情请求。
    pub fn get(&self) -> get::GetAgentScheduleRequest {
        get::GetAgentScheduleRequest::new(self.config.clone(), self.agent_id.clone())
    }

    /// 创建补丁请求。
    pub fn patch(self) -> patch::PatchAgentScheduleRequest {
        patch::PatchAgentScheduleRequest::new(self.config, self.agent_id)
    }
}
