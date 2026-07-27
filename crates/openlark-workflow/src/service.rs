// approval v4 用户级接口（用户态，需 user_access_token）

// approval v4 用户级接口的公开类型重新导出
// 这些 Request/Body/Response 类型供用户直接 new() + builder + execute() 使用
// （用户态接口需 user_access_token，不适合封装成 service helper）
pub use crate::approval::approval::v4::instance::add_cc::{
    AddCcInstanceBodyV4, AddCcInstanceRequestV4, AddCcInstanceResponseV4,
};
pub use crate::approval::approval::v4::instance::detail::{
    DetailInstanceRequestV4, DetailInstanceResponseV4, DetailInstanceTaskV4,
};
pub use crate::approval::approval::v4::instance::initiated::{
    InitiatedInstanceItemV4, InitiatedInstanceRequestV4, InitiatedInstanceResponseV4,
    InstanceSummaryV4,
};
pub use crate::approval::approval::v4::instance::recall::{
    RecallInstanceBodyV4, RecallInstanceRequestV4, RecallInstanceResponseV4,
};
pub use crate::approval::approval::v4::instance::remind::{
    RemindInstanceBodyV4, RemindInstanceRequestV4, RemindInstanceResponseV4,
};
pub use crate::approval::approval::v4::task::add_sign::{
    AddSignTaskBodyV4, AddSignTaskRequestV4, AddSignTaskResponseV4,
};
pub use crate::approval::approval::v4::task::forward::{
    ForwardTaskBodyV4, ForwardTaskRequestV4, ForwardTaskResponseV4,
};
pub use crate::approval::approval::v4::task::list::{
    ListTaskItemV4, ListTaskRequestV4, ListTaskResponseV4, TaskSummaryV4,
};
pub use crate::approval::approval::v4::task::pass::{
    PassTaskBodyV4, PassTaskRequestV4, PassTaskResponseV4,
};
pub use crate::approval::approval::v4::task::refuse::{
    RefuseTaskBodyV4, RefuseTaskRequestV4, RefuseTaskResponseV4,
};
pub use crate::approval::approval::v4::task::rollback::{
    RollbackTaskBodyV4, RollbackTaskRequestV4, RollbackTaskResponseV4,
};

use openlark_core::{SDKResult, config::Config};
use std::sync::Arc;

use crate::common::constants::MAX_PAGE_SIZE;

/// 任务列表查询 helper。
///
/// 用于封装常见的任务列表过滤条件，并让 helper 统一处理分页。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkflowTaskListQuery {
    /// 任务清单 GUID。
    pub tasklist_guid: Option<String>,
    /// 分组 GUID。
    pub section_guid: Option<String>,
    /// 过滤条件。
    pub filter: Option<String>,
    /// 排序条件。
    pub sort: Option<serde_json::Value>,
    /// 用户 ID 类型。
    pub user_type: Option<String>,
    /// 分页大小。
    pub page_size: Option<i32>,
}

impl WorkflowTaskListQuery {
    /// 为指定任务清单创建查询条件。
    pub fn for_tasklist(tasklist_guid: impl Into<String>) -> Self {
        Self {
            tasklist_guid: Some(tasklist_guid.into()),
            ..Self::default()
        }
    }

