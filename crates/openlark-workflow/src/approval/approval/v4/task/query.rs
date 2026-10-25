//! 查询用户的任务列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/approval-search/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 任务相关 URL（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskUrlsV4 {
    /// 帮助服务台 URL
    #[serde(default)]
    pub helpdesk: Option<String>,
    /// 移动端 URL
    #[serde(default)]
    pub mobile: Option<String>,
    /// PC 端 URL
    #[serde(default)]
    pub pc: Option<String>,
}

/// 列表计数（v4，只在分页第一页返回）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskCountV4 {
    /// 总数，大于等于 1000 个项目时将返回 999
    #[serde(default)]
    pub total: Option<i64>,
    /// 还有更多，当大于等于 1000 时将返回 true
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 审批任务列表项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskItemV4 {
    /// 任务 ID，全局唯一
    #[serde(default)]
    pub task_id: Option<String>,
    /// 任务三方 ID，仅第三方流程，需要在当前流程实例内唯一
    #[serde(default)]
    pub task_external_id: Option<String>,
    /// 流程定义 Code（官方字段名 definition_code）
    #[serde(default, rename = "definition_code")]
    pub approval_code: Option<String>,
    /// 流程 Code（官方字段名 process_code）
    #[serde(default, rename = "process_code")]
    pub instance_code: Option<String>,
    /// 流程 ID，全局唯一
    #[serde(default)]
    pub process_id: Option<String>,
    /// 流程三方 ID，仅第三方流程，需要在当前租户、当前 APP 内唯一
    #[serde(default)]
    pub process_external_id: Option<String>,
    /// 任务状态（1：待办 2：已办 17：未读 18：已读 33：处理中 34：撤回）
    pub status: String,
    /// 流程实例状态（0：无状态 1：流转中 2：已通过 3：已拒绝 4：已撤销 5：已终止）
    #[serde(default)]
    pub process_status: Option<String>,
    /// 任务所属的任务分组（1：待办 2：已办 3：已发起 17：未读知会 18：已读知会）
    #[serde(default)]
    pub topic: Option<String>,
    /// 任务题目
    #[serde(default)]
    pub title: Option<String>,
    /// 任务所属的用户 ID
    #[serde(default)]
    pub user_id: Option<String>,
    /// 任务相关 URL
    #[serde(default)]
    pub urls: Option<TaskUrlsV4>,
    /// 发起人 ID 列表
    #[serde(default)]
    pub initiators: Vec<String>,
    /// 发起人姓名列表
    #[serde(default)]
    pub initiator_names: Vec<String>,
    /// 流程定义分组 ID
    #[serde(default)]
    pub definition_group_id: Option<String>,
    /// 流程定义分组名称
    #[serde(default)]
    pub definition_group_name: Option<String>,
    /// 流程定义 ID
    #[serde(default)]
    pub definition_id: Option<String>,
    /// 流程定义名称
    #[serde(default)]
    pub definition_name: Option<String>,
}

/// 查询用户的任务列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct QueryTaskResponseV4 {
    /// 任务列表
    #[serde(default, alias = "items")]
    pub tasks: Vec<TaskItemV4>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 列表计数，只在分页第一页返回
    #[serde(default)]
    pub count: Option<TaskCountV4>,
}

/// 查询用户的任务列表请求（v4）
#[derive(Debug, Clone)]
pub struct QueryTaskRequestV4 {
    config: Arc<Config>,
    user_id: String,
    topic: String,
    user_id_type: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl QueryTaskRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            user_id: String::new(),
            topic: String::new(),
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 设置主题。
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = topic.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryTaskResponseV4> {
        validate_required!(self.user_id.trim(), "user_id 不能为空");
        validate_required!(self.topic.trim(), "topic 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskQuery;
        let mut request = ApiRequest::<QueryTaskResponseV4>::get(api_endpoint.to_url())
            .query("user_id", self.user_id)
            .query("topic", self.topic);

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询审批任务",
        )
        .await
    }
}

