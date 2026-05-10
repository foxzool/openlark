/// Docs文档服务 v1
///
/// 提供飞书文档v1版本的完整管理功能，包括：
/// - 创建和删除文档
/// - 文档信息查询和管理
/// - 文档内容读取和编辑
/// - 文档块操作和管理
// pub mod document; // Generated: Module file not found
pub mod models;

/// 重新导出模型类型
pub use self::models::*;
// pub mod requests; // Generated: Module file not found
// pub mod responses; // Generated: Module file not found

// 重新导出所有服务类型
// pub use document::*; // Generated: Module use not found
