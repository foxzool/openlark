//! 下载文件
//!
//! 使用该接口可以下载在云空间目录下的文件（不含飞书文档/电子表格/多维表格等在线文档）。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/download/download>

use crate::common::api_endpoints::DriveApi;
use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    SDKResult,
};

/// 下载文件请求
///
/// 用于下载云空间目录下的文件（不含飞书文档/电子表格/多维表格等在线文档）。
///
/// # 字段说明
///
/// - `file_token`: 文件的 token，不能为空
/// - `range`: HTTP Range 头，用于分片下载（可选），格式如 "bytes=0-100"
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::file::DownloadFileRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = DownloadFileRequest::new(config, "file_token");
/// let response = request.execute().await?;
/// println!("文件大小: {} 字节", response.data.len());
/// ```
#[derive(Debug)]
pub struct DownloadFileRequest {
    config: Config,
    /// 文件 token
    pub file_token: String,
    /// Range HTTP header，用于分片下载（可选），格式如 "bytes=0-100"
    pub range: Option<String>,
}

impl DownloadFileRequest {
    /// 创建下载文件请求
    ///
    /// # 参数
    ///
    /// - `config`: SDK 配置实例
    /// - `file_token`: 文件 token
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let request = DownloadFileRequest::new(config, "file_token");
    /// ```
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            range: None,
        }
    }

    /// 设置 HTTP Range 头
    ///
    /// 用于分片下载，格式如 "bytes=0-100"。
    ///
    /// # 参数
    ///
    /// - `range`: Range 头值，例如 "bytes=0-100" 或 "bytes=0-"
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 执行下载请求（使用默认选项）
    ///
    /// 成功时返回文件二进制内容。
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用自定义选项执行请求
    ///
    /// # 参数
    ///
    /// - `option`: 请求选项，可用于设置超时、重试策略等
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Response<Vec<u8>>> {
        // === 必填字段验证 ===
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        // === 业务规则验证 ===
        // Range 格式验证
        if let Some(r) = &self.range {
            // 文档约束：Range 形如 bytes=start-end（start/end 均为非负整数，end 可省略）
            let r = r.trim();
            if !r.starts_with("bytes=") {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range 必须以 bytes= 开头，例如 bytes=0-1023",
                ));
            }
            let range_spec = &r["bytes=".len()..];
            let (start, end) = range_spec.split_once('-').ok_or_else(|| {
                openlark_core::error::validation_error(
                    "range",
                    "Range 格式错误，应为 bytes=start-end，例如 bytes=0-1023",
                )
            })?;
            if start.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range start 不能为空，例如 bytes=0-1023",
                ));
            }
            if start.trim().parse::<u64>().is_err() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range start 必须为非负整数",
                ));
            }
            if !end.trim().is_empty() && end.trim().parse::<u64>().is_err() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range end 必须为非负整数或为空（例如 bytes=0-）",
                ));
            }
        }

        let api_endpoint = DriveApi::DownloadFile(self.file_token.clone());
        let mut request = ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url());

        if let Some(r) = &self.range {
            request = request.header("Range", r);
        }

        Transport::request(request, &self.config, Some(option)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_file_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let file_token = "test_file_token_123";
        let range_header = "bytes=0-100";

        let request = DownloadFileRequest::new(config.clone(), file_token).range(range_header);

        assert_eq!(request.file_token, file_token);
        assert_eq!(request.range, Some(range_header.to_string()));
    }

    #[test]
    fn test_download_file_with_empty_file_token() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "");
        assert_eq!(request.file_token, "");
        // 空 file_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_download_file_with_invalid_range_no_prefix() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("0-100");
        assert_eq!(request.range, Some("0-100".to_string()));
        // 缺少 bytes= 前缀应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_download_file_with_invalid_range_no_dash() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("bytes=0100");
        assert_eq!(request.range, Some("bytes=0100".to_string()));
        // 缺少连字符应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_download_file_with_invalid_range_empty_start() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("bytes=-100");
        assert_eq!(request.range, Some("bytes=-100".to_string()));
        // 空起始位置应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_download_file_with_invalid_range_non_numeric() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("bytes=abc-100");
        assert_eq!(request.range, Some("bytes=abc-100".to_string()));
        // 非数字起始位置应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_download_file_with_valid_range_end_empty() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("bytes=0-");
        assert_eq!(request.range, Some("bytes=0-".to_string()));
        // 这是一个有效的 range（从位置0到文件末尾）
    }

    #[test]
    fn test_download_file_with_valid_range_full() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token").range("bytes=0-1023");
        assert_eq!(request.range, Some("bytes=0-1023".to_string()));
        // 这是一个有效的 range
    }

    #[test]
    fn test_download_file_without_range() {
        let config = Config::default();
        let request = DownloadFileRequest::new(config, "file_token");
        assert_eq!(request.range, None);
        // 不设置 range 也可以正常下载整个文件
    }
}
