//! 查询审批任务列表（用户级，v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_utils::missing_response_data_error;

/// 审批摘要项（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct TaskSummaryV4 {
    /// 摘要键
    pub key: String,
    /// 摘要值
    pub value: String,
}

/// 审批任务列表项（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskItemV4 {
    /// 任务主题
    pub topic: String,
    /// 任务处理人用户 ID
    pub user_id: String,
    /// 审批标题
    #[serde(default)]
    pub title: Option<String>,
    /// 任务状态
    pub status: String,
    /// 审批实例状态
    #[serde(default)]
    pub instance_status: Option<String>,
    /// 审批定义 Code
    pub definition_code: String,
    /// 发起人用户 ID
    pub initiator: String,
    /// 发起人姓名
    pub initiator_name: String,
    /// 审批任务 ID
    pub task_id: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批定义分组 ID
    #[serde(default)]
    pub definition_group_id: Option<String>,
    /// 审批定义分组名称
    #[serde(default)]
    pub definition_group_name: Option<String>,
    /// 审批定义名称
    pub definition_name: String,
    /// 审批摘要列表
    #[serde(default)]
    pub summaries: Vec<TaskSummaryV4>,
}

/// 查询审批任务列表响应（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListTaskResponseV4 {
    /// 任务列表
    #[serde(default)]
    pub tasks: Vec<ListTaskItemV4>,
    /// 是否有更多数据
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
}

/// 查询审批任务列表请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct ListTaskRequestV4 {
    config: Arc<Config>,
    topic: String,
    definition_code: Option<String>,
    user_id_type: Option<String>,
    locale: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListTaskRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            topic: String::new(),
            definition_code: None,
            user_id_type: None,
            locale: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置任务主题（必填，1=待办 2=已办）
    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = topic.into();
        self
    }

    /// 设置审批定义 Code（按审批类型过滤）
    pub fn definition_code(mut self, definition_code: impl Into<String>) -> Self {
        self.definition_code = Some(definition_code.into());
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskResponseV4> {
        validate_required!(self.topic.trim(), "topic 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskList;
        let mut request =
            ApiRequest::<ListTaskResponseV4>::get(api_endpoint.to_url()).query("topic", self.topic);

        if let Some(definition_code) = self.definition_code {
            request = request.query("definition_code", definition_code);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(locale) = self.locale {
            request = request.query("locale", locale);
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
            missing_response_data_error(
                "查询审批任务列表（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for ListTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_task_list_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::TaskList;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/tasks");
    }

    #[test]
    fn test_list_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = ListTaskRequestV4::new(config)
            .topic("1")
            .definition_code("definition_code")
            .user_id_type("open_id")
            .locale("zh-CN")
            .page_size(100)
            .page_token("token_1");

        assert_eq!(request.topic, "1");
        assert_eq!(request.definition_code.as_deref(), Some("definition_code"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token.as_deref(), Some("token_1"));
    }

    #[test]
    fn test_list_task_response_deserialize() {
        let json = serde_json::json!({
            "tasks": [{
                "topic": "1",
                "user_id": "ou_xxx",
                "title": "审批",
                "status": "Todo",
                "instance_status": "Running",
                "definition_code": "ABC",
                "initiator": "ou_init",
                "initiator_name": "张三",
                "task_id": "t1",
                "instance_code": "INS1",
                "definition_group_id": "g1",
                "definition_group_name": "考勤",
                "definition_name": "加班",
                "summaries": [{"key": "k", "value": "v"}]
            }],
            "has_more": true,
            "page_token": "next"
        });
        let resp: ListTaskResponseV4 =
            serde_json::from_value(json).expect("list response should deserialize");
        assert_eq!(resp.tasks.len(), 1);
        assert_eq!(resp.tasks[0].definition_name, "加班");
        assert_eq!(resp.tasks[0].summaries.len(), 1);
        assert_eq!(resp.has_more, Some(true));
    }
}
