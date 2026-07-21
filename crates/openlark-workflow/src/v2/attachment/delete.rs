//! 删除附件
//!
//! docPath: <https://open.feishu.cn/document/task-v2/attachment/delete>

use crate::common::api_endpoints::TaskApiV2;
use crate::v2::attachment::models::DeleteAttachmentResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除附件请求
#[derive(Debug, Clone)]
pub struct DeleteAttachmentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 附件 GUID
    attachment_guid: String,
}

impl DeleteAttachmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, attachment_guid: String) -> Self {
        Self {
            config,
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
        validate_required!(self.attachment_guid.trim(), "附件GUID不能为空");

        let api_endpoint = TaskApiV2::AttachmentDelete(self.attachment_guid.clone());
        let request = ApiRequest::<DeleteAttachmentResponse>::delete(api_endpoint.to_url());

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除附件",
        )
        .await
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

        let request = DeleteAttachmentRequest::new(Arc::new(config), "attachment_456".to_string());

        assert_eq!(request.attachment_guid, "attachment_456");
    }
}
