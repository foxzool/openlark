//! 邮件撤回模块

pub mod get_recall_detail;
pub mod recall;

use openlark_core::config::Config;
use std::sync::Arc;

/// Recall：邮件撤回资源入口
#[derive(Clone)]
pub struct Recall {
    config: Arc<Config>,
    mailbox_id: String,
    message_id: String,
}

impl Recall {
    /// 创建新的实例。
    pub fn new(
        config: Arc<Config>,
        mailbox_id: impl Into<String>,
        message_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
            message_id: message_id.into(),
        }
    }

    /// 撤回已发送邮件（POST）。
    pub fn recall(&self) -> recall::RecallMessageRequest {
        recall::RecallMessageRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            self.message_id.clone(),
        )
    }

    /// 获取邮件撤回进度（GET）。
    pub fn get_recall_detail(&self) -> get_recall_detail::GetRecallDetailRequest {
        get_recall_detail::GetRecallDetailRequest::new(
            self.config.clone(),
            self.mailbox_id.clone(),
            self.message_id.clone(),
        )
    }
}
