//! OAuth旧版本API实现

mod default;

// 重新导出授权构建器和服务
pub use default::{AuthorizationRequestBuilder, OAuthServiceOld};
#[allow(deprecated)]
pub use default::AuthorizationBuilder;
