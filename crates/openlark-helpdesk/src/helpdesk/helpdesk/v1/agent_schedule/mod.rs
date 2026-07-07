//! 客服工作日程模块
//!
//! 提供客服工作日程相关的 API。

pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服工作日程服务
#[derive(Clone)]
pub struct AgentSchedule {
    config: Arc<Config>,
}

impl AgentSchedule {
    /// 创建新的客服工作日程服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服工作日程列表
    pub fn list(&self) -> list::ListAgentScheduleRequest {
        list::ListAgentScheduleRequest::new(self.config.clone())
    }

    /// 创建客服工作日程
    pub fn create(&self) -> create::CreateAgentScheduleRequest {
        create::CreateAgentScheduleRequest::new(self.config.clone())
    }

    /// 获取指定客服的工作日程
    pub fn get(&self, agent_id: impl Into<String>) -> get::GetAgentScheduleRequest {
        get::GetAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }

    /// 更新指定客服的工作日程
    pub fn patch(&self, agent_id: impl Into<String>) -> patch::PatchAgentScheduleRequest {
        patch::PatchAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }

    /// 删除指定客服的工作日程
    pub fn delete(&self, agent_id: impl Into<String>) -> delete::DeleteAgentScheduleRequest {
        delete::DeleteAgentScheduleRequest::new(self.config.clone(), agent_id.into())
    }
}

pub use create::{CreateAgentScheduleRequest, CreateAgentScheduleRequestBuilder};
pub use delete::{DeleteAgentScheduleRequest, DeleteAgentScheduleRequestBuilder};
pub use get::{GetAgentScheduleRequest, GetAgentScheduleRequestBuilder};
pub use list::{ListAgentScheduleRequest, ListAgentScheduleRequestBuilder};
pub use patch::{PatchAgentScheduleRequest, PatchAgentScheduleRequestBuilder};
