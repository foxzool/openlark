/// HR API 端点定义。
pub mod api_endpoints;
/// HR 通用模型定义。
pub mod models;
/// 域无关的 HR 共享原型类型（i18n 文本、ID+name 引用、分页壳等）。
pub mod shared_models;

/// 重新导出模型类型
pub use self::models::*;
