/// base 项目模块
///
/// base 项目当前仅包含 `base/v2`（自定义角色）相关接口。
pub mod v2;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
// v2 模块显式导出
pub use v2::{
    AppRole, Create, CreateReq, CreateResp, List, ListReq, ListResp, Update, UpdateReq, UpdateResp,
};
