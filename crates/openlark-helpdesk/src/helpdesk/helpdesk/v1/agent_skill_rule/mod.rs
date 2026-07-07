//! 客服技能规则模块
//!
//! 提供客服技能规则相关的 API。

pub mod list;

use openlark_core::config::Config;
use std::sync::Arc;

/// 客服技能规则服务
#[derive(Clone)]
pub struct AgentSkillRule {
    config: Arc<Config>,
}

impl AgentSkillRule {
    /// 创建新的客服技能规则服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取客服技能规则列表
    pub fn list(&self) -> list::ListAgentSkillRuleRequest {
        list::ListAgentSkillRuleRequest::new(self.config.clone())
    }
}

pub use list::{ListAgentSkillRuleRequest, ListAgentSkillRuleRequestBuilder};
