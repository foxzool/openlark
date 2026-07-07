//! 工单自定义字段模块
//!
//! 提供工单自定义字段相关的 API。

/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
pub mod list;
/// 更新接口。
pub mod patch;

use openlark_core::config::Config;
use std::sync::Arc;

/// 工单自定义字段服务
#[derive(Clone)]
pub struct TicketCustomizedField {
    config: Arc<Config>,
}

impl TicketCustomizedField {
    /// 创建新的工单自定义字段服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取工单自定义字段列表
    pub fn list(&self) -> list::ListTicketCustomizedFieldRequest {
        list::ListTicketCustomizedFieldRequest::new(self.config.clone())
    }

    /// 创建工单自定义字段
    pub fn create(&self) -> create::CreateTicketCustomizedFieldRequest {
        create::CreateTicketCustomizedFieldRequest::new(self.config.clone())
    }

    /// 获取指定工单自定义字段
    pub fn get(&self, id: impl Into<String>) -> get::GetTicketCustomizedFieldRequest {
        get::GetTicketCustomizedFieldRequest::new(self.config.clone(), id.into())
    }

    /// 更新指定工单自定义字段
    pub fn patch(&self, id: impl Into<String>) -> patch::PatchTicketCustomizedFieldRequest {
        patch::PatchTicketCustomizedFieldRequest::new(self.config.clone(), id.into())
    }

    /// 删除指定工单自定义字段
    pub fn delete(&self, id: impl Into<String>) -> delete::DeleteTicketCustomizedFieldRequest {
        delete::DeleteTicketCustomizedFieldRequest::new(self.config.clone(), id.into())
    }
}

pub use create::{CreateTicketCustomizedFieldRequest, CreateTicketCustomizedFieldRequestBuilder};
pub use delete::{DeleteTicketCustomizedFieldRequest, DeleteTicketCustomizedFieldRequestBuilder};
pub use get::{GetTicketCustomizedFieldRequest, GetTicketCustomizedFieldRequestBuilder};
pub use list::{ListTicketCustomizedFieldRequest, ListTicketCustomizedFieldRequestBuilder};
pub use patch::{PatchTicketCustomizedFieldRequest, PatchTicketCustomizedFieldRequestBuilder};
