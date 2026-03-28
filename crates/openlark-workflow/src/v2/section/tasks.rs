//! 获取自定义分组任务列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section-tasks/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::TaskItem;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取自定义分组任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListSectionTasksResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,

    /// 任务列表
    #[serde(default)]
    pub items: Vec<TaskItem>,
}

/// 获取自定义分组任务列表请求
#[derive(Debug, Clone)]
pub struct GetSectionTasksRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 分组 GUID
    section_guid: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 筛选条件（公式字符串）
    filter: Option<String>,
    /// 排序条件（JSON 数组）
    sort: Option<serde_json::Value>,
}

impl GetSectionTasksRequest {
    pub fn new(config: Arc<Config>, section_guid: impl Into<String>) -> Self {
        Self {
            config,
            section_guid: section_guid.into(),
            page_size: None,
            page_token: None,
            filter: None,
            sort: None,
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListSectionTasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListSectionTasksResponse> {
        // 验证必填字段
        validate_required!(self.section_guid.trim(), "分组GUID不能为空");

        let api_endpoint = TaskApiV2::SectionGetTasks(self.section_guid.clone());
        let mut request = ApiRequest::<ListSectionTasksResponse>::get(api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(filter) = &self.filter {
            request = request.query("filter", filter);
        }
        if let Some(sort) = &self.sort {
            request = request.query("sort", sort.to_string());
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取自定义分组任务列表")
    }
}

impl ApiResponseTrait for ListSectionTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_get_section_tasks_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = GetSectionTasksRequest::new(Arc::new(config), "section_123")
            .page_size(20)
            .filter("status = incomplete");

        assert_eq!(request.section_guid, "section_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.filter, Some("status = incomplete".to_string()));
    }

    #[test]
    fn test_section_get_tasks_api_v2_url() {
        let endpoint = TaskApiV2::SectionGetTasks("section_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/sections/section_123/tasks"
        );
    }
}
