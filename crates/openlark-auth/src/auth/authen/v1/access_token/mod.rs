//! 用户访问令牌API实现
//!
//! 对应meta.resource=access_token

mod create;

// 重新导出用户访问令牌构建器
pub use create::UserAccessTokenV1RequestBuilder;
#[allow(deprecated)]
pub use create::UserAccessTokenV1Builder;
