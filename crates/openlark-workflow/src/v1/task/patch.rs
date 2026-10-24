//! 更新任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/update>

use crate::v1::task::models::{TaskCollaboratorV1, TaskDueV1, TaskFollowerV1, TaskOriginV1};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 被更新的任务实体基础信息（v1，update 请求内嵌 task 对象）
#[derive(Debug, Clone, Serialize, Default)]
pub struct InputTaskV1 {
    /// 任务标题（普通文本）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// 任务描述（普通文本）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 附属信息（base64 编码后的自定义数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,

    /// 任务截止时间设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDueV1>,

    /// 任务关联的第三方平台来源信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaskOriginV1>,

    /// 控制任务在飞书任务中心是否可编辑（官方已废弃，为向前兼容保留，不推荐使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,

    /// 自定义完成配置（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// 任务的关注者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<TaskFollowerV1>>,

    /// 任务的执行者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborators: Option<Vec<TaskCollaboratorV1>>,

    /// 执行者用户 ID 列表（最大 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_ids: Option<Vec<String>>,

    /// 关注者用户 ID 列表（最大 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<String>>,

    /// 重复任务的规则表达式（RRule 语法）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,

    /// 富文本任务标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_summary: Option<String>,

    /// 富文本任务备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_description: Option<String>,
}

/// 更新任务请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
/// 更新任务请求体。
pub struct UpdateTaskBodyV1 {
    /// 被更新的任务实体基础信息
    pub task: InputTaskV1,

    /// 指定需要更新的任务字段；合法值：summary / rich_summary / description /
    /// rich_description / due / extra / custom / follower_ids / collaborator_ids / repeat_rule
    pub update_fields: Vec<String>,
}

/// 任务详情（v1，响应内嵌 task 对象）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TaskDetailV1 {
    /// 任务的唯一 ID
    #[serde(default)]
    pub id: Option<String>,

    /// 任务标题（普通文本）
    #[serde(default)]
    pub summary: Option<String>,

    /// 任务描述（普通文本）
    #[serde(default)]
    pub description: Option<String>,

    /// 任务完成时间戳（单位为秒，0 表示未完成）
    #[serde(default)]
    pub complete_time: Option<String>,

    /// 任务创建者 ID
    #[serde(default)]
    pub creator_id: Option<String>,

    /// 附属信息
    #[serde(default)]
    pub extra: Option<String>,

    /// 任务创建时间的 Unix 时间戳（单位为秒）
    #[serde(default)]
    pub create_time: Option<String>,

    /// 任务更新时间的 Unix 时间戳（单位为秒）
    #[serde(default)]
    pub update_time: Option<String>,

    /// 任务截止时间设置
    #[serde(default)]
    pub due: Option<TaskDueV1>,

    /// 任务关联的第三方平台来源信息
    #[serde(default)]
    pub origin: Option<TaskOriginV1>,

    /// 控制任务在飞书任务中心是否可编辑（官方已废弃）
    #[serde(default)]
    pub can_edit: Option<bool>,

    /// 自定义完成配置
    #[serde(default)]
    pub custom: Option<String>,

    /// 任务创建的来源（0-8）
    #[serde(default)]
    pub source: Option<i32>,

    /// 任务的关注者
    #[serde(default)]
    pub followers: Option<Vec<TaskFollowerV1>>,

    /// 任务的执行者
    #[serde(default)]
    pub collaborators: Option<Vec<TaskCollaboratorV1>>,

    /// 执行者用户 ID 列表
    #[serde(default)]
    pub collaborator_ids: Option<Vec<String>>,

    /// 关注者用户 ID 列表
    #[serde(default)]
    pub follower_ids: Option<Vec<String>>,

    /// 重复任务的规则表达式
    #[serde(default)]
    pub repeat_rule: Option<String>,

    /// 富文本任务标题
    #[serde(default)]
    pub rich_summary: Option<String>,

    /// 富文本任务备注
    #[serde(default)]
    pub rich_description: Option<String>,
}

/// 更新任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 更新任务响应。
pub struct UpdateTaskResponseV1 {
    /// 返回修改后的任务详情
    pub task: TaskDetailV1,
}

/// 更新任务请求（v1）
#[derive(Debug, Clone)]
/// 更新任务请求构建器。
pub struct UpdateTaskRequestV1 {
    config: Arc<Config>,
    task_id: String,
    body: UpdateTaskBodyV1,
}