impl ApiResponseTrait for QueryTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_query_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskQuery;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks/query");
    }

    #[test]
    fn test_query_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = QueryTaskRequestV4::new(config)
            .user_id("ou_xxx")
            .topic("1")
            .user_id_type("open_id")
            .page_size(100)
            .page_token("token_1");

        assert_eq!(request.user_id, "ou_xxx");
        assert_eq!(request.topic, "1");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token.as_deref(), Some("token_1"));
    }

    /// 官方文档示例：同一 task 内 task_id 与 task_external_id、
    /// process_id/process_code/process_external_id 为并存字段（非别名合并），
    /// 旧实现的 serde alias 会在该形状下报 duplicate field 直接反序列化失败。
    #[test]
    fn test_task_item_official_coexisting_fields() {
        let item: TaskItemV4 = serde_json::from_value(serde_json::json!({
            "task_id": "1212564555454",
            "task_external_id": "example_task_id",
            "process_id": "1214564545474",
            "process_code": "123e4567-e89b-12d3-a456-426655440000",
            "process_external_id": "example_instance_id",
            "definition_code": "000000-00000000000000-0example",
            "status": "Todo",
            "process_status": "Running",
            "topic": "1",
            "title": "任务题目示例",
            "user_id": "example_user_id",
            "urls": {
                "helpdesk": "https://helpdesk.example.com",
                "mobile": "https://mobile.example.com",
                "pc": "https://pc.example.com"
            },
            "initiators": ["starter"],
            "initiator_names": ["发起人姓名"],
            "definition_group_id": "1212564555454",
            "definition_group_name": "流程定义名称",
            "definition_id": "1212564555454",
            "definition_name": "流程定义组名称"
        }))
        .expect("task item with coexisting official fields should deserialize");

        assert_eq!(item.task_id.as_deref(), Some("1212564555454"));
        assert_eq!(item.task_external_id.as_deref(), Some("example_task_id"));
        assert_eq!(item.process_id.as_deref(), Some("1214564545474"));
        assert_eq!(
            item.instance_code.as_deref(),
            Some("123e4567-e89b-12d3-a456-426655440000")
        );
        assert_eq!(
            item.process_external_id.as_deref(),
            Some("example_instance_id")
        );
        assert_eq!(
            item.approval_code.as_deref(),
            Some("000000-00000000000000-0example")
        );
        assert_eq!(item.status, "Todo");
        assert_eq!(item.process_status.as_deref(), Some("Running"));
        assert_eq!(item.topic.as_deref(), Some("1"));
        assert_eq!(item.title.as_deref(), Some("任务题目示例"));
        assert_eq!(item.user_id.as_deref(), Some("example_user_id"));
        assert_eq!(
            item.urls.as_ref().and_then(|urls| urls.pc.as_deref()),
            Some("https://pc.example.com")
        );
        assert_eq!(item.initiators, vec!["starter".to_string()]);
        assert_eq!(item.initiator_names, vec!["发起人姓名".to_string()]);
        assert_eq!(item.definition_group_id.as_deref(), Some("1212564555454"));
        assert_eq!(item.definition_group_name.as_deref(), Some("流程定义名称"));
        assert_eq!(item.definition_id.as_deref(), Some("1212564555454"));
        assert_eq!(item.definition_name.as_deref(), Some("流程定义组名称"));
    }

    /// 仅含三方 ID 的任务（无平台侧 task_id/process_code）也应可反序列化
    #[test]
    fn test_task_item_external_id_only() {
        let item: TaskItemV4 = serde_json::from_value(serde_json::json!({
            "task_external_id": "example_task_id",
            "process_external_id": "example_instance_id",
            "status": "Todo"
        }))
        .expect("external-id-only task item should deserialize");

        assert_eq!(item.task_id, None);
        assert_eq!(item.instance_code, None);
        assert_eq!(item.task_external_id.as_deref(), Some("example_task_id"));
        assert_eq!(
            item.process_external_id.as_deref(),
            Some("example_instance_id")
        );
        assert!(item.initiators.is_empty());
        assert!(item.initiator_names.is_empty());
    }

    #[test]
    fn test_query_task_response_contract() {
        let response: QueryTaskResponseV4 = serde_json::from_value(serde_json::json!({
            "tasks": [
                {
                    "task_id": "1212564555454",
                    "task_external_id": "example_task_id",
                    "process_id": "1214564545474",
                    "process_code": "123e4567-e89b-12d3-a456-426655440000",
                    "process_external_id": "example_instance_id",
                    "definition_code": "000000-00000000000000-0example",
                    "status": "Todo",
                    "process_status": "Running",
                    "topic": "1",
                    "title": "任务题目示例",
                    "user_id": "example_user_id",
                    "urls": {
                        "helpdesk": "https://helpdesk.example.com",
                        "mobile": "https://mobile.example.com",
                        "pc": "https://pc.example.com"
                    },
                    "initiators": ["starter"],
                    "initiator_names": ["发起人姓名"],
                    "definition_group_id": "1212564555454",
                    "definition_group_name": "流程定义名称",
                    "definition_id": "1212564555454",
                    "definition_name": "流程定义组名称"
                }
            ],
            "page_token": "example_page_token",
            "has_more": true,
            "count": {
                "total": 123,
                "has_more": false
            }
        }))
        .expect("query task response should deserialize");

        assert_eq!(response.tasks.len(), 1);
        assert_eq!(response.tasks[0].task_id.as_deref(), Some("1212564555454"));
        assert_eq!(
            response.tasks[0].task_external_id.as_deref(),
            Some("example_task_id")
        );
        assert_eq!(
            response.tasks[0].approval_code.as_deref(),
            Some("000000-00000000000000-0example")
        );
        assert_eq!(
            response.tasks[0].instance_code.as_deref(),
            Some("123e4567-e89b-12d3-a456-426655440000")
        );
        assert_eq!(
            response.tasks[0].user_id.as_deref(),
            Some("example_user_id")
        );
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token.as_deref(), Some("example_page_token"));
        assert_eq!(response.count.as_ref().and_then(|c| c.total), Some(123));
        assert_eq!(
            response.count.as_ref().and_then(|c| c.has_more),
            Some(false)
        );
    }
}
