/// download_as_image 模块。
pub mod download_as_image;
/// node 模块。
pub mod node;
/// theme 模块。
pub mod theme;
/// update_theme 模块。
pub mod update_theme;

// download_as_image 模块显式导出（官方 GET 无请求体、响应为二进制流，仅保留请求构建器）

pub use download_as_image::DownloadWhiteboardAsImageRequestV1;
// node 模块显式导出
pub use node::{
    BatchDeleteWhiteboardNodeBodyV1, BatchDeleteWhiteboardNodeRequestV1,
    BatchDeleteWhiteboardNodeResponseV1, CreatePlantumlNodeBodyV1, CreatePlantumlNodeRequestV1,
    CreatePlantumlNodeResponseV1, CreateWhiteboardNodeBodyV1, CreateWhiteboardNodeRequestV1,
    CreateWhiteboardNodeResponseV1, ListWhiteboardNodeRequestV1, ListWhiteboardNodeResponseV1,
    NodePosition, WhiteboardNode, WhiteboardNodeV1,
};
// theme 模块显式导出
pub use theme::{GetWhiteboardThemeRequestV1, GetWhiteboardThemeResponseV1, WhiteboardTheme};
// update_theme 模块显式导出
pub use update_theme::{
    UpdateWhiteboardThemeBodyV1, UpdateWhiteboardThemeRequestV1, UpdateWhiteboardThemeResponseV1,
};
