//! 删除分组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section/delete>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::section::models::DeleteSectionResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 删除分组请求
#[derive(Debug, Clone)]
pub struct DeleteSectionRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务清单 GUID
    tasklist_guid: String,
    /// 分组 GUID
    section_guid: String,
}

impl DeleteSectionRequest {
    pub fn new(config: Arc<Config>, tasklist_guid: String, section_guid: String) -> Self {
        Self {
            config,
            tasklist_guid,
            section_guid,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteSectionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteSectionResponse> {
        // 验证必填字段
        validate_required!(self.tasklist_guid.trim(), "任务清单GUID不能为空");
        validate_required!(self.section_guid.trim(), "分组GUID不能为空");

        let api_endpoint =
            TaskApiV2::SectionDelete(self.tasklist_guid.clone(), self.section_guid.clone());
        let request = ApiRequest::<DeleteSectionResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除分组")
    }
}

impl ApiResponseTrait for DeleteSectionResponse {
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
    fn test_delete_section_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = DeleteSectionRequest::new(
            Arc::new(config),
            "tasklist_123".to_string(),
            "section_456".to_string(),
        );

        assert_eq!(request.tasklist_guid, "tasklist_123");
        assert_eq!(request.section_guid, "section_456");
    }
}
