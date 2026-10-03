//! OAuth旧版本API实现

mod default;

// 重新导出授权构建器和服务
#[allow(deprecated)]
pub use default::AuthorizationBuilder;
pub use default::{AuthorizationRequestBuilder, OAuthServiceOld};
