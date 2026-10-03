/// 创建 Wiki 空间接口。
pub mod create;
/// 获取 Wiki 空间接口。
pub mod get;
/// 获取 Wiki 节点接口。
pub mod get_node;
/// 列举 Wiki 空间接口。
pub mod list;
/// Wiki 空间成员接口。
pub mod member;
/// Wiki 空间节点接口。
pub mod node;
/// Wiki 空间设置接口。
pub mod setting;

/// 重新导出创建 Wiki 空间类型。
#[allow(deprecated)]
pub use create::{CreateWikiSpaceRequest, CreateWikiSpaceResponse};
/// 重新导出获取 Wiki 空间类型。
pub use get::{GetWikiSpaceRequest, GetWikiSpaceResponse};
/// 重新导出获取 Wiki 节点类型。
pub use get_node::{GetWikiSpaceNodeRequest, GetWikiSpaceNodeResponse};
/// 重新导出列举 Wiki 空间类型。
#[allow(deprecated)]
pub use list::{ListWikiSpacesRequest, ListWikiSpacesResponse};
/// 重新导出 Wiki 空间成员类型。
pub use member::{
    CreateWikiSpaceMemberRequest, CreateWikiSpaceMemberResponse, DeleteWikiSpaceMemberRequest,
    DeleteWikiSpaceMemberResponse, ListWikiSpaceMembersRequest, ListWikiSpaceMembersResponse,
};
/// 重新导出 Wiki 节点管理类型。
#[allow(deprecated)]
pub use node::{
    CopyWikiSpaceNodeParams, CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse,
    CreateWikiSpaceNodeParams, CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse,
    ListWikiSpaceNodesParams, ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse,
    MoveDocsToWikiRequest, MoveDocsToWikiResponse, MoveWikiSpaceNodeParams,
    MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse, UpdateWikiSpaceNodeTitleParams,
    UpdateWikiSpaceNodeTitleRequest, UpdateWikiSpaceNodeTitleResponse,
};
/// 重新导出 Wiki 空间设置类型。
pub use setting::{UpdateWikiSpaceSettingRequest, UpdateWikiSpaceSettingResponse};
