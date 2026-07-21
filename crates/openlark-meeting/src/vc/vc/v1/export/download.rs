//! 下载导出文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/download>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 下载导出文件请求

#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 下载导出文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadExportResponse {
    /// 下载 URL
    pub url: String,
}

impl ApiResponseTrait for DownloadExportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DownloadExportRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
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
        let mut api_request: ApiRequest<DownloadExportResponse> =
            ApiRequest::get("/open-apis/vc/v1/exports/download");

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        Transport::request_typed(api_request, &self.config, Some(option), "下载导出文件").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../exports/download + query 拼装 + 强类型 DownloadExportResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_download_export_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/exports/download"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "url": "https://files.example.com/export_001.zip" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DownloadExportRequest::new(config)
            .query_param("task_id", "task_001")
            .execute()
            .await
            .expect("下载导出文件应成功");
        assert_eq!(resp.url, "https://files.example.com/export_001.zip");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/exports/download");
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("task_id=task_001")
        );
    }
}
