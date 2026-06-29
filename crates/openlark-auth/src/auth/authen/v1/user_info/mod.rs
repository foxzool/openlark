//! 用户信息API实现
//!
//! 本模块提供用户信息获取的API实现，根据meta.resource=user_info组织

mod get;

// 重新导出用户信息服务
#[allow(deprecated)]
pub use get::UserInfoBuilder;
pub use get::{UserInfoRequestBuilder, UserInfoService};
