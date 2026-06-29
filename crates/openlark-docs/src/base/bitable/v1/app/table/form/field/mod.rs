/// list 子模块。
pub mod list;
/// models 子模块。
pub mod models;
/// patch 子模块。
pub mod patch;

// 使用通配符导出所有子模块
// list 模块显式导出
/// 重新导出相关类型。
pub use list::{FormFieldQuestion, ListFormFieldQuestionRequest, ListFormFieldQuestionResponse};
// models 模块显式导出
/// 重新导出相关类型。
pub use models::PatchFormFieldRequest;
// patch 模块显式导出
/// 重新导出相关类型。
pub use patch::{
    PatchFormFieldQuestionRequest, PatchFormFieldQuestionRequestBuilder,
    PatchFormFieldQuestionResponse, PatchedFormFieldQuestion,
};

// 旧名兼容别名（deprecated alias，v1.0 移除）
#[allow(deprecated)]
pub use patch::PatchFormFieldQuestionBuilder;
