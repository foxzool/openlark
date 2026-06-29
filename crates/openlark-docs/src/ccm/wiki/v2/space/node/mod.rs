/// Wiki V2 空间节点管理模块
///
/// 提供知识空间节点的创建、列表、移动、复制、标题更新等操作。
pub mod copy;
pub mod create;
pub mod list;
pub mod r#move;
pub mod move_docs_to_wiki;
pub mod update_title;

// 显式导出 - 避免使用 glob reexport
pub use copy::{CopyWikiSpaceNodeParams, CopyWikiSpaceNodeRequest, CopyWikiSpaceNodeResponse};

pub use create::{
    CreateWikiSpaceNodeParams, CreateWikiSpaceNodeRequest, CreateWikiSpaceNodeResponse,
};

pub use list::{ListWikiSpaceNodesParams, ListWikiSpaceNodesRequest, ListWikiSpaceNodesResponse};

pub use move_docs_to_wiki::{MoveDocsToWikiRequest, MoveDocsToWikiResponse};

pub use r#move::{MoveWikiSpaceNodeParams, MoveWikiSpaceNodeRequest, MoveWikiSpaceNodeResponse};

pub use update_title::{
    UpdateWikiSpaceNodeTitleParams, UpdateWikiSpaceNodeTitleRequest,
    UpdateWikiSpaceNodeTitleResponse,
};
