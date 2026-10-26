//! 关联组织可见规则相关 API

use openlark_core::config::Config;
use serde::{Deserialize, Serialize};

pub mod create;
pub mod delete;
pub mod list;
pub mod update;

/// 可搜可见规则实体（官方 collaboration_rule_entities）
///
/// 规则主体（subjects）与客体（objects）共用的实体结构；
/// 三个列表均为可选，官方要求实体数量之和小于 100，
/// 且主体/客体不能同时为空（官方错误码 2223106）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollaborationRuleEntities {
    /// 用户 open id 列表（从我方通讯录/组织架构接口获取）
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_user_ids: Vec<String>,
    /// 部门 open id 列表（"0" 代表全部成员）
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_department_ids: Vec<String>,
    /// 用户组 open id 列表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_group_ids: Vec<String>,
}

impl CollaborationRuleEntities {
    /// 创建空的实体集合
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户 open id 列表
    pub fn with_open_user_ids(
        mut self,
        open_user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.open_user_ids = open_user_ids.into_iter().map(Into::into).collect();
        self
    }

    /// 设置部门 open id 列表
    pub fn with_open_department_ids(
        mut self,
        open_department_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.open_department_ids = open_department_ids.into_iter().map(Into::into).collect();
        self
    }

    /// 设置用户组 open id 列表
    pub fn with_open_group_ids(
        mut self,
        open_group_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.open_group_ids = open_group_ids.into_iter().map(Into::into).collect();
        self
    }

    /// 实体总数（用户 + 部门 + 用户组）
    pub fn total(&self) -> usize {
        self.open_user_ids.len() + self.open_department_ids.len() + self.open_group_ids.len()
    }

    /// 是否未设置任何实体
    pub fn is_empty(&self) -> bool {
        self.total() == 0
    }
}

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
    ///
    /// `target_tenant_key` 为对方组织的 tenant key（必填查询参数）。
    pub fn create(
        &self,
        target_tenant_key: impl Into<String>,
    ) -> create::CollaborationRuleCreateRequestBuilder {
        create::CollaborationRuleCreateRequestBuilder::new(self.config.clone(), target_tenant_key)
    }

    /// 删除可见规则
    ///
    /// `target_tenant_key` 为对方组织的 tenant key（必填查询参数）。
    pub fn delete(
        &self,
        collaboration_rule_id: impl Into<String>,
        target_tenant_key: impl Into<String>,
    ) -> delete::CollaborationRuleDeleteRequestBuilder {
        delete::CollaborationRuleDeleteRequestBuilder::new(
            self.config.clone(),
            collaboration_rule_id,
            target_tenant_key,
        )
    }

    /// 可见规则列表
    ///
    /// `target_tenant_key` 为对方组织的 tenant key（必填查询参数）。
    pub fn list(
        &self,
        target_tenant_key: impl Into<String>,
    ) -> list::CollaborationRuleListRequestBuilder {
        list::CollaborationRuleListRequestBuilder::new(self.config.clone(), target_tenant_key)
    }

    /// 更新可见规则
    ///
    /// `target_tenant_key` 为对方组织的 tenant key（必填查询参数）。
    pub fn update(
        &self,
        collaboration_rule_id: impl Into<String>,
        target_tenant_key: impl Into<String>,
    ) -> update::CollaborationRuleUpdateRequestBuilder {
        update::CollaborationRuleUpdateRequestBuilder::new(
            self.config.clone(),
            collaboration_rule_id,
            target_tenant_key,
        )
    }
}
