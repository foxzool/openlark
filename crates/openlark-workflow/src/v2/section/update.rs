//! 更新分组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section/update>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::section::models::{UpdateSectionBody, UpdateSectionResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 更新分组请求
#[derive(Debug, Clone)]
pub struct UpdateSectionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 分组 GUID
    section_guid: String,
    /// 请求体
    body: UpdateSectionBody,
}

impl UpdateSectionRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String, section_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            section_guid,
            body: UpdateSectionBody::default(),
        }
    }

    /// 设置分组标题
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.body.summary = Some(summary.into());
        self
    }

    /// 设置分组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.body.description = Some(description.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateSectionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.section_guid.trim(), "分组GUID不能为空");

        let api_endpoint =
            TaskApiV2::SectionUpdate(self.tasklist_guid.clone(), self.section_guid.clone());
        let mut request = ApiRequest::<UpdateSectionResponse>::put(api_endpoint.to_url());

        let request_body = &self.body;
        request = request.body(serialize_params(request_body, "更新分组")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新分组")
    }
}

impl ApiResponseTrait for UpdateSectionResponse {
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
    fn test_update_section_builder() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = UpdateSectionRequest::new(
            config,
            "tasklist_123".to_string(),
            "section_456".to_string(),
        )
        .summary("更新的标题");

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.section_guid, "section_456");
        assert_eq!(request.body.summary, Some("更新的标题".to_string()));
    }
}
