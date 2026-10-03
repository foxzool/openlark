//! 下载用户人脸识别照片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/file/download>

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
    /// 照片 ID（必填）
    photo_id: String,
    /// 配置信息
    config: Config,
}

impl DownloadRequest {
    /// 创建请求
    pub fn new(config: Config, photo_id: String) -> Self {
        Self { photo_id, config }
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
        validate_required!(self.photo_id.trim(), "photo_id");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::FileDownload(self.photo_id.clone()).to_url();
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadResponse {
    /// 照片 ID
    pub photo_id: String,
    /// 用户 ID
    pub user_id: String,
    /// 照片数据（Base64 编码）
    pub photo_data: String,
    /// 照片格式
    pub content_type: String,
}

impl ApiResponseTrait for DownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_file_download_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"photo_id": "test", "user_id": "test", "photo_data": "test", "content_type": "test"}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/files/photo_001/download"))
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

        let data = DownloadRequest::new(config, "photo_001".to_string())
            .execute()
            .await
            .expect("attendance_v1_file_download 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/files/photo_001/download"
        );
    }
}
