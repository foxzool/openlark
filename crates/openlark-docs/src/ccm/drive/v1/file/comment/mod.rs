use openlark_core::{SDKResult, validate_required};

/// 批量查询评论接口。
pub mod batch_query;
/// 创建评论接口。
pub mod create;
/// 获取评论接口。
pub mod get;
/// 列举评论接口。
pub mod list;
/// 评论模型模块。
pub mod models;
/// 更新评论接口。
pub mod patch;
/// 评论回复模块。
pub mod reply;

/// 校验适用于列表类接口的文件类型参数。
pub fn validate_comment_file_type_for_list_like(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

/// 校验创建评论时的文件类型参数。
pub fn validate_comment_file_type_for_create(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

/// 校验获取评论时的文件类型参数。
pub fn validate_comment_file_type_for_get(file_type: &str) -> SDKResult<()> {
    validate_required!(file_type.trim(), "file_type不能为空");
    Ok(())
}

/// 重新导出批量查询评论类型。
pub use batch_query::{BatchQueryCommentRequest, BatchQueryCommentResponse};
/// 重新导出创建评论请求。
pub use create::CreateCommentRequest;
/// 重新导出获取评论类型。
pub use get::{GetCommentRequest, GetCommentResponse};
/// 重新导出列举评论类型。
pub use list::{ListCommentsRequest, ListCommentsResponse};
/// 重新导出评论模型。
pub use models::{
    Comment, CommentContent, CommentElement, CreateCommentReply, CreateCommentReplyList, DocsLink,
    Person, ReplyExtra, TextRun,
};
/// 重新导出更新评论类型。
pub use patch::{PatchCommentRequest, PatchCommentResponse};
/// 重新导出评论回复类型。
pub use reply::{
    DeleteCommentReplyRequest, DeleteCommentReplyResponse, ListCommentReplyRequest,
    ListCommentReplyResponse,
};
