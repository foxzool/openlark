//! 获取素材临时下载链接
//!
//! 通过 file_tokens 获取素材临时下载链接，链接时效性是 24 小时，过期失效。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/media/batch_get_tmp_download_url>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
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
    pub fn new(config: Config, file_tokens: Vec<String>) -> Self {
        Self {
            config,
            file_tokens,
            extra: None,
        }
    }

    pub fn add_file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_tokens.push(file_token.into());
        self
    }

    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub async fn execute(self) -> SDKResult<BatchGetTmpDownloadUrlResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetTmpDownloadUrlResponse> {
        // ===== 验证必填字段 =====
        if self.file_tokens.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_tokens",
                "file_tokens 不能为空",
            ));
        }
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
        let request = ApiRequest::<BatchGetTmpDownloadUrlResponse>::get(url);

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
    use openlark_core::testing::prelude::test_runtime;

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

    /// 测试空 file_tokens 验证
    #[test]
    fn test_empty_file_tokens_validation() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(config, vec![]);

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 file_tokens 数量限制
    #[test]
    fn test_file_tokens_count_validation() {
        let config = Config::default();

        // 最大 5 个 token
        let tokens_5: Vec<String> = ["a"; 5].iter().map(|s| s.to_string()).collect();
        let request1 = BatchGetTmpDownloadUrlRequest::new(config.clone(), tokens_5);
        assert_eq!(request1.file_tokens.len(), 5);

        // 超过 5 个
        let tokens_6: Vec<String> = ["a"; 6].iter().map(|s| s.to_string()).collect();
        let request2 = BatchGetTmpDownloadUrlRequest::new(config, tokens_6);

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试空 token 验证
    #[test]
    fn test_empty_token_in_list_validation() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(
            config,
            vec!["valid_token".to_string(), "".to_string()],
        );

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
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
}
