//! 身份信息

pub mod create;

pub use create::{HumanAuthenticationUserIdType, IdentityCreateRequestBuilder, IdentityCreateResponse};
// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use create::IdentityCreateBuilder;
