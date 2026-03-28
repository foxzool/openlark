//! 任务加入清单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/task-add_tasklist/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 任务加入清单请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddTasklistBody {
    /// 任务清单 GUID
    pub tasklist_guid: String,
    /// 分组 GUID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_guid: Option<String>,
}

/// 任务加入清单响应
#[derive(Debug, Clone, Deserialize)]
pub struct AddTasklistResponse {
    /// 任务 GUID
    pub task_guid: String,
    /// 任务清单 GUID
    pub tasklist_guid: String,
    /// 分组 GUID
    #[serde(default)]
    pub section_guid: Option<String>,
}

/// 任务加入清单请求
#[derive(Debug, Clone)]
pub struct AddTasklistRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 请求体
    body: AddTasklistBody,
}

impl AddTasklistRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: AddTasklistBody::default(),
        }
    }

    /// 设置任务清单 GUID
    pub fn tasklist_guid(mut self, tasklist_guid: impl Into<String>) -> Self {
        self.body.tasklist_guid = tasklist_guid.into();
        self
    }

    /// 设置分组 GUID
    pub fn section_guid(mut self, section_guid: impl Into<String>) -> Self {
        self.body.section_guid = Some(section_guid.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddTasklistResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.body.tasklist_guid.trim(), "任务清单GUID不能为空");

        let api_endpoint = TaskApiV2::TaskAddTasklist(self.task_guid.clone());
        let mut request = ApiRequest::<AddTasklistResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "任务加入清单")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "任务加入清单")
    }
}

impl ApiResponseTrait for AddTasklistResponse {
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
    fn test_add_tasklist_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = AddTasklistRequest::new(config, "task_123")
            .tasklist_guid("tasklist_456")
            .section_guid("section_789");

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.tasklist_guid, "tasklist_456");
        assert_eq!(request.body.section_guid, Some("section_789".to_string()));
    }

    #[test]
    fn test_task_add_tasklist_api_v2_url() {
        let endpoint = TaskApiV2::TaskAddTasklist("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/tasks/task_123/add_tasklist"
        );
    }
}
