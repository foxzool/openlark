//! 更新任务（PATCH）
//!
//! docPath: <https://open.feishu.cn/document/task-v2/task/patch>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::{
    InputCustomFieldValue, TaskTimePoint, UpdateTaskBody, UpdateTaskResponse,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde_json::Value;
use std::sync::Arc;

/// 更新任务请求
#[derive(Debug, Clone)]
pub struct UpdateTaskRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: UpdateTaskBody,
    /// 用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    user_id_type: Option<String>,
}

impl UpdateTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: String) -> Self {
        Self {
            config,
            task_guid,
            body: UpdateTaskBody::default(),
            user_id_type: None,
        }
    }

    /// 将字段名加入 update_fields（去重）
    fn push_update_field(&mut self, field: &str) {
        if !self.body.update_fields.iter().any(|f| f == field) {
            self.body.update_fields.push(field.to_string());
        }
    }

    /// 设置任务标题，并将 summary 加入 update_fields
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.task.summary = Some(summary.into());
        self.push_update_field("summary");
        self
    }

    /// 设置任务描述，并将 description 加入 update_fields
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.task.description = Some(description.into());
        self.push_update_field("description");
        self
    }

    /// 设置任务开始时间，并将 start 加入 update_fields
    pub fn start(mut self, start: TaskTimePoint) -> Self {
        self.body.task.start = Some(start);
        self.push_update_field("start");
        self
    }

    /// 设置任务截止时间，并将 due 加入 update_fields
    pub fn due(mut self, due: TaskTimePoint) -> Self {
        self.body.task.due = Some(due);
        self.push_update_field("due");
        self
    }

    /// 设置任务的完成时刻，并将 completed_at 加入 update_fields
    pub fn completed_at(mut self, completed_at: impl Into<String>) -> Self {
        self.body.task.completed_at = Some(completed_at.into());
        self.push_update_field("completed_at");
        self
    }

    /// 设置附属自定义数据，并将 extra 加入 update_fields
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.task.extra = Some(extra.into());
        self.push_update_field("extra");
        self
    }

    /// 设置重复任务规则，并将 repeat_rule 加入 update_fields
    pub fn repeat_rule(mut self, repeat_rule: impl Into<String>) -> Self {
        self.body.task.repeat_rule = Some(repeat_rule.into());
        self.push_update_field("repeat_rule");
        self
    }

    /// 设置任务自定义完成配置，并将 custom_complete 加入 update_fields
    pub fn custom_complete(mut self, custom_complete: Value) -> Self {
        self.body.task.custom_complete = Some(custom_complete);
        self.push_update_field("custom_complete");
        self
    }

    /// 设置任务的完成模式（1-会签；2-或签），并将 mode 加入 update_fields
    pub fn mode(mut self, mode: i32) -> Self {
        self.body.task.mode = Some(mode);
        self.push_update_field("mode");
        self
    }

    /// 设置是否里程碑任务，并将 is_milestone 加入 update_fields
    pub fn is_milestone(mut self, is_milestone: bool) -> Self {
        self.body.task.is_milestone = Some(is_milestone);
        self.push_update_field("is_milestone");
        self
    }

    /// 设置自定义字段值，并将 custom_fields 加入 update_fields
    pub fn custom_fields(mut self, custom_fields: Vec<InputCustomFieldValue>) -> Self {
        self.body.task.custom_fields = Some(custom_fields);
        self.push_update_field("custom_fields");
        self
    }

    /// 显式设置要更新的字段名列表（覆盖 Builder 自动收集的值）
    pub fn update_fields(mut self, update_fields: Vec<String>) -> Self {
        self.body.update_fields = update_fields;
        self
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTaskResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.update_fields, "update_fields 不能为空");

        let api_endpoint = TaskApiV2::TaskUpdate(self.task_guid.clone());
        let mut request = ApiRequest::<UpdateTaskResponse>::patch(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新任务")?);
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "更新任务",
        )
        .await
    }
}

impl ApiResponseTrait for UpdateTaskResponse {
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
    fn test_update_task_builder_collects_update_fields() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateTaskRequest::new(config, "task_123".to_string())
            .summary("更新后的标题")
            .description("更新后的描述")
            .user_id_type("open_id");

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.task.summary, Some("更新后的标题".to_string()));
        assert_eq!(
            request.body.task.description,
            Some("更新后的描述".to_string())
        );
        assert_eq!(request.body.update_fields, vec!["summary", "description"]);
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    #[test]
    fn test_update_task_body_serialization() {
        let body = UpdateTaskBody {
            task: crate::v2::task::models::InputTask {
                summary: Some("针对全年销售进行一次复盘".to_string()),
                due: Some(TaskTimePoint {
                    timestamp: Some("1675454764000".to_string()),
                    is_all_day: Some(true),
                }),
                ..Default::default()
            },
            update_fields: vec!["summary".to_string(), "due".to_string()],
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "task": {
                    "summary": "针对全年销售进行一次复盘",
                    "due": {"timestamp": "1675454764000", "is_all_day": true}
                },
                "update_fields": ["summary", "due"]
            })
        );
    }
}
