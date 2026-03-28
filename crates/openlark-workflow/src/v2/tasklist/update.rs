//! 更新任务清单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist/update>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::models::{TasklistIcon, UpdateTasklistBody, UpdateTasklistResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 更新任务清单请求
#[derive(Debug, Clone)]
pub struct UpdateTasklistRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: UpdateTasklistBody,
}

impl UpdateTasklistRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            body: UpdateTasklistBody::default(),
        }
    }

    /// 设置任务清单标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = Some(summary.into());
        self
    }

    /// 设置任务清单描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 设置任务清单图标
    pub fn icon(mut self, icon: TasklistIcon) -> Self {
        self.body.icon = Some(icon);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateTasklistResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");

        let api_endpoint = TaskApiV2::TasklistUpdate(self.tasklist_guid.clone());
        let mut request = ApiRequest::<UpdateTasklistResponse>::put(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新任务清单")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新任务清单")
    }
}

impl ApiResponseTrait for UpdateTasklistResponse {
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
    fn test_update_tasklist_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            UpdateTasklistRequest::new(config, "tasklist_123".to_string()).summary("更新的标题");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.summary, Some("更新的标题".to_string()));
    }
}
