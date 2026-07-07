//! 工单消息模块
//!
//! 提供工单消息相关的 API。

pub mod create;
/// 列表接口。
pub mod list;

/// 工单消息服务（挂载在 Ticket 下）
#[derive(Clone)]
pub struct TicketMessage<'a> {
    ticket: &'a super::Ticket,
}

impl<'a> TicketMessage<'a> {
    /// 创建新的工单消息服务实例
    pub fn new(ticket: &'a super::Ticket) -> Self {
        Self { ticket }
    }

    /// 获取工单消息列表
    pub fn list(&self, ticket_id: impl Into<String>) -> list::ListTicketMessageRequest {
        list::ListTicketMessageRequest::new(self.ticket.config.clone(), ticket_id.into())
    }

    /// 发送工单消息
    pub fn create(&self, ticket_id: impl Into<String>) -> create::CreateTicketMessageRequest {
        create::CreateTicketMessageRequest::new(self.ticket.config.clone(), ticket_id.into())
    }
}

pub use create::{CreateTicketMessageRequest, CreateTicketMessageRequestBuilder};
pub use list::{ListTicketMessageRequest, ListTicketMessageRequestBuilder};
