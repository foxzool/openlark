//! 上传文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/file/create>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_FILES,
    im::im::v1::file::models::CreateFileResponse,
};

/// 上传文件请求体（multipart 表单字段）
///
/// # 字段说明
///
/// - `file_type`: 文件类型，必填
/// - `file_name`: 文件名，必填
/// - `duration`: 视频时长（秒），可选
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateFileBody::new("pdf", "document.pdf");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileBody {
    file_type: String,
    file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i32>,
}

impl CreateFileBody {
    pub fn new(file_type: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            file_type: file_type.into(),
            file_name: file_name.into(),
            duration: None,
        }
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.file_type = file_type.into();
        self
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = file_name.into();
        self
    }

    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// 上传文件请求
///
/// 用于上传文件到飞书服务器。
///
/// # 字段说明
///
/// - `config`: 配置信息
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateFileBody::new("pdf", "document.pdf");
/// let file_bytes = vec![...]; // 文件二进制内容
/// let request = CreateFileRequest::new(config)
///     .execute(body, file_bytes).await?;
/// ```
pub struct CreateFileRequest {
    config: Config,
}

impl CreateFileRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口为 multipart 上传，请传入文件元信息 + 文件二进制内容。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/file/create>
    pub async fn execute(
        self,
        body: CreateFileBody,
        file_bytes: Vec<u8>,
    ) -> SDKResult<CreateFileResponse> {
        self.execute_with_options(
            body,
            file_bytes,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateFileBody,
        file_bytes: Vec<u8>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        // === 必填字段验证 ===
        validate_required!(body.file_type, "file_type 不能为空");
        validate_required!(body.file_name, "file_name 不能为空");
        if file_bytes.is_empty() {
            return Err(error::validation_error(
                "file 不能为空".to_string(),
                "不允许上传空文件".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/files
        let req: ApiRequest<CreateFileResponse> = ApiRequest::post(IM_V1_FILES)
            .body(serialize_params(&body, "上传文件")?)
            .file_content(file_bytes);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "上传文件")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file_body_builder() {
        let body = CreateFileBody::new("pdf", "document.pdf");
        assert_eq!(body.file_type, "pdf");
        assert_eq!(body.file_name, "document.pdf");
        assert!(body.duration.is_none());
    }

    #[test]
    fn test_create_file_body_with_duration() {
        let body = CreateFileBody::new("mp4", "video.mp4").duration(120);
        assert_eq!(body.duration, Some(120));
    }

    #[test]
    fn test_create_file_request_builder() {
        let config = Config::default();
        let request = CreateFileRequest::new(config);
        // Just verify the request can be created
        assert_eq!(request.config.app_id(), "");
    }

    #[test]
    fn test_create_file_body_chaining() {
        let body = CreateFileBody::new("pdf", "doc.pdf")
            .file_type("docx")
            .file_name("new_doc.docx");
        assert_eq!(body.file_type, "docx");
        assert_eq!(body.file_name, "new_doc.docx");
    }
}
