//! 删除附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/attachment/delete>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::attachment::models::DeleteAttachmentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 删除附件请求
#[derive(Debug, Clone)]
pub struct DeleteAttachmentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 附件 GUID
    attachment_guid: String,
}

impl DeleteAttachmentRequest {
    pub fn new(config: Arc<Config>, task_guid: String, attachment_guid: String) -> Self {
        Self {
            config,
            task_guid,
            attachment_guid,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteAttachmentResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.attachment_guid.trim(), "附件GUID不能为空");

        let api_endpoint =
            TaskApiV2::AttachmentDelete(self.task_guid.clone(), self.attachment_guid.clone());
        let request = ApiRequest::<DeleteAttachmentResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除附件")
    }
}

impl ApiResponseTrait for DeleteAttachmentResponse {
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
    fn test_delete_attachment_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = DeleteAttachmentRequest::new(
            Arc::new(config),
            "task_123".to_string(),
            "attachment_456".to_string(),
        );

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.attachment_guid, "attachment_456");
    }
}
