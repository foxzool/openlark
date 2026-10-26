/// 自定义角色协作者（member）服务模块
///
/// 提供多维表格高级权限中自定义角色的协作者管理能力：
/// - 新增协作者
/// - 批量新增协作者
/// - 列出协作者
/// - 删除协作者
/// - 批量删除协作者
pub mod batch_create;
/// batch_delete 子模块。
pub mod batch_delete;
/// create 子模块。
pub mod create;
/// delete 子模块。
pub mod delete;
/// list 子模块。
pub mod list;
/// models 子模块。
pub mod models;

/// 重新导出相关类型。
pub use batch_create::{BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse};
/// 重新导出相关类型。
pub use batch_delete::{
    BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberRequestBody, BatchDeleteRoleMemberResponse,
};
/// 重新导出相关类型。
pub use create::{CreateRoleMemberRequest, CreateRoleMemberRequestBody, CreateRoleMemberResponse};
/// 重新导出相关类型。
pub use delete::{DeleteRoleMemberRequest, DeleteRoleMemberResponse};
/// 重新导出相关类型。
pub use list::{ListRoleMembersRequest, ListRoleMembersResponse};
/// 重新导出相关类型。
pub use models::{RoleMemberId, RoleMemberIdType, RoleMemberInfo, RoleMemberType};

// Type alias for compatibility
/// 兼容旧版接口的服务类型占位符。
pub type ServiceType = ();
