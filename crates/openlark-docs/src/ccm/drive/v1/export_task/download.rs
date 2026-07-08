//! 下载导出文件
//!
//! 下载导出的文件内容。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/download>

use crate::common::api_endpoints::DriveApi;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

/// 默认最大下载大小限制（100MB）
const DEFAULT_MAX_DOWNLOAD_SIZE: usize = 100 * 1024 * 1024;

/// 下载导出文件请求
#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    config: Config,
    /// 文件token
    pub file_token: String,
    /// 最大允许下载大小（字节）
    max_size: usize,
}

impl DownloadExportRequest {
    /// 创建新的导出文件下载请求。
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            max_size: DEFAULT_MAX_DOWNLOAD_SIZE,
        }
    }

    /// 设置最大下载大小（字节）
    pub fn max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// 执行下载请求，返回二进制内容
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行下载请求，返回二进制内容（带请求选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response<Vec<u8>>> {
        validate_required!(self.file_token, "file_token 不能为空");

        let api_endpoint = DriveApi::DownloadExportFile(self.file_token.clone());

        let api_request = ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url());

        let result = Transport::request(api_request, &self.config, Some(option)).await;
        match result {
            Ok(response) => {
                let data_len = response.data.as_ref().map_or(0, <Vec<u8>>::len);
                if data_len > self.max_size {
                    return Err(openlark_core::error::validation_error(
                        "max_size",
                        &format!("下载文件大小 {} 超过限制 {}", data_len, self.max_size),
                    ));
                }
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_download_export_request_builder() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token");
        assert_eq!(request.file_token, "file_token");
    }

    /// 测试 file_token 边界值
    #[test]
    fn test_file_token_boundaries() {
        let config = Config::default();

        // 单字符 token
        let request1 = DownloadExportRequest::new(config.clone(), "a");
        assert_eq!(request1.file_token, "a");

        // 长 token
        let long_token = "a".repeat(100);
        let request2 = DownloadExportRequest::new(config, long_token);
        assert_eq!(request2.file_token.len(), 100);
    }

    #[test]
    fn test_download_export_default_max_size() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token");
        assert_eq!(request.max_size, 100 * 1024 * 1024);
    }

    #[test]
    fn test_download_export_custom_max_size() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token").max_size(2048);
        assert_eq!(request.max_size, 2048);
    }

    /// 端到端：GET .../export_tasks/file/{file_token}/download → Response<Vec<u8>> 二进制载荷。
    #[tokio::test]
    async fn test_download_export_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let body = b"PK\x03\x04 fake export file bytes";
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/drive/v1/export_tasks/file/ftk001/download",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(body.to_vec()))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DownloadExportRequest::new(config, "ftk001")
            .execute()
            .await
            .expect("下载导出文件应成功");
        let bytes = resp.data.expect("响应 data 应非空");
        assert_eq!(bytes, body.to_vec());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/export_tasks/file/ftk001/download"
        );
    }
}
