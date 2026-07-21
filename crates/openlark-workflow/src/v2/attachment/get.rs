//! 获取附件
//!
//! docPath: <https://open.feishu.cn/document/task-v2/attachment/get>

use crate::common::api_endpoints::TaskApiV2;
use crate::v2::attachment::models::AttachmentInfo;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 获取附件响应
#[derive(Debug, Clone, serde::Deserialize)]
pub struct GetAttachmentResponse {
    /// 附件信息
    pub attachment: AttachmentInfo,
}

/// 获取附件请求
#[derive(Debug, Clone)]
pub struct GetAttachmentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 附件 GUID
    attachment_guid: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl GetAttachmentRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, attachment_guid: impl Into<String>) -> Self {
        Self {
            config,
            attachment_guid: attachment_guid.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAttachmentResponse> {
        // 验证必填字段
        validate_required!(self.attachment_guid.trim(), "附件GUID不能为空");

        let api_endpoint = TaskApiV2::AttachmentGet(self.attachment_guid.clone());
        let mut request = ApiRequest::<GetAttachmentResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取附件",
        )
        .await
    }
}

impl ApiResponseTrait for GetAttachmentResponse {
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
    fn test_get_attachment_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = GetAttachmentRequest::new(Arc::new(config), "attachment_123");

        assert_eq!(request.attachment_guid, "attachment_123");
    }

    #[test]
    fn test_attachment_get_api_v2_url() {
        let endpoint = TaskApiV2::AttachmentGet("attachment_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v2/attachments/attachment_123"
        );
    }
}
