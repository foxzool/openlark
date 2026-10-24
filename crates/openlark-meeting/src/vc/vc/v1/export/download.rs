//! 下载导出文件
//!
//! 官方查询参数仅 `file_token`（string，必填）。成功响应为「返回文件二进制流」
//! （无 JSON 字段表），按二进制下载处理，保留响应头中的文件名。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/download>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 下载导出文件请求
#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    /// 配置信息
    config: Config,
    /// 导出文件 token（必填）
    file_token: String,
}

/// 下载得到的导出文件。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DownloadExportResponse {
    /// `Content-Disposition` 中的文件名；响应头缺失时为空字符串。
    pub file_name: String,
    /// 文件二进制内容。
    pub content: Vec<u8>,
}

impl ApiResponseTrait for DownloadExportResponse {
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

impl DownloadExportRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            file_token: String::new(),
        }
    }

    /// 设置导出文件 token（必填）
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_token = file_token.into();
        self
    }

    /// 执行请求（返回文件二进制内容）
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/download>
    pub async fn execute(self) -> SDKResult<DownloadExportResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DownloadExportResponse> {
        validate_required!(self.file_token, "file_token 不能为空");

        let api_request: ApiRequest<DownloadExportResponse> =
            ApiRequest::get("/open-apis/vc/v1/exports/download")
                .query("file_token", self.file_token);

        Transport::request_typed(api_request, &self.config, Some(option), "下载导出文件").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use wiremock::MockServer;

    /// 端到端：GET .../exports/download?file_token=... → 返回原始文件二进制流并保留文件名。
    #[tokio::test]
    async fn test_download_export_returns_binary_on_success() {
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let file_bytes = b"PK\x03\x04export-file".to_vec();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/exports/download"))
            .and(query_param("file_token", "ftk_001"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-disposition", "attachment; filename=\"export.mp4\"")
                    .set_body_bytes(file_bytes.clone()),
            )
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DownloadExportRequest::new(config)
            .file_token("ftk_001")
            .execute()
            .await
            .expect("下载导出文件应成功");
        assert_eq!(resp.file_name, "export.mp4");
        assert_eq!(resp.content, file_bytes);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/exports/download");
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("file_token=ftk_001")
        );
    }

    /// file_token 缺失时应被校验拦截，不发请求。
    #[tokio::test]
    async fn test_download_export_rejects_empty_file_token() {
        let server = MockServer::start().await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let result = DownloadExportRequest::new(config).execute().await;
        assert!(result.is_err());
        assert!(
            server
                .received_requests()
                .await
                .unwrap_or_default()
                .is_empty()
        );
    }
}
