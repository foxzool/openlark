//! OAuth API实现模块 (meta.project=oauth)
//!
//! 包含OAuth授权相关的API实现：
//! - authorization.v1/index: 获取登录预授权码

// old 模块显式导出
pub use old::{AuthorizationRequestBuilder, OAuthServiceOld};
#[allow(deprecated)]
pub use old::AuthorizationBuilder;

pub mod old;
