//! Approval Task module

pub mod add_assignee;
pub mod agree;
pub mod cancel;
pub mod reject;
/// 转交审批任务。
pub mod transfer;

use openlark_core::config::Config;

/// approval_task 资源服务
#[derive(Debug, Clone)]
pub struct ApprovalTaskService {
    config: Config,
}

impl ApprovalTaskService {
    /// 创建新的 approval_task 服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 同意审批任务
    pub fn agree(&self) -> agree::AgreeTaskRequestBuilder {
        agree::AgreeTaskRequestBuilder::new(self.config.clone())
    }

    /// 拒绝审批任务
    pub fn reject(&self) -> reject::RejectTaskRequestBuilder {
        reject::RejectTaskRequestBuilder::new(self.config.clone())
    }

    /// 取消审批任务
    pub fn cancel(&self) -> cancel::CancelTaskRequestBuilder {
        cancel::CancelTaskRequestBuilder::new(self.config.clone())
    }

    /// 添加审批人
    pub fn add_assignee(&self) -> add_assignee::AddAssigneeRequestBuilder {
        add_assignee::AddAssigneeRequestBuilder::new(self.config.clone())
    }

    /// 转交审批任务（接收 approval_task_id 与 transfer_to_user_id 两个路径参数）
    pub fn transfer(
        &self,
        approval_task_id: impl Into<String>,
        transfer_to_user_id: impl Into<String>,
    ) -> transfer::TransferApprovalTaskRequestBuilder {
        transfer::TransferApprovalTaskRequestBuilder::new(
            self.config.clone(),
            approval_task_id,
            transfer_to_user_id,
        )
    }
}
