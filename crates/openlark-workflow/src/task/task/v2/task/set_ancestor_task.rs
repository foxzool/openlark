//! 设置父任务。
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/task-v2/task/set_ancestor_task>

use crate::common::{TaskV2Endpoint, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 设置父任务请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct SetAncestorTaskBody {
    /// 父任务 GUID。不设置时表示将任务转为独立任务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestor_guid: Option<String>,
    /// 用户 ID 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 设置父任务响应。
#[derive(Debug, Clone, Deserialize)]
pub struct SetAncestorTaskResponse {}

/// 设置父任务请求。
#[derive(Debug, Clone)]
pub struct SetAncestorTaskRequest {
    config: Arc<Config>,
    task_guid: String,
    body: SetAncestorTaskBody,
}

impl SetAncestorTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            body: SetAncestorTaskBody::default(),
        }
    }

    /// 设置父任务 GUID。
    pub fn ancestor_guid(mut self, ancestor_guid: impl Into<String>) -> Self {
        self.body.ancestor_guid = Some(ancestor_guid.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.body.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SetAncestorTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SetAncestorTaskResponse> {
        validate_required!(self.task_guid.trim(), "任务 GUID 不能为空");

        let api_endpoint = TaskV2Endpoint::TaskSetAncestorTask(self.task_guid.clone());
        let mut request = ApiRequest::<SetAncestorTaskResponse>::post(api_endpoint.to_url());
        request = request.body(serialize_params(&self.body, "设置父任务")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "设置父任务")
    }
}

impl ApiResponseTrait for SetAncestorTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_194_set_ancestor_task_builder() {
        let request = SetAncestorTaskRequest::new(Arc::new(Config::default()), "task_123")
            .ancestor_guid("parent_123")
            .user_id_type("open_id");

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.body.ancestor_guid.as_deref(), Some("parent_123"));
        assert_eq!(request.body.user_id_type.as_deref(), Some("open_id"));
    }
}
