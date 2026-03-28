//! 上传附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/attachment/upload>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::attachment::models::UploadAttachmentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 上传附件请求
#[derive(Debug, Clone)]
pub struct UploadAttachmentRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 任务 GUID
    task_guid: String,
    /// 文件路径
    file_path: String,
    /// 文件名
    file_name: Option<String>,
}

impl UploadAttachmentRequest {
    pub fn new(config: Arc<Config>, task_guid: String, file_path: String) -> Self {
        Self {
            config,
            task_guid,
            file_path,
            file_name: None,
        }
    }

    /// 设置文件名（可选，默认使用文件路径的文件名）
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadAttachmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadAttachmentResponse> {
        // 验证必填字段
        validate_required!(self.task_guid.trim(), "任务GUID不能为空");
        validate_required!(self.file_path.trim(), "文件路径不能为空");

        let api_endpoint = TaskApiV2::AttachmentUpload(self.task_guid.clone());

        // 读取文件内容
        let file_content = std::fs::read(&self.file_path).map_err(|e| {
            openlark_core::error::validation_error(
                "读取文件失败",
                format!("无法读取文件 {}: {}", self.file_path, e).as_str(),
            )
        })?;

        // 使用文件名（如果提供）或从路径中提取
        let filename = self.file_name.unwrap_or_else(|| {
            std::path::Path::new(&self.file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("attachment")
                .to_string()
        });

        // 获取文件大小
        let file_size = file_content.len();

        // 创建 API 请求
        let mut request = ApiRequest::<UploadAttachmentResponse>::post(api_endpoint.to_url());
        request = request.body(RequestData::Binary(file_content));
        request = request.header("X-File-Name", filename);
        request = request.header("X-File-Size", file_size.to_string());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传附件")
    }
}

impl ApiResponseTrait for UploadAttachmentResponse {
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
    fn test_upload_attachment_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = UploadAttachmentRequest::new(
            Arc::new(config),
            "task_123".to_string(),
            "/path/to/file.pdf".to_string(),
        );

        assert_eq!(request.task_guid, "task_123");
        assert_eq!(request.file_path, "/path/to/file.pdf");
    }
}