    /// 设置分组 GUID。
    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.section_guid = Some(section_guid.into());
        self
    }

    /// 设置过滤条件。
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 设置排序条件。
    pub fn sort(mut self, sort: serde_json::Value) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_type(mut self, user_type: impl Into<String>) -> Self {
        self.user_type = Some(user_type.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 任务创建 helper。
///
/// 只覆盖高频创建字段（标题、描述、截止、优先级、执行者、所属清单等），
/// 不试图替代完整 typed `CreateTaskRequest`（自定义字段 / 子任务 / 重复规则等仍走 typed API）。
#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowTaskCreate {
    /// 任务标题（必填）。
    pub summary: String,
    /// 任务描述。
    pub description: Option<String>,
    /// 开始时间。
    pub start: Option<String>,
    /// 截止时间。
    pub due: Option<String>,
    /// 优先级。
    pub priority: Option<i32>,
    /// 执行者。
    pub assignee: Option<String>,
    /// 任务清单 GUID。
    pub tasklist_guid: Option<String>,
    /// 分组 GUID。
    pub section_guid: Option<String>,
    /// 关注者。
    pub followers: Option<Vec<String>>,
    /// 提醒时间。
    pub remind_time: Option<String>,
}

impl WorkflowTaskCreate {
    /// 以必填标题创建任务描述。
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            description: None,
            start: None,
            due: None,
            priority: None,
            assignee: None,
            tasklist_guid: None,
            section_guid: None,
            followers: None,
            remind_time: None,
        }
    }

    /// 设置任务描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置开始时间。
    pub fn start(mut self, start: impl Into<String>) -> Self {
        self.start = Some(start.into());
        self
    }

    /// 设置截止时间。
    pub fn due(mut self, due: impl Into<String>) -> Self {
        self.due = Some(due.into());
        self
    }

    /// 设置优先级。
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    /// 设置执行者。
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// 设置任务清单 GUID。
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.tasklist_guid = Some(tasklist_guid.into());
        self
    }

    /// 设置分组 GUID。
    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.section_guid = Some(section_guid.into());
        self
    }

    /// 设置关注者列表。
    pub fn followers(mut self, followers: Vec<String>) -> Self {
        self.followers = Some(followers);
        self
    }

    /// 设置提醒时间。
    pub fn remind_time(mut self, remind_time: impl Into<String>) -> Self {
        self.remind_time = Some(remind_time.into());
        self
    }
}

/// 任务变更 helper。
///
/// 只覆盖高频可变字段，不试图替代完整 typed request。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorkflowTaskMutation {
    /// 任务标题。
    pub summary: Option<String>,
    /// 任务描述。
    pub description: Option<String>,
    /// 截止时间。
    pub due: Option<String>,
    /// 优先级。
    pub priority: Option<i32>,
    /// 执行者。
    pub assignee: Option<String>,
    /// 状态。
    pub status: Option<String>,
}

impl WorkflowTaskMutation {
    /// 创建空的任务变更描述。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置任务标题。
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// 设置任务描述。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置截止时间。
    pub fn due(mut self, due: impl Into<String>) -> Self {
        self.due = Some(due.into());
        self
    }

    /// 设置优先级。
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = Some(priority);
        self
    }

    /// 设置执行者。
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    /// 设置审批状态。
    /// 设置任务状态。
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// 审批任务查询 helper。
///
/// 用于封装审批待办的常见筛选条件。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApprovalTaskQuery {
    /// 用户 ID。
    pub user_id: String,
    /// 审批主题。
    pub topic: String,
    /// 用户 ID 类型。
    pub user_id_type: Option<String>,
    /// 审批状态。
    pub status: Option<String>,
    /// 实例编码。
    pub instance_code: Option<String>,
    /// 分页大小。
    pub page_size: Option<i32>,
}

