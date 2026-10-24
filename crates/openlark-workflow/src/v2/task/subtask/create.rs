//! 创建子任务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-subtask/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::{TaskInTasklistInfo, TaskMember, TaskTimePoint};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// 创建子任务请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateSubtaskBody {
    /// 子任务标题
    pub summary: String,

    /// 子任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 子任务开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskTimePoint>,

    /// 子任务截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskTimePoint>,

    /// 子任务完成时间（毫秒时间戳字符串，最长 20 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,

    /// 幂等 token（长度 10~100 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,

    /// 调用方附加数据（最长 65536 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,

    /// 任务关联的第三方平台来源信息（origin）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<Value>,

    /// 任务成员列表（member[]）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskMember>>,

    /// 重复规则（RRULE，最长 1000 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,

    /// 自定义完成配置（custom_complete）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<Value>,

    /// 任务加入的清单信息（task_in_tasklist_info[]）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasklists: Option<Vec<TaskInTasklistInfo>>,

    /// 任务提醒（reminder[]）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Value>>,

    /// 任务完成模式：1 会签、2 或签（取值 1~2，默认 2）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,

    /// 是否里程碑（默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_milestone: Option<bool>,

    /// 自定义字段值（input_custom_field_value[]）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<Value>>,

    /// 任务关联的云文档来源（docx_source）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docx_source: Option<Value>,
}

/// 创建子任务响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSubtaskResponse {
    /// 子任务 GUID
    pub task_guid: String,

    /// 子任务标题
    pub summary: String,

    /// 子任务描述
    #[serde(default)]
    pub description: Option<String>,

    /// 子任务状态
    pub status: String,

    /// 父任务 GUID
    pub parent_task_guid: String,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 创建子任务请求
#[derive(Debug, Clone)]
pub struct CreateSubtaskRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 父任务 GUID
    task_guid: String,
    /// 请求体
    body: CreateSubtaskBody,
}

impl CreateSubtaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: CreateSubtaskBody::default(),
        }
    }

    /// 设置子任务标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
        self
    }

    /// 设置子任务描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置子任务开始时间
    pub fn start(mut self, start: TaskTimePoint) -> Self {
        self.body.start = Some(start);
        self
    }

    /// 设置子任务截止时间
    pub fn due(mut self, due: TaskTimePoint) -> Self {
        self.body.due = Some(due);
        self
    }

    /// 设置子任务完成时间（毫秒时间戳字符串）
    pub fn completed_at(mut self, completed_at: impl Into<String>) -> Self {
        self.body.completed_at = Some(completed_at.into());
        self
    }

    /// 设置幂等 token
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.body.client_token = Some(client_token.into());
        self
    }

    /// 设置调用方附加数据
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 设置第三方平台来源信息（origin 对象）
    pub fn origin(mut self, origin: Value) -> Self {
        self.body.origin = Some(origin);
        self
    }

    /// 设置任务成员列表
    pub fn members(mut self, members: Vec<TaskMember>) -> Self {
        self.body.members = Some(members);
        self
    }

    /// 设置重复规则
    pub fn repeat_rule(mut self, repeat_rule: impl Into<String>) -> Self {
        self.body.repeat_rule = Some(repeat_rule.into());
        self
    }

    /// 设置自定义完成配置（custom_complete 对象）
    pub fn custom_complete(mut self, custom_complete: Value) -> Self {
        self.body.custom_complete = Some(custom_complete);
        self
    }

    /// 设置任务加入的清单信息
    pub fn tasklists(mut self, tasklists: Vec<TaskInTasklistInfo>) -> Self {
        self.body.tasklists = Some(tasklists);
        self
    }

    /// 设置任务提醒
    pub fn reminders(mut self, reminders: Vec<Value>) -> Self {
        self.body.reminders = Some(reminders);
        self
    }

    /// 设置任务完成模式：1 会签、2 或签
    pub fn mode(mut self, mode: i32) -> Self {
        self.body.mode = Some(mode);
        self
    }

    /// 设置是否里程碑
    pub fn is_milestone(mut self, is_milestone: bool) -> Self {
        self.body.is_milestone = Some(is_milestone);
        self
    }

    /// 设置自定义字段值
    pub fn custom_fields(mut self, custom_fields: Vec<Value>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 设置云文档来源（docx_source 对象）
    pub fn docx_source(mut self, docx_source: Value) -> Self {
        self.body.docx_source = Some(docx_source);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateSubtaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateSubtaskResponse> {
        // 验证必填字段
        validate_required!(self.body.summary.trim(), "子任务标题不能为空");
        validate_required!(self.task_guid.trim(), "父任务 GUID 不能为空");

        let api_endpoint = TaskApiV2::SubtaskCreate(self.task_guid.clone());
        let mut request = ApiRequest::<CreateSubtaskResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建子任务")?);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建子任务",
        )
        .await
    }
}

impl ApiResponseTrait for CreateSubtaskResponse {
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
    fn test_create_subtask_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateSubtaskRequest::new(config, "parent_task_123")
            .summary("测试子任务")
            .description("子任务描述")
            .due(TaskTimePoint {
                timestamp: Some("1675742789470".to_string()),
                is_all_day: Some(false),
            })
            .members(vec![TaskMember {
                id: "ou_user_123".to_string(),
                r#type: Some("user".to_string()),
                role: Some("assignee".to_string()),
                name: None,
            }])
            .client_token("daa2237f-8310-4707-a83b-52c8a81e0fb7");

        assert_eq!(request.task_guid, "parent_task_123");
        assert_eq!(request.body.summary, "测试子任务");
        assert_eq!(request.body.description, Some("子任务描述".to_string()));
        assert_eq!(
            request
                .body
                .due
                .as_ref()
                .and_then(|d| d.timestamp.as_deref()),
            Some("1675742789470")
        );
        assert_eq!(request.body.members.as_ref().map(|m| m.len()), Some(1));
        assert_eq!(
            request.body.client_token.as_deref(),
            Some("daa2237f-8310-4707-a83b-52c8a81e0fb7")
        );
    }

    #[test]
    fn test_create_subtask_body_official_shape() {
        // 官方请求体 17 个顶层字段：不设置的可选字段不应序列化
        let body = CreateSubtaskBody {
            summary: "进行销售年中总结".to_string(),
            description: Some("提前阅读总结文档".to_string()),
            ..CreateSubtaskBody::default()
        };
        let json = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(
            json,
            serde_json::json!({
                "summary": "进行销售年中总结",
                "description": "提前阅读总结文档"
            })
        );
    }

    #[test]
    fn test_subtask_api_v2_url() {
        let endpoint = TaskApiV2::SubtaskCreate("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/subtasks"
        );
    }
}
