//! 上传进展记录图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/image/upload>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// 上传进展记录图片请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// 插入图片所在的待创建/修改的进展记录对应的目标 ID（必填）
    target_id: String,
    /// 插入图片所在的待创建/修改的进展记录对应的目标类型（必填）
    /// - 2: okr 的 O
    /// - 3: okr 的 KR
    target_type: i32,
    /// 图片文件名（用于 multipart 文件部件与 MIME 类型推断）
    file_name: String,
    /// 图片二进制数据（必填，支持 JPG/JPEG/PNG/WEBP/GIF/BMP/ICO/TIFF/HEIC）
    image_data: Vec<u8>,
    /// 配置信息
    config: Config,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        target_id: String,
        target_type: i32,
        file_name: String,
        image_data: Vec<u8>,
    ) -> Self {
        Self {
            target_id,
            target_type,
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
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.target_id.trim(), "target_id");
        if !(2..=3).contains(&self.target_type) {
            return Err(openlark_core::error::validation_error(
                "target_type 无效",
                "target_type 必须为 2（okr 的 O）或 3（okr 的 KR）",
            ));
        }
        validate_required!(self.file_name.trim(), "file_name");
        validate_required!(self.image_data, "图片数据不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ImageUpload;
        let request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体（multipart/form-data：data/target_id/target_type）
        let random_suffix: u32 = rand::thread_rng().gen_range(0..1000000);
        let boundary = format!("----OpenLarkBoundary{random_suffix:06}");
        let mime_type = detect_mime_type(self.file_name.trim());
        let body = build_multipart_body(
            self.file_name.trim(),
            &self.image_data,
            &self.target_id,
            self.target_type,
            &boundary,
            mime_type,
        );

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
            "上传进展记录图片响应数据为空",
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
        "ico" => "image/x-icon",
        "tiff" | "tif" => "image/tiff",
        "heic" => "image/heic",
        _ => "application/octet-stream",
    }
}

/// 构建 multipart 表单数据（官方字段：data 文件 + target_id + target_type）
fn build_multipart_body(
    file_name: &str,
    image_data: &[u8],
    target_id: &str,
    target_type: i32,
    boundary: &str,
    mime_type: &str,
) -> Vec<u8> {
    let mut body = Vec::new();

    // 图片文件部分（部件名 data）
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"data\"; filename=\"{file_name}\"\r\n")
            .as_bytes(),
    );
    body.extend_from_slice(format!("Content-Type: {mime_type}\r\n\r\n").as_bytes());
    body.extend_from_slice(image_data);
    body.extend_from_slice(b"\r\n");

    // target_id 部分
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"target_id\"\r\n\r\n");
    body.extend_from_slice(target_id.as_bytes());
    body.extend_from_slice(b"\r\n");

    // target_type 部分
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"target_type\"\r\n\r\n");
    body.extend_from_slice(target_type.to_string().as_bytes());
    body.extend_from_slice(b"\r\n");

    // 结束边界
    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    body
}

/// 上传进展记录图片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 图片 token
    pub file_token: String,
    /// 图片下载链接
    pub url: String,
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

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_image_upload_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_string_contains, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"file_token": "boxToken1", "url": "https://example.com/i.png"}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/images/upload"))
            .and(body_string_contains("name=\"target_id\""))
            .and(body_string_contains("7041430377642082323"))
            .and(body_string_contains("target_type"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UploadRequest::new(
            config,
            "7041430377642082323".to_string(),
            2,
            "sample.png".to_string(),
            vec![1u8, 2, 3],
        )
        .execute()
        .await
        .expect("okr_v1_image_upload 应成功");

        assert_eq!(data.file_token, "boxToken1");
        assert_eq!(data.url, "https://example.com/i.png");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/images/upload");
        let content_type = received[0]
            .headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        assert!(content_type.starts_with("multipart/form-data"));
        let body = String::from_utf8_lossy(&received[0].body);
        assert!(body.contains("name=\"data\""));
        assert!(body.contains("name=\"target_id\""));
        assert!(body.contains("name=\"target_type\""));
    }
}
