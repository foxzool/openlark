//! 创建任务（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/task/create>

use crate::v1::task::models::{TaskDueV1, TaskOriginV1, is_default_origin};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建任务请求体（v1）
#[derive(Debug, Clone, Serialize, Default)]
/// 创建任务请求体。
pub struct CreateTaskBodyV1 {
    /// 任务标题（与 rich_summary 至少填其中一个）
    pub summary: String,

    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 附属信息（base64 编码后的自定义数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,

    /// 任务截止时间设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDueV1>,

    /// 任务关联的第三方平台来源信息（官方文档标记必填；未设置时不序列化）
    #[serde(skip_serializing_if = "is_default_origin")]
    pub origin: TaskOriginV1,

    /// 控制任务在飞书任务中心是否可编辑（官方已废弃，为向前兼容保留，不推荐使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,

    /// 自定义完成配置（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,

    /// 创建任务时添加的执行者用户 ID 列表（最大 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_ids: Option<Vec<String>>,

    /// 创建任务时添加的关注者用户 ID 列表（最大 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_ids: Option<Vec<String>>,

    /// 重复任务的规则表达式（RRule 语法）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,

    /// 富文本任务标题（与 summary 至少填其中一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_summary: Option<String>,

    /// 富文本任务备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_description: Option<String>,
}

/// 创建任务响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 创建任务响应。
pub struct CreateTaskResponseV1 {
    /// 任务 GUID
    pub task_guid: String,

    /// 任务标题
    pub summary: String,
}

/// 创建任务请求（v1）
#[derive(Debug, Clone)]
/// 创建任务请求构建器。
pub struct CreateTaskRequestV1 {
    config: Arc<Config>,
    body: CreateTaskBodyV1,
}

