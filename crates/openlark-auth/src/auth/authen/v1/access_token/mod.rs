//! 用户访问令牌API实现
//!
//! 对应meta.resource=access_token

mod create;

// 重新导出用户访问令牌构建器
#[allow(deprecated)]
pub use create::UserAccessTokenV1Builder;
pub use create::UserAccessTokenV1RequestBuilder;
