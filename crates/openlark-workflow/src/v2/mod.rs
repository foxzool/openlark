//! 任务 v2 入口模块
//!
//! 提供任务 API v2 的统一访问入口，聚合任务、任务清单、自定义字段、评论、分组与附件能力。
//!
//! ## 主要功能
//! - `task`: 任务资源模块入口
//! - `tasklist` / `custom_field`: 任务清单与自定义字段能力
//! - `comment` / `section` / `attachment`: 评论、分组与附件能力

/// 附件模块。
pub mod attachment;
/// 评论模块。
pub mod comment;
/// 自定义字段模块。
pub mod custom_field;
/// 分组模块。
pub mod section;
/// 任务模块。
pub mod task;
/// task_v2 资源模块。
pub mod task_v2;
/// 任务清单模块。
pub mod tasklist;

use openlark_core::config::Config;
use std::sync::Arc;

/// TaskV2：任务 API v2 访问入口
#[derive(Clone)]
pub struct TaskV2 {
    config: Arc<Config>,
}

impl TaskV2 {
    /// 创建新的任务 v2 API 入口。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问任务资源
    pub fn task(&self) -> task::Task {
        task::Task::new(self.config.clone())
    }

    /// 访问 task_v2 资源。
    pub fn task_v2(&self) -> task_v2::TaskV2Resource {
        task_v2::TaskV2Resource::new(self.config.clone())
    }

    /// 访问任务清单资源
    pub fn tasklist(&self) -> tasklist::Tasklist {
        tasklist::Tasklist::new(self.config.clone())
    }

    /// 访问自定义字段资源
    pub fn custom_field(&self) -> custom_field::CustomField {
        custom_field::CustomField::new(self.config.clone())
    }

    /// 访问评论资源
    pub fn comment(&self) -> comment::Comment {
        comment::Comment::new(self.config.clone())
    }

    /// 访问分组资源
    pub fn section(&self) -> section::Section {
        section::Section::new(self.config.clone())
    }

    /// 访问附件资源
    pub fn attachment(&self) -> attachment::Attachment {
        attachment::Attachment::new(self.config.clone())
    }
}

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
    fn test_task_v2_new() {
        let config = create_test_config();
        let _task_v2 = TaskV2::new(config);
    }

    #[test]
    fn test_task_v2_task() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _task = task_v2.task();
    }

    #[test]
    fn test_task_v2_tasklist() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _tasklist = task_v2.tasklist();
    }

    #[test]
    fn test_task_v2_custom_field() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _custom_field = task_v2.custom_field();
    }

    #[test]
    fn test_task_v2_comment() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _comment = task_v2.comment();
    }

    #[test]
    fn test_task_v2_section() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _section = task_v2.section();
    }

    #[test]
    fn test_task_v2_attachment() {
        let config = create_test_config();
        let task_v2 = TaskV2::new(config);
        let _attachment = task_v2.attachment();
    }
}
