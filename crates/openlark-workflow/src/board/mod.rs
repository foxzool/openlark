//! 看板模块
//!
//! 提供飞书任务看板相关 API 能力，包含白板节点管理、主题管理以及图片导出能力。
//!
//! ## 主要功能
//! - `board`: 看板资源子模块入口
//! - 白板节点：创建、查询与列表管理
//! - 看板主题与导出：主题读取/更新、白板导出为图片

#[allow(clippy::module_inception)]
/// Board 子模块。
pub mod board;

// board 模块显式导出

pub use board::{
    BatchDeleteWhiteboardNodeBodyV1, BatchDeleteWhiteboardNodeRequestV1,
    BatchDeleteWhiteboardNodeResponseV1, CreatePlantumlNodeBodyV1, CreatePlantumlNodeRequestV1,
    CreatePlantumlNodeResponseV1, CreateWhiteboardNodeBodyV1, CreateWhiteboardNodeRequestV1,
    CreateWhiteboardNodeResponseV1, DownloadWhiteboardAsImageBodyV1,
    DownloadWhiteboardAsImageRequestV1, DownloadWhiteboardAsImageResponseV1,
    GetWhiteboardThemeRequestV1, GetWhiteboardThemeResponseV1, ListWhiteboardNodeRequestV1,
    ListWhiteboardNodeResponseV1, NodePosition, UpdateWhiteboardThemeBodyV1,
    UpdateWhiteboardThemeRequestV1, UpdateWhiteboardThemeResponseV1, WhiteboardNode,
    WhiteboardTheme,
};
