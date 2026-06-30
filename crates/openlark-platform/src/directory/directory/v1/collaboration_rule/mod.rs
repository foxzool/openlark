//! 关联组织可见规则相关 API

use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod list;
pub mod update;

/// collaboration_rule 资源服务
#[derive(Debug, Clone)]
pub struct CollaborationRuleService {
    config: Config,
}

impl CollaborationRuleService {
    /// 创建新的 collaboration_rule 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建可见规则
    pub fn create(&self, name: impl Into<String>) -> create::CollaborationRuleCreateRequestBuilder {
        create::CollaborationRuleCreateRequestBuilder::new(self.config.clone(), name)
    }

    /// 删除可见规则
    pub fn delete(
        &self,
        collaboration_rule_id: impl Into<String>,
    ) -> delete::CollaborationRuleDeleteRequestBuilder {
        delete::CollaborationRuleDeleteRequestBuilder::new(
            self.config.clone(),
            collaboration_rule_id,
        )
    }

    /// 可见规则列表
    pub fn list(&self) -> list::CollaborationRuleListRequestBuilder {
        list::CollaborationRuleListRequestBuilder::new(self.config.clone())
    }

    /// 更新可见规则
    pub fn update(
        &self,
        collaboration_rule_id: impl Into<String>,
    ) -> update::CollaborationRuleUpdateRequestBuilder {
        update::CollaborationRuleUpdateRequestBuilder::new(
            self.config.clone(),
            collaboration_rule_id,
        )
    }
}
