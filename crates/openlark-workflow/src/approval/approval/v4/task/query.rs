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

use crate::common::api_utils::missing_response_data_error;

/// 审批任务列表项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskItemV4 {
    /// 任务 ID / 任务三方 ID
    #[serde(alias = "task_external_id")]
    pub task_id: String,
    /// 审批定义 Code
    #[serde(alias = "definition_code")]
    pub approval_code: String,
    /// 审批实例 Code / 流程 Code / 流程三方 ID
    #[serde(alias = "process_code", alias = "process_external_id")]
    pub instance_code: String,
    /// 任务状态
    pub status: String,
    /// 审批标题
    #[serde(default)]
    pub title: Option<String>,
    /// 用户 ID
    #[serde(default)]
    pub user_id: Option<String>,
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
    /// 待补充文档。
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

    /// 待补充文档。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 待补充文档。
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = topic.into();
        self
    }

    /// 待补充文档。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 待补充文档。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 待补充文档。
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

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error("查询审批任务", response.raw_response.request_id.clone())
        })
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

    #[test]
    fn test_task_item_response_aliases() {
        let item: TaskItemV4 = serde_json::from_value(serde_json::json!({
            "task_external_id": "task_ext_123",
            "definition_code": "approval_code_456",
            "process_external_id": "instance_ext_789",
            "status": "Todo",
            "title": "请假审批"
        }))
        .expect("task item should deserialize");

        assert_eq!(item.task_id, "task_ext_123");
        assert_eq!(item.approval_code, "approval_code_456");
        assert_eq!(item.instance_code, "instance_ext_789");
        assert_eq!(item.title.as_deref(), Some("请假审批"));
    }

    #[test]
    fn test_query_task_response_contract() {
        let response: QueryTaskResponseV4 = serde_json::from_value(serde_json::json!({
            "tasks": [
                {
                    "task_external_id": "task_ext_123",
                    "definition_code": "approval_code_456",
                    "process_external_id": "instance_ext_789",
                    "status": "Todo",
                    "title": "请假审批",
                    "user_id": "ou_xxx"
                }
            ],
            "has_more": true,
            "page_token": "next_page"
        }))
        .expect("query task response should deserialize");

        assert_eq!(response.tasks.len(), 1);
        assert_eq!(response.tasks[0].task_id, "task_ext_123");
        assert_eq!(response.tasks[0].approval_code, "approval_code_456");
        assert_eq!(response.tasks[0].instance_code, "instance_ext_789");
        assert_eq!(response.tasks[0].user_id.as_deref(), Some("ou_xxx"));
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token.as_deref(), Some("next_page"));
    }
}
