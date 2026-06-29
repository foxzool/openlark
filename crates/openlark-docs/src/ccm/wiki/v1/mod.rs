/// Wiki V1 API 模块
pub mod node;

// 使用通配符导出所有子模块
// node 模块显式导出
pub use node::{SearchWikiRequest, SearchWikiResponse};
