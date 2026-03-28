//! 获取附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/attachment/get>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

/// 附件信息
#[derive(Debug, Clone, Deserialize)]
pub struct AttachmentInfo {
    /// 附件 GUID
    pub attachment_guid: String,
    /// 任务 GUID
    pub task_guid: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件类型
    pub file_type: String,
    /// 上传时间
    pub created_at: String,
    /// 上传者 ID
    #[serde(default)]
    pub creator_id: Option<String>,
}

/// 获取附件响应
#[derive(Debug, Clone, Deserialize)]
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
}

impl GetAttachmentRequest {
    pub fn new(config: Arc<Config>, attachment_guid: impl Into<String>) -> Self {
        Self {
            config,
            attachment_guid: attachment_guid.into(),
        }
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
        let request = ApiRequest::<GetAttachmentResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取附件")
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
