//! Helpdesk 工单 API 模块

/// 回复用户提问接口。
pub mod answer_user_query;
/// 创建工单接口。
pub mod create;
/// 工单自定义字段接口。
pub mod customized_fields;
/// 获取工单详情接口。
pub mod get;
/// 工单列表接口。
pub mod list;
/// 工单消息接口。
pub mod message;
/// 工单数据模型。
pub mod models;
/// 拉起服务接口。
pub mod start_service;
/// 工单图片接口。
pub mod ticket_image;
/// 更新工单接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Ticket。
#[derive(Clone)]
pub struct Ticket {
    config: Arc<Config>,
}

impl Ticket {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 创建工单请求。
    pub fn create(&self) -> create::CreateTicketRequest {
        create::CreateTicketRequest::new(self.config.clone())
    }

    /// 获取工单详情请求。
    pub fn get(&self, ticket_id: impl Into<String>) -> get::GetTicketRequest {
        get::GetTicketRequest::new(self.config.clone(), ticket_id.into())
    }

    /// 更新工单请求。
    pub fn update(&self, ticket_id: impl Into<String>) -> update::UpdateTicketRequest {
        update::UpdateTicketRequest::new(self.config.clone(), ticket_id.into())
    }

    /// 工单列表请求。
    pub fn list(&self) -> list::TicketListRequest {
        list::TicketListRequest::new(self.config.clone())
    }

    /// 拉起服务请求。
    pub fn start_service(&self) -> start_service::StartServiceRequest {
        start_service::StartServiceRequest::new(self.config.clone())
    }

    /// 回复用户提问请求。
    pub fn answer_user_query(
        &self,
        ticket_id: impl Into<String>,
    ) -> answer_user_query::AnswerUserQueryRequest {
        answer_user_query::AnswerUserQueryRequest::new(self.config.clone(), ticket_id)
    }

    /// 获取工单图片请求。
    pub fn ticket_image(
        &self,
        ticket_id: impl Into<String>,
        image_key: impl Into<String>,
    ) -> ticket_image::GetTicketImageRequest {
        ticket_image::GetTicketImageRequest::new(self.config.clone(), ticket_id, image_key)
    }

    /// 获取工单自定义字段请求。
    pub fn customized_fields(&self) -> customized_fields::GetCustomizedFieldsRequest {
        customized_fields::GetCustomizedFieldsRequest::new(self.config.clone())
    }

    /// 访问工单消息 API。
    pub fn message(&self) -> message::TicketMessage<'_> {
        message::TicketMessage::new(self)
    }
}

pub use answer_user_query::AnswerUserQueryRequest;
pub use create::CreateTicketRequest;
pub use customized_fields::GetCustomizedFieldsRequest;
pub use get::GetTicketRequest;
pub use list::TicketListRequest;
pub use message::{
    CreateTicketMessageRequest, CreateTicketMessageRequestBuilder, ListTicketMessageRequest,
    ListTicketMessageRequestBuilder,
};
// models 模块显式导出
pub use models::{
    CreateTicketBody, CreateTicketResponse, DeleteTicketResponse, GetTicketResponse, TicketItem,
    TicketListResponse, UpdateTicketBody, UpdateTicketResponse,
};
pub use start_service::StartServiceRequest;
pub use ticket_image::GetTicketImageRequest;
pub use update::UpdateTicketRequest;
