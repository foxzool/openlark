//! 获取任务的子任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-subtask/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::TaskItem;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取子任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListSubtasksResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,

    /// 子任务列表
    #[serde(default)]
    pub items: Vec<TaskItem>,
}

/// 获取子任务列表请求
#[derive(Debug, Clone)]
pub struct ListSubtasksRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 父任务 GUID
    task_guid: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListSubtasksRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            page_size: None,
            page_token: None,
        }
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
    pub async fn execute(self) -> SDKResult<ListSubtasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListSubtasksResponse> {
        let api_endpoint = TaskApiV2::SubtaskList(self.task_guid.clone());
        let mut request = ApiRequest::<ListSubtasksResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取子任务列表")
    }
}

impl ApiResponseTrait for ListSubtasksResponse {
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
    fn test_list_subtasks_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = ListSubtasksRequest::new(Arc::new(config), "parent_task_123")
            .page_size(20)
            .page_token("next_page_token");

        assert_eq!(request.task_guid, "parent_task_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    #[test]
    fn test_list_subtasks_api_v2_url() {
        let endpoint = TaskApiV2::SubtaskList("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/subtasks"
        );
    }
}
