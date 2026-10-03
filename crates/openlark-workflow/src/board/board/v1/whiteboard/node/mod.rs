pub mod batch_delete;
pub mod create;
pub mod create_plantuml;
pub mod list;

// 重新导出子模块类型
pub use batch_delete::{
    BatchDeleteWhiteboardNodeBodyV1, BatchDeleteWhiteboardNodeRequestV1,
    BatchDeleteWhiteboardNodeResponseV1,
};
pub use create::{
    CreateWhiteboardNodeBodyV1, CreateWhiteboardNodeRequestV1, CreateWhiteboardNodeResponseV1,
    NodePosition,
};
pub use create_plantuml::{
    CreatePlantumlNodeBodyV1, CreatePlantumlNodeRequestV1, CreatePlantumlNodeResponseV1,
};
pub use list::{ListWhiteboardNodeRequestV1, ListWhiteboardNodeResponseV1, WhiteboardNode};
