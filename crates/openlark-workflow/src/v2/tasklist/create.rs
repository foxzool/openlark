//! 创建任务清单
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/tasklist/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::tasklist::models::{CreateTasklistBody, CreateTasklistResponse, TasklistIcon};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建任务清单请求
#[derive(Debug, Clone)]
pub struct CreateTasklistRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 请求体
    body: CreateTasklistBody,
}

impl CreateTasklistRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateTasklistBody::default(),
        }
    }

    /// 设置任务清单标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
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
    pub async fn execute(self) -> SDKResult<CreateTasklistResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTasklistResponse> {
        // 验证必填字段
        validate_required!(self.body.summary.trim(), "任务清单标题不能为空");

        let api_endpoint = TaskApiV2::TasklistCreate;
        let mut request = ApiRequest::<CreateTasklistResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建任务清单")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建任务清单")
    }
}

impl ApiResponseTrait for CreateTasklistResponse {
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
    fn test_create_tasklist_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = CreateTasklistRequest::new(config)
            .summary("测试任务清单")
            .description("任务清单描述");

        assert_eq!(request.body.summary, "测试任务清单");
        assert_eq!(request.body.description, Some("任务清单描述".to_string()));
    }
}
