//! card.element
//!
//! 卡片组件相关 API（cardkit-v1）。

pub mod content;
pub mod create;
pub mod delete;
pub mod models;
pub mod patch;
pub mod update;

// content 模块显式导出

pub use content::{
    UpdateCardElementContentBody, UpdateCardElementContentRequest,
    UpdateCardElementContentRequestBuilder,
};
// create 模块显式导出
pub use create::{
    CreateCardElementBody, CreateCardElementRequest, CreateCardElementRequestBuilder,
};
// delete 模块显式导出
pub use delete::{
    DeleteCardElementBody, DeleteCardElementRequest, DeleteCardElementRequestBuilder,
};
// models 模块显式导出
pub use models::{
    CreateCardElementResponse, DeleteCardElementResponse, PatchCardElementResponse,
    UpdateCardElementContentResponse, UpdateCardElementResponse,
};
// patch 模块显式导出
pub use patch::{PatchCardElementBody, PatchCardElementRequest, PatchCardElementRequestBuilder};
// update 模块显式导出
pub use update::{
    UpdateCardElementBody, UpdateCardElementRequest, UpdateCardElementRequestBuilder,
};
