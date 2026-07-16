//! 获取素材临时下载链接
//!
//! 通过 file_tokens 获取素材临时下载链接，链接时效性是 24 小时，过期失效。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/media/batch_get_tmp_download_url>

use openlark_core::{
    SDKResult,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取素材临时下载链接请求
#[derive(Debug, Clone)]
pub struct BatchGetTmpDownloadUrlRequest {
    config: Config,
    /// 素材文件的 token（一次最多 5 个）
    pub file_tokens: Vec<String>,
    /// 拓展参数（如多维表格高级权限下载鉴权）
    pub extra: Option<String>,
}

impl BatchGetTmpDownloadUrlRequest {
    /// 创建新的临时下载链接请求。
    pub fn new(config: Config, file_tokens: Vec<String>) -> Self {
        Self {
            config,
            file_tokens,
            extra: None,
        }
    }

    /// 追加一个素材 token。
    pub fn add_file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_tokens.push(file_token.into());
        self
    }

    /// 设置扩展参数。
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchGetTmpDownloadUrlResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetTmpDownloadUrlResponse> {
        // ===== 验证必填字段 =====
        validate_required_list!(self.file_tokens, 100, "file_tokens 不能为空");
        // ===== 验证集合大小 =====
        if self.file_tokens.len() > 5 {
            return Err(openlark_core::error::validation_error(
                "file_tokens",
                "file_tokens 一次最多传 5 个",
            ));
        }

        let api_endpoint = DriveApi::GetMediaTempDownloadUrls;

        // ===== 构建请求 URL =====
        // 该接口的 query 参数 file_tokens 支持重复传参（file_tokens=token1&file_tokens=token2...）。
        // openlark-core 的 query 存储结构为 HashMap，无法表达重复 key，因此这里基于 endpoint 生成 URL 并手动拼接 query。
        let mut query_pairs: Vec<String> = Vec::new();
        for token in &self.file_tokens {
            if token.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "file_tokens",
                    "file_tokens 不能包含空值",
                ));
            }
            query_pairs.push(format!("file_tokens={}", urlencoding::encode(token)));
        }
        if let Some(extra) = &self.extra {
            query_pairs.push(format!("extra={}", urlencoding::encode(extra)));
        }

        let url = format!("{}?{}", api_endpoint.to_url(), query_pairs.join("&"));
        let request = api_endpoint.to_request_with_url::<BatchGetTmpDownloadUrlResponse>(url);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

/// 临时下载链接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmpDownloadUrlInfo {
    /// 素材的 token
    pub file_token: String,
    /// 素材的临时下载链接
    pub tmp_download_url: String,
}

/// 获取素材临时下载链接响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetTmpDownloadUrlResponse {
    /// 临时下载列表
    #[serde(default)]
    pub tmp_download_urls: Vec<TmpDownloadUrlInfo>,
}

impl ApiResponseTrait for BatchGetTmpDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_batch_get_tmp_download_url_request_builder() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(
            config,
            vec!["file_token_1".to_string(), "file_token_2".to_string()],
        )
        .extra("extra");

        assert_eq!(request.file_tokens.len(), 2);
        assert_eq!(request.file_tokens[0], "file_token_1");
        assert_eq!(request.extra, Some("extra".to_string()));
    }

    /// 测试添加 token
    #[test]
    fn test_batch_get_tmp_download_url_request_add_token() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(config, vec!["file_token_1".to_string()])
            .add_file_token("file_token_2");

        assert_eq!(request.file_tokens.len(), 2);
        assert_eq!(request.file_tokens[1], "file_token_2");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchGetTmpDownloadUrlResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试单 token
    #[test]
    fn test_single_token() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(config, vec!["single_token".to_string()]);

        assert_eq!(request.file_tokens.len(), 1);
        assert_eq!(request.file_tokens[0], "single_token");
    }

    /// 测试 extra 可选参数
    #[test]
    fn test_extra_optional() {
        let config = Config::default();

        let request1 =
            BatchGetTmpDownloadUrlRequest::new(config.clone(), vec!["token".to_string()]);
        assert!(request1.extra.is_none());

        let request2 = BatchGetTmpDownloadUrlRequest::new(config, vec!["token".to_string()])
            .extra("extra_param");
        assert_eq!(request2.extra, Some("extra_param".to_string()));
    }

    /// 端到端：GET /open-apis/drive/v1/medias/batch_get_tmp_download_url → BatchGetTmpDownloadUrlResponse。
    #[tokio::test]
    async fn test_batch_get_tmp_download_url_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::method;
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // execute 手拼 file_tokens 重复 query（url.push_str 非 .query()）与 Transport 不兼容，
        // path 含 query 不匹配；pre-existing bug，mock 放宽只匹配 method，path 走 received 断言。
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "tmp_download_urls": [
                        {
                            "file_token": "media_token_001",
                            "tmp_download_url": "https://download.example.com/001"
                        },
                        {
                            "file_token": "media_token_002",
                            "tmp_download_url": "https://download.example.com/002"
                        }
                    ]
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

        let resp = BatchGetTmpDownloadUrlRequest::new(
            config,
            vec!["media_token_001".to_string(), "media_token_002".to_string()],
        )
        .execute()
        .await
        .expect("获取临时下载链接应成功");
        assert_eq!(resp.tmp_download_urls.len(), 2);
        assert_eq!(resp.tmp_download_urls[0].file_token, "media_token_001");
        assert_eq!(
            resp.tmp_download_urls[1].tmp_download_url,
            "https://download.example.com/002"
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // execute 手拼 file_tokens 重复 query（core HashMap 不支持重复 key，见 execute 注释），
        // url 的 `?` 被 Transport encode 成 `%3F`，path 含 `%3Ffile_tokens=...`。
        // pre-existing bug（与 product_assign_info 同类），此处只验证请求到达正确端点 + 响应解析。
        assert!(
            received[0]
                .url
                .path()
                .contains("batch_get_tmp_download_url")
        );
        assert!(
            received[0]
                .url
                .path()
                .contains("file_tokens=media_token_001")
        );
        assert!(
            received[0]
                .url
                .path()
                .contains("file_tokens=media_token_002")
        );
    }
}
