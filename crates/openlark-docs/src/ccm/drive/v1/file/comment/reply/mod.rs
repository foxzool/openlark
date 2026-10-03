use openlark_core::{SDKResult, validate_required};

/// 删除评论回复接口。
pub mod delete;
/// 列举评论回复接口。
pub mod list;
/// 评论回复模型模块。
pub mod models;
/// 更新评论回复接口。
pub mod update;

/// 校验评论回复列表类接口的文件类型参数。
pub fn validate_comment_file_type_for_list_like(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

/// 重新导出删除评论回复类型。
pub use delete::{DeleteCommentReplyRequest, DeleteCommentReplyResponse};
/// 重新导出列举评论回复类型。
pub use list::{ListCommentReplyRequest, ListCommentReplyResponse};
/// 重新导出评论回复模型。
pub use models::{DocsLink, Person, ReplyContent, ReplyElement, ReplyExtra, ReplyInfo, TextRun};
/// 重新导出更新评论回复类型。
pub use update::{UpdateReplyRequest, UpdateReplyResponse};
/// create 模块。
pub mod create;
