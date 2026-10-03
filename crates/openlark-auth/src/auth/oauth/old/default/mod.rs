//! OAuth旧版本default路径API实现

mod index;

// 重新导出授权构建器和服务
#[allow(deprecated)]
pub use index::AuthorizationBuilder;
pub use index::{AuthorizationRequestBuilder, OAuthServiceOld};