impl CreateTaskRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTaskBodyV1::default(),
        }
    }

    /// 设置任务标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
        self
    }

    /// 设置任务描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置附属信息（base64 编码后的自定义数据）
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 设置任务截止时间
    pub fn due(mut self, due: TaskDueV1) -> Self {
        self.body.due = Some(due);
        self
    }

    /// 设置任务第三方来源信息
    pub fn origin(mut self, origin: TaskOriginV1) -> Self {
        self.body.origin = origin;
        self
    }

    /// 设置任务在飞书任务中心是否可编辑（官方已废弃，不推荐使用）
    pub fn can_edit(mut self, can_edit: bool) -> Self {
        self.body.can_edit = Some(can_edit);
        self
    }

    /// 设置自定义完成配置（JSON 字符串）
    pub fn custom(mut self, custom: impl Into<String>) -> Self {
        self.body.custom = Some(custom.into());
        self
    }

    /// 设置创建任务时添加的执行者用户 ID 列表
    pub fn collaborator_ids(mut self, collaborator_ids: Vec<impl Into<String>>) -> Self {
        self.body.collaborator_ids = Some(collaborator_ids.into_iter().map(|v| v.into()).collect());
        self
    }

    /// 设置创建任务时添加的关注者用户 ID 列表
    pub fn follower_ids(mut self, follower_ids: Vec<impl Into<String>>) -> Self {
        self.body.follower_ids = Some(follower_ids.into_iter().map(|v| v.into()).collect());
        self
    }

    /// 设置重复任务的规则表达式（RRule 语法）
    pub fn repeat_rule(mut self, repeat_rule: impl Into<String>) -> Self {
        self.body.repeat_rule = Some(repeat_rule.into());
        self
    }

    /// 设置富文本任务标题
    pub fn rich_summary(mut self, rich_summary: impl Into<String>) -> Self {
        self.body.rich_summary = Some(rich_summary.into());
        self
    }

    /// 设置富文本任务备注
    pub fn rich_description(mut self, rich_description: impl Into<String>) -> Self {
        self.body.rich_description = Some(rich_description.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTaskResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTaskResponseV1> {
        let summary_blank = self.body.summary.trim().is_empty();
        let rich_summary_blank = self
            .body
            .rich_summary
            .as_deref()
            .is_none_or(|v| v.trim().is_empty());
        if summary_blank && rich_summary_blank {
            return Err(openlark_core::error::CoreError::validation_msg(
                "任务标题 summary 与 rich_summary 不能同时为空，需至少填充其中一个字段",
            ));
        }

        let api_endpoint = crate::common::api_endpoints::TaskApiV1::TaskCreate;
        let mut request = ApiRequest::<CreateTaskResponseV1>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CreateTaskResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::v1::task::models::TaskHrefV1;

    #[test]
    fn test_create_task_v1_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTaskRequestV1::new(config)
            .summary("测试任务")
            .description("任务描述")
            .collaborator_ids(vec!["user_1", "user_2"])
            .follower_ids(vec!["user_3"])
            .rich_summary("富文本标题");

        assert_eq!(request.body.summary, "测试任务");
        assert_eq!(request.body.description, Some("任务描述".to_string()));
        assert_eq!(
            request.body.collaborator_ids,
            Some(vec!["user_1".to_string(), "user_2".to_string()])
        );
        assert_eq!(request.body.follower_ids, Some(vec!["user_3".to_string()]));
        assert_eq!(request.body.rich_summary, Some("富文本标题".to_string()));
    }

    #[test]
    fn test_create_task_v1_body_serialization() {
        let body = CreateTaskBodyV1 {
            summary: "完成本季度OKR编写".to_string(),
            description: Some("对本次会议内容复盘总结".to_string()),
            extra: Some("dGVzdA==".to_string()),
            due: Some(TaskDueV1 {
                time: Some("1623124318".to_string()),
                timezone: Some("Asia/Shanghai".to_string()),
                is_all_day: Some(false),
            }),
            origin: TaskOriginV1::default(),
            can_edit: None,
            custom: None,
            collaborator_ids: Some(vec!["ou_49dadcd6fd55da971d887087c4f3a37a".to_string()]),
            follower_ids: None,
            repeat_rule: Some("FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU,WE,TH,FR".to_string()),
            rich_summary: None,
            rich_description: None,
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "summary": "完成本季度OKR编写",
                "description": "对本次会议内容复盘总结",
                "extra": "dGVzdA==",
                "due": {"time": "1623124318", "timezone": "Asia/Shanghai", "is_all_day": false},
                "collaborator_ids": ["ou_49dadcd6fd55da971d887087c4f3a37a"],
                "repeat_rule": "FREQ=WEEKLY;INTERVAL=1;BYDAY=MO,TU,WE,TH,FR"
            })
        );
    }

    #[test]
    fn test_create_task_v1_origin_serialization() {
        let body = CreateTaskBodyV1 {
            summary: "带来源的任务".to_string(),
            origin: TaskOriginV1 {
                platform_i18n_name: "{\"zh_cn\": \"IT 工作台\", \"en_us\": \"IT Workspace\"}"
                    .to_string(),
                href: Some(TaskHrefV1 {
                    url: Some("https://support.feishu.cn/internal/foo-bar".to_string()),
                    title: Some("反馈一个问题，需要协助排查".to_string()),
                }),
            },
            ..Default::default()
        };
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(
            json.get("origin").unwrap(),
            &serde_json::json!({
                "platform_i18n_name": "{\"zh_cn\": \"IT 工作台\", \"en_us\": \"IT Workspace\"}",
                "href": {
                    "url": "https://support.feishu.cn/internal/foo-bar",
                    "title": "反馈一个问题，需要协助排查"
                }
            })
        );
    }

    #[test]
    fn test_task_api_v1_url() {
        let endpoint = crate::common::api_endpoints::TaskApiV1::TaskCreate;
        assert_eq!(endpoint.to_url(), "/open-apis/task/v1/tasks");
    }
}
