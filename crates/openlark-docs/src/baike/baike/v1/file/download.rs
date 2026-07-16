//! 下载图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/file/download>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::api_endpoints::BaikeApiV1;

/// 下载图片请求
pub struct DownloadFileRequest {
    config: Config,
    /// 文件 token。
    file_token: String,
}

impl DownloadFileRequest {
    /// 创建新的图片下载请求。
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    /// 下载原图，返回二进制内容（包含 HTTP 头部）
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 下载原图（支持自定义选项），返回二进制内容（包含 HTTP 头部）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response<Vec<u8>>> {
        validate_required!(self.file_token, "file_token 不能为空");

        // 使用 catalog 提供 method + path + auth（#443）
        let api_request: ApiRequest<Vec<u8>> =
            BaikeApiV1::FileDownload(self.file_token).to_request();
        Transport::request(api_request, &self.config, Some(option)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/baike/v1/files/{file_token}/download → 二进制内容。
    #[tokio::test]
    async fn test_download_file_returns_data_on_success() {
        let server = MockServer::start().await;
        let body = b"baike binary payload".to_vec();
        Mock::given(method("GET"))
            .and(path("/open-apis/lingo/v1/files/ftk001/download"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(body.clone()))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = DownloadFileRequest::new(config, "ftk001")
            .execute()
            .await
            .expect("下载文件应成功");
        let data = resp.data.expect("响应应包含二进制数据");
        assert_eq!(data, body);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/lingo/v1/files/ftk001/download"
        );
    }
}