impl ApprovalTaskQuery {
    /// 创建审批任务查询条件。
    pub fn new(user_id: impl Into<String>, topic: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            topic: topic.into(),
            ..Self::default()
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置审批状态。
    /// 设置任务状态。
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// 设置实例编码。
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.instance_code = Some(instance_code.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

/// 审批任务操作 helper。
///
/// 统一高频审批动作的 `task_id + comment` 组合。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ApprovalTaskAction {
    /// 审批定义编码。
    pub approval_code: String,
    /// 审批实例编码。
    pub instance_code: String,
    /// 操作人用户 ID。
    pub user_id: String,
    /// 审批任务 ID。
    pub task_id: String,
    /// 用户 ID 类型。
    pub user_id_type: Option<String>,
    /// 备注。
    pub comment: Option<String>,
    /// 表单内容。
    pub form: Option<String>,
}

impl ApprovalTaskAction {
    /// 创建审批任务动作参数。
    pub fn new(
        approval_code: impl Into<String>,
        instance_code: impl Into<String>,
        user_id: impl Into<String>,
        task_id: impl Into<String>,
    ) -> Self {
        Self {
            approval_code: approval_code.into(),
            instance_code: instance_code.into(),
            user_id: user_id.into(),
            task_id: task_id.into(),
            user_id_type: None,
            comment: None,
            form: None,
        }
    }

    /// 设置备注。
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置表单内容。
    pub fn form(mut self, form: impl Into<String>) -> Self {
        self.form = Some(form.into());
        self
    }
}

/// 审批任务条目类型别名。
pub type ApprovalTaskItem = crate::approval::approval::v4::task::query::TaskItemV4;

/// WorkflowService：工作流服务的统一入口
///
/// 提供对任务、审批、看板 API 的访问能力
#[derive(Clone)]
pub struct WorkflowService {
    config: Arc<Config>,
}

impl WorkflowService {
    /// 创建工作流服务入口。
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    #[cfg(feature = "v1")]
    /// 返回 v1 任务服务入口。
    pub fn v1(&self) -> crate::v1::TaskV1 {
        crate::v1::TaskV1::new(self.config.clone())
    }

    #[cfg(feature = "v2")]
    /// 返回 v2 任务服务入口。
    pub fn v2(&self) -> crate::v2::TaskV2 {
        crate::v2::TaskV2::new(self.config.clone())
    }

    /// 列取任务并自动处理分页。
    #[cfg(feature = "v2")]
    pub async fn list_tasks_all(
        &self,
        query: WorkflowTaskListQuery,
    ) -> SDKResult<Vec<crate::v2::task::models::TaskItem>> {
        use crate::v2::task::list::ListTasksRequest;

        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut request = ListTasksRequest::new(self.config.clone())
                .page_size(query.page_size.unwrap_or(MAX_PAGE_SIZE));

            if let Some(tasklist_guid) = &query.tasklist_guid {
                request = request.tasklist_guid(tasklist_guid.clone());
            }
            if let Some(section_guid) = &query.section_guid {
                request = request.section_guid(section_guid.clone());
            }
            if let Some(filter) = &query.filter {
                request = request.filter(filter.clone());
            }
            if let Some(sort) = &query.sort {
                request = request.sort(sort.clone());
            }
            if let Some(user_type) = &query.user_type {
                request = request.user_type(user_type.clone());
            }
            if let Some(token) = &page_token {
                request = request.page_token(token.clone());
            }

            let response = request.execute().await?;
            items.extend(response.items);

            if !response.has_more {
                break;
            }
            page_token = response.page_token;
        }

        Ok(items)
    }

    /// 使用 helper 风格创建任务（高频字段）。
    ///
    /// 在 typed `CreateTaskRequest` 之上固化常见创建动作，返回业务结果
    /// `CreateTaskResponse`，而不是底层响应壳。
    #[cfg(feature = "v2")]
    pub async fn create_task(
        &self,
        create: WorkflowTaskCreate,
    ) -> SDKResult<crate::v2::task::models::CreateTaskResponse> {
        use crate::v2::task::create::CreateTaskRequest;

        let mut request = CreateTaskRequest::new(self.config.clone()).summary(create.summary);
        if let Some(description) = create.description {
            request = request.description(description);
        }
        if let Some(start) = create.start {
            request = request.start(start);
        }
        if let Some(due) = create.due {
            request = request.due(due);
        }
        if let Some(priority) = create.priority {
            request = request.priority(priority);
        }
        if let Some(assignee) = create.assignee {
            request = request.assignee(assignee);
        }
        if let Some(tasklist_guid) = create.tasklist_guid {
            request = request.tasklist_guid(tasklist_guid);
        }
        if let Some(section_guid) = create.section_guid {
            request = request.section_guid(section_guid);
        }
        if let Some(followers) = create.followers {
            request = request.followers(followers);
        }
        if let Some(remind_time) = create.remind_time {
            request = request.remind_time(remind_time);
        }

        request.execute().await
    }

    /// 使用 helper 风格更新任务高频字段。
    #[cfg(feature = "v2")]
    pub async fn mutate_task(
        &self,
        task_guid: impl Into<String>,
        mutation: WorkflowTaskMutation,
    ) -> SDKResult<crate::v2::task::models::UpdateTaskResponse> {
        use crate::v2::task::update::UpdateTaskRequest;

        let mut request = UpdateTaskRequest::new(self.config.clone(), task_guid.into());
        if let Some(summary) = mutation.summary {
            request = request.summary(summary);
        }
        if let Some(description) = mutation.description {
            request = request.description(description);
        }
        if let Some(due) = mutation.due {
            request = request.due(due);
        }
        if let Some(priority) = mutation.priority {
            request = request.priority(priority);
        }
        if let Some(assignee) = mutation.assignee {
            request = request.assignee(assignee);
        }
        if let Some(status) = mutation.status {
            request = request.status(status);
        }

        request.execute().await
    }

    /// 完成任务 helper。
    #[cfg(feature = "v2")]
    pub async fn complete_task(
        &self,
        task_guid: impl Into<String>,
    ) -> SDKResult<crate::v2::task::models::CompleteTaskResponse> {
        use crate::v2::task::complete::CompleteTaskRequest;

        CompleteTaskRequest::new(self.config.clone(), task_guid.into())
            .execute()
            .await
    }

    /// 重新打开任务 helper。
    #[cfg(feature = "v2")]
    pub async fn reopen_task(
        &self,
        task_guid: impl Into<String>,
    ) -> SDKResult<crate::v2::task::models::UncompleteTaskResponse> {
        use crate::v2::task::uncomplete::UncompleteTaskRequest;

        UncompleteTaskRequest::new(self.config.clone(), task_guid.into())
            .execute()
            .await
    }

    /// 查询审批任务，并支持按状态/实例做本地筛选。
    pub async fn query_approval_tasks(
        &self,
        query: ApprovalTaskQuery,
    ) -> SDKResult<Vec<ApprovalTaskItem>> {
        let mut items = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut request = crate::approval::approval::v4::task::query::QueryTaskRequestV4::new(
                self.config.clone(),
            )
            .user_id(query.user_id.clone())
            .topic(query.topic.clone())
            .page_size(query.page_size.unwrap_or(MAX_PAGE_SIZE));

            if let Some(user_id_type) = &query.user_id_type {
                request = request.user_id_type(user_id_type.clone());
            }
            if let Some(token) = &page_token {
                request = request.page_token(token.clone());
            }

            let response = request.execute().await?;
            items.extend(response.tasks);

            if !response.has_more.unwrap_or(false) {
                break;
            }
            page_token = response.page_token;
        }

        if let Some(status) = &query.status {
            items.retain(|item| item.status == *status);
        }
        if let Some(instance_code) = &query.instance_code {
            items.retain(|item| item.instance_code == *instance_code);
        }

        Ok(items)
    }

