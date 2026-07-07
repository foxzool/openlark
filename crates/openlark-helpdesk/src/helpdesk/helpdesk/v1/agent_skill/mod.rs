//! 客服技能模块
//!
//! 提供客服技能相关的 API。

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

/// 客服技能服务
#[derive(Clone)]
pub struct AgentSkill {
    config: Arc<Config>,
}

impl AgentSkill {
    /// 创建新的客服技能服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服技能列表
    pub fn list(&self) -> list::ListAgentSkillRequest {
        list::ListAgentSkillRequest::new(self.config.clone())
    }

    /// 创建客服技能
    pub fn create(&self) -> create::CreateAgentSkillRequest {
        create::CreateAgentSkillRequest::new(self.config.clone())
    }

    /// 获取指定客服技能
    pub fn get(&self, agent_skill_id: impl Into<String>) -> get::GetAgentSkillRequest {
        get::GetAgentSkillRequest::new(self.config.clone(), agent_skill_id.into())
    }

    /// 更新指定客服技能
    pub fn patch(&self, agent_skill_id: impl Into<String>) -> patch::PatchAgentSkillRequest {
        patch::PatchAgentSkillRequest::new(self.config.clone(), agent_skill_id.into())
    }

    /// 删除指定客服技能
    pub fn delete(&self, agent_skill_id: impl Into<String>) -> delete::DeleteAgentSkillRequest {
        delete::DeleteAgentSkillRequest::new(self.config.clone(), agent_skill_id.into())
    }
}

pub use create::{CreateAgentSkillRequest, CreateAgentSkillRequestBuilder};
pub use delete::{DeleteAgentSkillRequest, DeleteAgentSkillRequestBuilder};
pub use get::{GetAgentSkillRequest, GetAgentSkillRequestBuilder};
pub use list::{ListAgentSkillRequest, ListAgentSkillRequestBuilder};
pub use patch::{PatchAgentSkillRequest, PatchAgentSkillRequestBuilder};
