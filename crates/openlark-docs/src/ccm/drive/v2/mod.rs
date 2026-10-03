/// Drive v2 文件接口模块。
pub mod file;
/// Drive v2 权限接口模块。
pub mod permission;

/// 重新导出文件点赞相关类型。
pub use file::{FileLike, ListFileLikesRequest, ListFileLikesResponse};
/// 重新导出公开权限相关类型。
pub use permission::{
    GetPermissionPublicRequest, GetPermissionPublicResponse, PermissionPublic,
    UpdatePermissionPublicRequest, UpdatePermissionPublicResponse,
};
/// comment_reaction 模块。
pub mod comment_reaction;
