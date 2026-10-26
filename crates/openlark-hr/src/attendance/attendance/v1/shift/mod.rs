//! 班次管理模块
//!
//! 包含创建、删除、查询、搜索班次等 API

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod models;
pub mod query;

pub use create::CreateShiftRequest;
pub use delete::DeleteShiftRequest;
pub use get::GetShiftRequest;
pub use list::ListShiftRequest;
pub use models::{
    CreateShiftRequestBody, CreateShiftResponse, DeleteShiftResponse, EarlyLeaveRule,
    GetShiftResponse, LateRule, ListShiftResponse, OvertimeRule, PunchPlace, PunchTime,
    PunchTimeRule, PunchWifi, QueryShiftResponse, RestRule, ShiftInfo, ShiftListItem,
};
pub use query::QueryShiftRequest;
