//! 列取任务所在清单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-tasklists/list>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 任务清单关联信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskTasklistItem {
    /// 任务清单 GUID
    pub tasklist_guid: String,
    /// 任务清单名称
    pub tasklist_name: String,
    /// 分组 GUID
    #[serde(default)]
    pub section_guid: Option<String>,
    /// 分组名称
    #[serde(default)]
    pub section_name: Option<String>,
}

/// 列取任务所在清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetTaskTasklistsResponse {
    /// 任务清单列表
    #[serde(default)]
    pub items: Vec<TaskTasklistItem>,
    /// 总数
    #[serde(default)]
    pub total: Option<i32>,
}

/// 列取任务所在清单请求
#[derive(Debug, Clone)]
pub struct GetTaskTasklistsRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
}

impl GetTaskTasklistsRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetTaskTasklistsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTaskTasklistsResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");

        let api_endpoint = TaskApiV2::TaskGetTasklists(self.task_guid.clone());
        let request = ApiRequest::<GetTaskTasklistsResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取任务所在清单")
    }
}

impl ApiResponseTrait for GetTaskTasklistsResponse {
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
    fn test_get_task_tasklists_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = GetTaskTasklistsRequest::new(Arc::new(config), "task_123");

        assert_eq!(request.task_guid, "task_123");
    }

    #[test]
    fn test_task_get_tasklists_api_v2_url() {
        let endpoint = TaskApiV2::TaskGetTasklists("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/tasklists"
        );
    }
}