impl UpdateTaskRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            body: UpdateTaskBodyV1::default(),
        }
    }

    /// 将字段名加入 update_fields（去重）
    fn push_update_field(&mut self, field: &str) {
        if !self.body.update_fields.iter().any(|f| f == field) {
            self.body.update_fields.push(field.to_string());
        }
    }

    /// 设置任务标题（普通文本），并将 summary 加入 update_fields
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.task.summary = Some(summary.into());
        self.push_update_field("summary");
        self
    }

    /// 设置富文本任务标题，并将 rich_summary 加入 update_fields
    pub fn rich_summary(mut self, rich_summary: impl Into<String>) -> Self {
        self.body.task.rich_summary = Some(rich_summary.into());
        self.push_update_field("rich_summary");
        self
    }

    /// 设置任务描述（普通文本），并将 description 加入 update_fields
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.task.description = Some(description.into());
        self.push_update_field("description");
        self
    }

    /// 设置富文本任务备注，并将 rich_description 加入 update_fields
    pub fn rich_description(mut self, rich_description: impl Into<String>) -> Self {
        self.body.task.rich_description = Some(rich_description.into());
        self.push_update_field("rich_description");
        self
    }

    /// 设置任务截止时间，并将 due 加入 update_fields
    pub fn due(mut self, due: TaskDueV1) -> Self {
        self.body.task.due = Some(due);
        self.push_update_field("due");
        self
    }

    /// 设置附属信息，并将 extra 加入 update_fields
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.task.extra = Some(extra.into());
        self.push_update_field("extra");
        self
    }

    /// 设置自定义完成配置，并将 custom 加入 update_fields
    pub fn custom(mut self, custom: impl Into<String>) -> Self {
        self.body.task.custom = Some(custom.into());
        self.push_update_field("custom");
        self
    }

    /// 设置执行者用户 ID 列表，并将 collaborator_ids 加入 update_fields
    pub fn collaborator_ids(mut self, collaborator_ids: Vec<impl Into<String>>) -> Self {
        self.body.task.collaborator_ids =
            Some(collaborator_ids.into_iter().map(|v| v.into()).collect());
        self.push_update_field("collaborator_ids");
        self
    }

    /// 设置关注者用户 ID 列表，并将 follower_ids 加入 update_fields
    pub fn follower_ids(mut self, follower_ids: Vec<impl Into<String>>) -> Self {
        self.body.task.follower_ids = Some(follower_ids.into_iter().map(|v| v.into()).collect());
        self.push_update_field("follower_ids");
        self
    }

    /// 设置重复任务规则，并将 repeat_rule 加入 update_fields
    pub fn repeat_rule(mut self, repeat_rule: impl Into<String>) -> Self {
        self.body.task.repeat_rule = Some(repeat_rule.into());
        self.push_update_field("repeat_rule");
        self
    }

    /// 设置任务第三方来源信息（官方 update_fields 枚举不含 origin，需配合
    /// [`UpdateTaskRequestV1::update_fields`] 显式控制）
    pub fn origin(mut self, origin: TaskOriginV1) -> Self {
        self.body.task.origin = Some(origin);
        self
    }

    /// 设置任务在飞书任务中心是否可编辑（官方已废弃；update_fields 枚举不含 can_edit）
    pub fn can_edit(mut self, can_edit: bool) -> Self {
        self.body.task.can_edit = Some(can_edit);
        self
    }

    /// 设置任务的关注者（官方 update_fields 枚举不含 followers，需配合
    /// [`UpdateTaskRequestV1::update_fields`] 显式控制）
    pub fn followers(mut self, followers: Vec<TaskFollowerV1>) -> Self {
        self.body.task.followers = Some(followers);
        self
    }

    /// 设置任务的执行者（官方 update_fields 枚举不含 collaborators，需配合
    /// [`UpdateTaskRequestV1::update_fields`] 显式控制）
    pub fn collaborators(mut self, collaborators: Vec<TaskCollaboratorV1>) -> Self {
        self.body.task.collaborators = Some(collaborators);
        self
    }

    /// 显式设置要更新的字段名列表（覆盖 Builder 自动收集的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTaskResponseV1> {
        validate_required!(self.body.update_fields, "update_fields 不能为空");

        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskUpdate(self.task_id);
        let mut request = ApiRequest::<UpdateTaskResponseV1>::patch(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateTaskResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_update_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateTaskRequestV1::new(config, "test_task_id")
            .summary("更新后的任务标题")
            .description("更新后的描述");

        assert_eq!(request.task_id, "test_task_id");
        assert_eq!(
            request.body.task.summary,
            Some("更新后的任务标题".to_string())
        );
        assert_eq!(
            request.body.task.description,
            Some("更新后的描述".to_string())
        );
        assert_eq!(request.body.update_fields, vec!["summary", "description"]);
    }

    #[test]
    fn test_update_task_v1_body_serialization() {
        let body = UpdateTaskBodyV1 {
            task: InputTaskV1 {
                summary: Some("每天喝八杯水，保持身心愉悦".to_string()),
                description: Some("多吃水果，多运动，健康生活，快乐工作。".to_string()),
                extra: Some("dGVzdA==".to_string()),
                due: Some(TaskDueV1 {
                    time: Some("1623124318".to_string()),
                    timezone: Some("Asia/Shanghai".to_string()),
                    is_all_day: Some(false),
                }),
                ..Default::default()
            },
            update_fields: vec!["summary".to_string()],
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "task": {
                    "summary": "每天喝八杯水，保持身心愉悦",
                    "description": "多吃水果，多运动，健康生活，快乐工作。",
                    "extra": "dGVzdA==",
                    "due": {"time": "1623124318", "timezone": "Asia/Shanghai", "is_all_day": false}
                },
                "update_fields": ["summary"]
            })
        );
    }

    #[test]
    fn test_task_api_v1_update_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskUpdate("task_123".to_string());
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks/task_123");
    }
}
