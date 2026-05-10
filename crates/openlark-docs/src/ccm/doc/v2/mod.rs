#![allow(unused_variables)]

///[allow(unused_imports)]

/// 旧版文档服务 v2
///
/// 提供飞书旧版文档v2版本的完整管理功能，包括：
/// - 创建和初始化文档
/// - 获取文档元信息
/// - 获取文档内容（纯文本和富文本）
/// - 编辑文档内容
/// - 获取电子表格元数据
// 重新启用已修复的模块
// pub mod batch_update; // ✅ 已修复 // Generated: Module file not found
// pub mod content; // ✅ 已修复 // Generated: Module file not found
// pub mod meta; // ✅ 已修复 // Generated: Module file not found
//   pub mod sheet_meta; // ✅ 已修复 // Generated: Module file not found
// pub mod create; // Generated: Module file not found
pub mod models;

/// 重新导出模型类型
pub use self::models::*;
// pub mod requests; // Generated: Module file not found
// pub mod responses; // Generated: Module file not found

// 重新导出所有服务类型
// pub use batch_update::*; // Generated: Module use not found
// pub use content::*; // Generated: Module use not found
// pub use create::*; // Generated: Module use not found
// pub use meta::*; // Generated: Module use not found
// pub use sheet_meta::*; // ✅ 已修复 // Generated: Module use not found
