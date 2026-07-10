//! 推送通知模块
//!
//! 提供客服推送通知相关的 API。

pub mod cancel_approve;
/// cancel_send 模块。
pub mod cancel_send;
/// 创建接口。
pub mod create;
/// execute_send 模块。
pub mod execute_send;
/// 获取接口。
pub mod get;
/// 更新接口。
pub mod patch;
/// preview 模块。
pub mod preview;
/// submit_approve 模块。
pub mod submit_approve;

use openlark_core::config::Config;
use std::sync::Arc;

/// 推送通知服务
#[derive(Clone)]
pub struct Notification {
    config: Arc<Config>,
}

impl Notification {
    /// 创建新的推送通知服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建推送通知
    pub fn create(&self) -> create::CreateNotificationRequest {
        create::CreateNotificationRequest::new(self.config.clone())
    }

    /// 获取指定推送通知
    pub fn get(&self, notification_id: impl Into<String>) -> get::GetNotificationRequest {
        get::GetNotificationRequest::new(self.config.clone(), notification_id.into())
    }

    /// 更新指定推送通知
    pub fn patch(&self, notification_id: impl Into<String>) -> patch::PatchNotificationRequest {
        patch::PatchNotificationRequest::new(self.config.clone(), notification_id.into())
    }

    /// 提交审核
    pub fn submit_approve(
        &self,
        notification_id: impl Into<String>,
    ) -> submit_approve::SubmitApproveNotificationRequest {
        submit_approve::SubmitApproveNotificationRequest::new(
            self.config.clone(),
            notification_id.into(),
        )
    }

    /// 取消审核
    pub fn cancel_approve(
        &self,
        notification_id: impl Into<String>,
    ) -> cancel_approve::CancelApproveNotificationRequest {
        cancel_approve::CancelApproveNotificationRequest::new(
            self.config.clone(),
            notification_id.into(),
        )
    }

    /// 执行推送
    pub fn execute_send(
        &self,
        notification_id: impl Into<String>,
    ) -> execute_send::ExecuteSendNotificationRequest {
        execute_send::ExecuteSendNotificationRequest::new(
            self.config.clone(),
            notification_id.into(),
        )
    }

    /// 预览推送
    pub fn preview(
        &self,
        notification_id: impl Into<String>,
    ) -> preview::PreviewNotificationRequest {
        preview::PreviewNotificationRequest::new(self.config.clone(), notification_id.into())
    }

    /// 取消推送
    pub fn cancel_send(
        &self,
        notification_id: impl Into<String>,
    ) -> cancel_send::CancelSendNotificationRequest {
        cancel_send::CancelSendNotificationRequest::new(self.config.clone(), notification_id.into())
    }
}

pub use cancel_approve::{
    CancelApproveNotificationRequest, CancelApproveNotificationRequestBuilder,
};
pub use cancel_send::{CancelSendNotificationRequest, CancelSendNotificationRequestBuilder};
pub use create::{CreateNotificationRequest, CreateNotificationRequestBuilder};
pub use execute_send::{ExecuteSendNotificationRequest, ExecuteSendNotificationRequestBuilder};
pub use get::{GetNotificationRequest, GetNotificationRequestBuilder};
pub use patch::{PatchNotificationRequest, PatchNotificationRequestBuilder};
pub use preview::{PreviewNotificationRequest, PreviewNotificationRequestBuilder};
pub use submit_approve::{
    SubmitApproveNotificationRequest, SubmitApproveNotificationRequestBuilder,
};