    /// 同意审批任务 helper。
    ///
    /// 成功/失败由 `SDKResult` 表达：飞书 approval v4 同意接口响应 data 为空，
    /// 不再伪造恒为 `true` 的 `success` 字段（#350 P9 接口形状撒谎修正）。
    pub async fn approve_task(&self, action: ApprovalTaskAction) -> SDKResult<()> {
        let mut request = crate::approval::approval::v4::task::approve::ApproveTaskRequestV4::new(
            self.config.clone(),
        )
        .approval_code(action.approval_code)
        .instance_code(action.instance_code)
        .user_id(action.user_id)
        .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        request.execute().await?;
        Ok(())
    }

    /// 拒绝审批任务 helper。
    ///
    /// 成功/失败由 `SDKResult` 表达；响应 data 为空时不伪造 `success: true`。
    pub async fn reject_task(&self, action: ApprovalTaskAction) -> SDKResult<()> {
        let mut request = crate::approval::approval::v4::task::reject::RejectTaskRequestV4::new(
            self.config.clone(),
        )
        .approval_code(action.approval_code)
        .instance_code(action.instance_code)
        .user_id(action.user_id)
        .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        request.execute().await?;
        Ok(())
    }

    /// 重新提交审批任务 helper。
    ///
    /// 成功/失败由 `SDKResult` 表达；响应 data 为空时不伪造 `success: true`。
    pub async fn resubmit_task(&self, action: ApprovalTaskAction) -> SDKResult<()> {
        let mut request =
            crate::approval::approval::v4::task::resubmit::ResubmitTaskRequestV4::new(
                self.config.clone(),
            )
            .approval_code(action.approval_code)
            .instance_code(action.instance_code)
            .user_id(action.user_id)
            .task_id(action.task_id);
        if let Some(user_id_type) = action.user_id_type {
            request = request.user_id_type(user_id_type);
        }
        if let Some(comment) = action.comment {
            request = request.comment(comment);
        }
        if let Some(form) = action.form {
            request = request.form(form);
        }
        request.execute().await?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_task_list_query_builder() {
        let query = WorkflowTaskListQuery::for_tasklist("tasklist_123")
            .section_guid("section_456")
            .filter("status = incomplete")
            .sort(json!([{"field": "due", "order": "asc"}]))
            .user_type("open_id")
            .page_size(50);

        assert_eq!(query.tasklist_guid.as_deref(), Some("tasklist_123"));
        assert_eq!(query.section_guid.as_deref(), Some("section_456"));
        assert_eq!(query.filter.as_deref(), Some("status = incomplete"));
        assert_eq!(query.user_type.as_deref(), Some("open_id"));
        assert_eq!(query.page_size, Some(50));
    }

    #[test]
    fn test_task_mutation_builder() {
        let mutation = WorkflowTaskMutation::new()
            .summary("完成项目文档")
            .description("补齐 workflow helper")
            .due("2026-09-30T23:59:59Z")
            .priority(3)
            .assignee("ou_xxx")
            .status("in_progress");

        assert_eq!(mutation.summary.as_deref(), Some("完成项目文档"));
        assert_eq!(
            mutation.description.as_deref(),
            Some("补齐 workflow helper")
        );
        assert_eq!(mutation.due.as_deref(), Some("2026-09-30T23:59:59Z"));
        assert_eq!(mutation.priority, Some(3));
        assert_eq!(mutation.assignee.as_deref(), Some("ou_xxx"));
        assert_eq!(mutation.status.as_deref(), Some("in_progress"));
    }

    #[test]
    fn test_task_create_builder() {
        let create = WorkflowTaskCreate::new("编写 release notes")
            .description("补齐 0.20 create_task helper")
            .due("2026-08-01T18:00:00Z")
            .start("2026-07-27T09:00:00Z")
            .priority(2)
            .assignee("ou_owner")
            .tasklist_guid("tasklist_abc")
            .section_guid("section_xyz")
            .followers(vec!["ou_follower".to_string()])
            .remind_time("2026-07-31T09:00:00Z");

        assert_eq!(create.summary, "编写 release notes");
        assert_eq!(
            create.description.as_deref(),
            Some("补齐 0.20 create_task helper")
        );
        assert_eq!(create.due.as_deref(), Some("2026-08-01T18:00:00Z"));
        assert_eq!(create.start.as_deref(), Some("2026-07-27T09:00:00Z"));
        assert_eq!(create.priority, Some(2));
        assert_eq!(create.assignee.as_deref(), Some("ou_owner"));
        assert_eq!(create.tasklist_guid.as_deref(), Some("tasklist_abc"));
        assert_eq!(create.section_guid.as_deref(), Some("section_xyz"));
        assert_eq!(
            create.followers,
            Some(vec!["ou_follower".to_string()])
        );
        assert_eq!(create.remind_time.as_deref(), Some("2026-07-31T09:00:00Z"));
    }

    #[test]
    fn test_approval_task_query_builder() {
        let query = ApprovalTaskQuery::new("ou_xxx", "1")
            .user_id_type("open_id")
            .status("PENDING")
            .instance_code("instance_123")
            .page_size(100);

        assert_eq!(query.user_id, "ou_xxx");
        assert_eq!(query.topic, "1");
        assert_eq!(query.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(query.status.as_deref(), Some("PENDING"));
        assert_eq!(query.instance_code.as_deref(), Some("instance_123"));
        assert_eq!(query.page_size, Some(100));
    }

    #[test]
    fn test_approval_task_action_builder() {
        let action =
            ApprovalTaskAction::new("approval_code", "instance_code", "ou_xxx", "task_123")
                .user_id_type("open_id")
                .comment("已确认")
                .form("[{}]");

        assert_eq!(action.approval_code, "approval_code");
        assert_eq!(action.instance_code, "instance_code");
        assert_eq!(action.user_id, "ou_xxx");
        assert_eq!(action.task_id, "task_123");
        assert_eq!(action.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(action.comment.as_deref(), Some("已确认"));
        assert_eq!(action.form.as_deref(), Some("[{}]"));
    }

    /// #572：create_task helper 透传高频字段并返回业务 CreateTaskResponse。
    #[cfg(feature = "v2")]
    #[tokio::test]
    async fn test_create_task_helper_posts_high_frequency_fields() {
        use wiremock::matchers::{body_partial_json, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/task/v2/tasks"))
            .and(body_partial_json(json!({
                "summary": "编写 release notes",
                "description": "补齐 0.20 create_task helper",
                "priority": 2,
                "assignee": "ou_owner",
                "tasklist_guid": "tasklist_abc"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "task_guid": "task_created_001",
                    "summary": "编写 release notes",
                    "description": "补齐 0.20 create_task helper",
                    "status": "todo",
                    "tasklist_guid": "tasklist_abc",
                    "section_guid": null,
                    "created_at": "2026-07-27T00:00:00Z",
                    "updated_at": "2026-07-27T00:00:00Z"
                }
            })))
            .mount(&server)
            .await;

        let service = WorkflowService::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let response = service
            .create_task(
                WorkflowTaskCreate::new("编写 release notes")
                    .description("补齐 0.20 create_task helper")
                    .priority(2)
                    .assignee("ou_owner")
                    .tasklist_guid("tasklist_abc"),
            )
            .await
            .expect("create_task 应在飞书成功响应时返回业务结果");

        assert_eq!(response.task_guid, "task_created_001");
        assert_eq!(response.summary, "编写 release notes");
        assert_eq!(response.status, "todo");
        assert_eq!(response.tasklist_guid.as_deref(), Some("tasklist_abc"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/task/v2/tasks");
    }

    /// #350：approve/reject/resubmit 成功时返回 `Ok(())`，不再伪造恒真 `success`。
    #[tokio::test]
    async fn test_approve_reject_resubmit_helpers_return_unit_on_success() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        let paths = [
            "/open-apis/approval/v4/tasks/approve",
            "/open-apis/approval/v4/tasks/reject",
            "/open-apis/approval/v4/tasks/resubmit",
        ];
        for p in paths {
            Mock::given(method("POST"))
                .and(path(p))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "code": 0,
                    "msg": "success",
                    "data": {}
                })))
                .mount(&server)
                .await;
        }

        let service = WorkflowService::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let action =
            ApprovalTaskAction::new("approval_code", "instance_code", "ou_xxx", "task_123")
                .user_id_type("open_id")
                .comment("ok")
                .form("[]");

        service
            .approve_task(action.clone())
            .await
            .expect("approve_task 应在飞书成功响应时返回 Ok(())");
        service
            .reject_task(action.clone())
            .await
            .expect("reject_task 应在飞书成功响应时返回 Ok(())");
        service
            .resubmit_task(action)
            .await
            .expect("resubmit_task 应在飞书成功响应时返回 Ok(())");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(
            received.len(),
            3,
            "三个 helper 应各打一次飞书 approval v4 端点"
        );
        let hit: Vec<_> = received.iter().map(|r| r.url.path().to_string()).collect();
        for p in paths {
            assert!(
                hit.iter().any(|h| h == p),
                "missing request to {p}; got {hit:?}"
            );
        }
    }

