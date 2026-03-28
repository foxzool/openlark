//! 创建分组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section/create>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::section::models::{CreateSectionBody, CreateSectionResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 创建分组请求
#[derive(Debug, Clone)]
pub struct CreateSectionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 请求体
    body: CreateSectionBody,
}

impl CreateSectionRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            body: CreateSectionBody::default(),
        }
    }

    /// 设置分组标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = summary.into();
        self
    }

    /// 设置分组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateSectionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.body.summary.trim(), "分组标题不能为空");

        let api_endpoint = TaskApiV2::SectionCreate(self.tasklist_guid.clone());
        let mut request = ApiRequest::<CreateSectionResponse>::post(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "创建分组")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建分组")
    }
}

impl ApiResponseTrait for CreateSectionResponse {
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
    fn test_create_section_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request =
            CreateSectionRequest::new(config, "tasklist_123".to_string()).summary("测试分组");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.body.summary, "测试分组");
    }
}
