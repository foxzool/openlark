//! 获取任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::ListTasksResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

/// 获取任务列表请求
#[derive(Debug, Clone)]
pub struct ListTasksRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: Option<String>,
    /// 分组 GUID
    section_guid: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 筛选条件（公式字符串）
    filter: Option<String>,
    /// 排序条件（JSON 数组）
    sort: Option<serde_json::Value>,
    /// 用户类型
    user_type: Option<String>,
}

impl ListTasksRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            tasklist_guid: None,
            section_guid: None,
            page_size: None,
            page_token: None,
            filter: None,
            sort: None,
            user_type: None,
        }
    }

    /// 设置任务清单 GUID
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.tasklist_guid = Some(tasklist_guid.into());
        self
    }

    /// 设置分组 GUID
    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.section_guid = Some(section_guid.into());
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

    /// 设置筛选条件（公式字符串）
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 设置排序条件（JSON 数组）
    pub fn sort(mut self, sort: serde_json::Value) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 设置用户类型
    pub fn user_type(mut self, user_type: impl Into<String>) -> Self {
        self.user_type = Some(user_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTasksResponse> {
        let api_endpoint = TaskApiV2::TaskList;
        let mut request = ApiRequest::<ListTasksResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(tasklist_guid) = &self.tasklist_guid {
            request = request.query("tasklist_guid", tasklist_guid);
        }
        if let Some(section_guid) = &self.section_guid {
            request = request.query("section_guid", section_guid);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(filter) = &self.filter {
            request = request.query("filter", filter);
        }
        if let Some(user_type) = &self.user_type {
            request = request.query("user_type", user_type);
        }
        if let Some(sort) = &self.sort {
            request = request.query("sort", sort.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取任务列表")
    }
}

impl ApiResponseTrait for ListTasksResponse {
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
    fn test_list_tasks_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = ListTasksRequest::new(Arc::new(config))
            .tasklist_guid("tasklist_123")
            .page_size(20);

        assert_eq!(request.tasklist_guid, Some("tasklist_123".to_string()));
        assert_eq!(request.page_size, Some(20));
    }
}
