/// 校验协作者权限接口。
pub mod auth;
/// 批量新增协作者接口。
pub mod batch_create;
/// 新增协作者接口。
pub mod create;
/// 删除协作者接口。
pub mod delete;
/// 列表协作者接口。
pub mod list;
/// 协作者权限模型模块。
pub mod models;
/// 转移拥有者接口。
pub mod transfer_owner;
/// 更新协作者权限接口。
pub mod update;

/// 重新导出校验协作者权限类型。
pub use auth::{AuthPermissionMemberRequest, AuthPermissionMemberResponse};
/// 重新导出批量新增协作者类型。
pub use batch_create::{
    BatchCreatePermissionMemberBody, BatchCreatePermissionMemberRequest,
    BatchCreatePermissionMemberResponse,
};
/// 重新导出新增协作者类型。
pub use create::{
    CreatePermissionMemberRequest, CreatePermissionMemberRequestBody,
    CreatePermissionMemberResponse,
};
/// 重新导出删除协作者类型。
pub use delete::{DeletePermissionMemberRequest, DeletePermissionMemberResponse};
/// 重新导出列表协作者类型。
pub use list::{ListPermissionMembersRequest, ListPermissionMembersResponse};
/// 重新导出协作者权限模型。
pub use models::{PermissionMember, PermissionMemberItem};
/// 重新导出转移拥有者类型。
pub use transfer_owner::{TransferOwnerRequest, TransferOwnerRequestBody, TransferOwnerResponse};
/// 重新导出更新协作者权限类型。
pub use update::{
    UpdatePermissionMemberRequest, UpdatePermissionMemberRequestBody,
    UpdatePermissionMemberResponse,
};
