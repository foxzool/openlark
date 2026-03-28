//! 上传图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/file/upload>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 上传图片响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadFileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传图片请求
pub struct UploadFileRequest {
    config: Config,
    name: String,
    file: Vec<u8>,
}

impl UploadFileRequest {
    /// 创建上传图片请求
    ///
    /// `name`：表单字段 `name`，同时会用于 multipart 的文件名（内部字段 `file_name`）。
    pub fn new(config: Config, name: impl Into<String>, file: Vec<u8>) -> Self {
        Self {
            config,
            name: name.into(),
            file,
        }
    }

    pub async fn execute(self) -> SDKResult<UploadFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UploadFileResponse> {
        use crate::common::api_endpoints::BaikeApiV1;

        // ===== 参数校验 =====
        validate_required!(self.name, "name 不能为空");
        validate_required!(self.file, "file 不能为空");
        let name_len = self.name.chars().count();
        if !(1..=100).contains(&name_len) {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 长度必须在 1~100 字符之间",
            ));
        }
        // 文档：图片格式仅支持 icon、bmp、gif、png、jpeg、webp
        if let Some(ext) = self.name.rsplit('.').next() {
            let ext = ext.to_ascii_lowercase();
            let allowed = ["icon", "ico", "bmp", "gif", "png", "jpeg", "jpg", "webp"];
            if !allowed.contains(&ext.as_str()) {
                return Err(openlark_core::error::validation_error(
                    "name",
                    "文件格式仅支持 icon/bmp/gif/png/jpeg/webp",
                ));
            }
        } else {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 必须包含文件扩展名",
            ));
        }
        // 文档：大小在 3KB-10MB
        let size = self.file.len();
        if size < 3 * 1024 || size > 10 * 1024 * 1024 {
            return Err(openlark_core::error::validation_error(
                "file",
                "file 大小必须在 3KB~10MB 之间",
            ));
        }

        // multipart/form-data：
        // - name：文档要求的文件名称字段
        // - file：二进制文件内容（由 core 层 MultipartBuilder 处理）
        // - __file_name：仅用于设置 multipart 的文件名（不会作为表单字段发送）
        let name = self.name;
        let body = serde_json::json!({
            "name": name,
            "__file_name": name,
        });

        // ===== 构建请求并发送 =====
        let api_request: ApiRequest<UploadFileResponse> =
            ApiRequest::post(&BaikeApiV1::FileUpload.to_url())
                .body(body)
                .file_content(self.file);

        let response: Response<UploadFileResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let file_data = vec![0u8; 10 * 1024]; // 10KB
        let request = UploadFileRequest::new(config, "test.png", file_data);

        assert_eq!(request.name, "test.png");
        assert_eq!(request.file.len(), 10 * 1024);
    }

    #[test]
    fn test_upload_file_response_structure() {
        let response = UploadFileResponse {
            file_token: Some("file_token_123".to_string()),
        };

        assert_eq!(response.file_token, Some("file_token_123".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_upload_file_valid_extensions() {
        let valid_extensions = vec![
            "test.png",
            "test.jpg",
            "test.jpeg",
            "test.gif",
            "test.webp",
            "test.bmp",
            "test.ico",
        ];
        for name in valid_extensions {
            let config = Config::default();
            let file_data = vec![0u8; 5 * 1024]; // 5KB
            let request = UploadFileRequest::new(config, name, file_data);
            assert_eq!(request.name, name);
        }
    }
}