    /// #350：底层失败时 helper 传播 Err，而非恒真 success。
    ///
    /// `ApproveTaskRequestV4` 对飞书 `code != 0` 且无 `data` 的响应走
    /// `missing_response_data`（Validation），而不是把 `msg` 映射成 `CoreError::Api`。
    /// 本测试锁定 helper 契约：`Err` 必须向上抛出，不能伪装 `Ok(())`。
    #[tokio::test]
    async fn test_approve_task_helper_propagates_api_error() {
        use openlark_core::error::CoreError;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/approval/v4/tasks/approve"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 99991400,
                "msg": "invalid approval task"
            })))
            .mount(&server)
            .await;

        let service = WorkflowService::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let err = service
            .approve_task(ApprovalTaskAction::new(
                "approval_code",
                "instance_code",
                "ou_xxx",
                "task_123",
            ))
            .await
            .expect_err("飞书失败响应应传播为 Err，不得伪装 Ok(())");
        assert!(
            matches!(err, CoreError::Validation { .. } | CoreError::Api(_)),
            "expected Validation (missing data on non-zero code) or Api, got {err:?}"
        );
        let msg = err.to_string();
        assert!(
            msg.contains("服务器没有返回有效的数据") || msg.contains("invalid approval task"),
            "error should surface leaf validation or Feishu msg, got: {err}"
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/approval/v4/tasks/approve"
        );
    }
}
