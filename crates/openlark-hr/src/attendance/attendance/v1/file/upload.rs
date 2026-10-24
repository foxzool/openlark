//! 上传用户人脸识别照片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/file/upload>
//!
//! 官方请求：必填 query 参数 `file_name` + multipart/form-data 单一 `file` part；
//! 响应为 `data.file.file_id`。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validation_error,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// 上传用户人脸识别照片请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// 带后缀的文件名（必填，query 参数）
    file_name: String,
    /// 文件内容（必填，multipart `file` part）
    image_data: Vec<u8>,
    /// 配置信息
    config: Config,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config, file_name: String, image_data: Vec<u8>) -> Self {
        Self {
            file_name,
            image_data,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.image_data, "文件内容不能为空");

        // 验证文件名
        let file_name = self.file_name.trim();
        if file_name.is_empty() {
            return Err(validation_error("file_name", "文件名不能为空"));
        }
        if file_name.len() > 255 {
            return Err(validation_error(
                "file_name",
                "文件名长度不能超过 255 个字符",
            ));
        }
        // 检查路径遍历字符
        if file_name.contains("..") || file_name.contains('/') || file_name.contains('\\') {
            return Err(validation_error(
                "file_name",
                "文件名不能包含路径分隔符或路径遍历字符",
            ));
        }

        // 2. 构建端点并附加必填 query 参数 file_name
        let api_endpoint = AttendanceApiV1::FileUpload;
        let request =
            ApiRequest::<UploadResponse>::post(api_endpoint.to_url()).query("file_name", file_name);

        // 3. 构建请求体（multipart/form-data，官方仅 file part）
        // 生成随机 boundary 后缀避免冲突
        let random_suffix: u32 = rand::thread_rng().gen_range(0..1000000);
        let boundary = format!("----OpenLarkBoundary{random_suffix:06}");
        let mime_type = detect_mime_type(file_name);
        let body = build_multipart_body(file_name, &self.image_data, &boundary, mime_type);

        let request = request
            .header(
                "Content-Type",
                format!("multipart/form-data; boundary={boundary}"),
            )
            .body(body);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "上传用户人脸识别照片响应数据为空",
        )
        .await
    }
}

/// 根据文件扩展名检测 MIME 类型
fn detect_mime_type(file_name: &str) -> &'static str {
    let ext = file_name.rsplit('.').next().unwrap_or("").to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "application/octet-stream",
    }
}

/// 构建 multipart 表单数据（官方仅 file 一个 part）
fn build_multipart_body(
    file_name: &str,
    image_data: &[u8],
    boundary: &str,
    mime_type: &str,
) -> Vec<u8> {
    let mut body = Vec::new();

    // 文件部分
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n")
            .as_bytes(),
    );
    body.extend_from_slice(format!("Content-Type: {mime_type}\r\n\r\n").as_bytes());
    body.extend_from_slice(image_data);
    body.extend_from_slice(b"\r\n");

    // 结束边界
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    body
}

/// 上传用户人脸识别照片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 文件信息
    pub file: FileData,
}

/// 上传得到的文件信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileData {
    /// 文件 ID
    pub file_id: String,
}

impl ApiResponseTrait for UploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    /// 端到端：Builder→execute→Transport→mock→assert 嵌套响应解析 + query 参数 + multipart 形状。
    #[tokio::test]
    async fn test_attendance_v1_file_upload_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/files/upload"))
            .and(query_param("file_name", "face.jpg"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "file": {
                        "file_id": "6b30e7636a38861bbe02869c726a4612"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UploadRequest::new(config, "face.jpg".to_string(), vec![1u8, 2u8, 3u8])
            .execute()
            .await
            .expect("attendance_v1_file_upload 应成功");

        assert_eq!(data.file.file_id, "6b30e7636a38861bbe02869c726a4612");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/files/upload"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("file_name=face.jpg"),
            "缺必填 query 参数 file_name: {:?}",
            received[0].url.query()
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("name=\"file\""),
            "multipart 缺 file part: {body}"
        );
        assert!(
            !body.contains("name=\"user_id\""),
            "官方无 user_id 参数: {body}"
        );
    }
}
