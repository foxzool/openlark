//! 下载用户人脸识别照片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/file/download>
//!
//! 官方请求仅路径参数 `file_id`，成功响应为照片文件二进制流（非 JSON envelope）。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 下载用户人脸识别照片请求
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// 文件 ID（必填，路径参数，可通过批量查询用户人脸识别信息获取）
    file_id: String,
    /// 配置信息
    config: Config,
}

impl DownloadRequest {
    /// 创建请求
    pub fn new(config: Config, file_id: String) -> Self {
        Self { file_id, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DownloadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DownloadResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.file_id, "file_id 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::FileDownload(self.file_id.clone()).to_url();
        let request = ApiRequest::<DownloadResponse>::get(&api_endpoint);
        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "下载用户人脸识别照片响应数据为空",
        )
        .await
    }
}

/// 下载用户人脸识别照片响应
///
/// 官方 200 响应为照片文件二进制流（非 JSON envelope）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadResponse {
    /// `Content-Disposition` 中的文件名；响应头缺失时为空字符串
    pub file_name: String,
    /// 照片二进制内容
    pub content: Vec<u8>,
}

impl ApiResponseTrait for DownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            file_name,
            content: body,
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_download_request_builder_new() {
        let request = DownloadRequest::new(TestConfigBuilder::new().build(), "test".to_string());
        let _ = request;
    }

    #[test]
    fn test_download_response_binary_decode() {
        let response =
            DownloadResponse::from_binary("face.jpg".to_string(), b"jpeg-bytes".to_vec())
                .expect("二进制响应解码失败");
        assert_eq!(response.file_name, "face.jpg");
        assert_eq!(response.content, b"jpeg-bytes".to_vec());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 二进制响应 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_file_download_returns_binary_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let photo_bytes = b"\xFF\xD8\xFF\xE0 fake-jpeg".to_vec();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/files/file_001/download"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-disposition", "attachment; filename=\"face.jpg\"")
                    .set_body_bytes(photo_bytes.clone()),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = DownloadRequest::new(config, "file_001".to_string())
            .execute()
            .await
            .expect("attendance_v1_file_download 应成功");

        assert_eq!(data.file_name, "face.jpg");
        assert_eq!(data.content, photo_bytes);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/files/file_001/download"
        );
    }
}
