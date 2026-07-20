/// 创建接口。
pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// 列表接口。
pub mod list;
/// 数据模型。
pub mod models;
/// 更新接口。
pub mod update;

use openlark_core::config::Config;
use std::sync::Arc;

/// Comment：评论资源
#[derive(Clone)]
pub struct Comment {
    config: Arc<Config>,
    task_guid: String,
}

impl Comment {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            task_guid: String::new(),
        }
    }

    /// 绑定任务上下文。
    pub fn with_task(mut self, task_guid: impl Into<String>) -> Self {
        self.task_guid = task_guid.into();
        self
    }

    /// 创建新建请求。
    pub fn create(&self) -> create::CreateCommentRequest {
        create::CreateCommentRequest::new(self.config.clone(), self.task_guid.clone())
    }

    /// 创建获取详情请求。
    pub fn get(&self, comment_guid: impl Into<String>) -> get::GetCommentRequest {
        get::GetCommentRequest::new(self.config.clone(), comment_guid.into())
    }

    /// 创建更新请求。
    pub fn update(&self, comment_guid: impl Into<String>) -> update::UpdateCommentRequest {
        update::UpdateCommentRequest::new(self.config.clone(), comment_guid.into())
    }

    /// 创建删除请求。
    pub fn delete(&self, comment_guid: impl Into<String>) -> delete::DeleteCommentRequest {
        delete::DeleteCommentRequest::new(self.config.clone(), comment_guid.into())
    }

    /// 创建列表请求。
    pub fn list(&self) -> list::ListCommentsRequest {
        list::ListCommentsRequest::new(self.config.clone(), self.task_guid.clone())
    }
}

// 重新导出请求类型
pub use create::CreateCommentRequest;
pub use delete::DeleteCommentRequest;
pub use get::GetCommentRequest;
pub use list::ListCommentsRequest;
pub use update::UpdateCommentRequest;

// 重新导出响应类型
pub use models::{
    CommentCreator, CommentItem, CreateCommentBody, CreateCommentResponse, DeleteCommentResponse,
    GetCommentResponse, InputComment, ListCommentsResponse, UpdateCommentBody,
    UpdateCommentResponse,
};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<Config> {
        Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        )
    }

    #[test]
    fn test_comment_new() {
        let config = create_test_config();
        let comment = Comment::new(config);
        assert!(comment.task_guid.is_empty());
    }

    #[test]
    fn test_comment_with_task() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        assert_eq!(comment.task_guid, "task_123");
    }

    #[test]
    fn test_comment_create() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.create();
    }

    #[test]
    fn test_comment_get() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.get("comment_456");
    }

    #[test]
    fn test_comment_update() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.update("comment_456");
    }

    #[test]
    fn test_comment_delete() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.delete("comment_456");
    }

    #[test]
    fn test_comment_list() {
        let config = create_test_config();
        let comment = Comment::new(config).with_task("task_123");
        let _request = comment.list();
    }
}
