//! 下载素材
//!
//! 下载各种类型文档中的素材（如电子表格图片、附件等），支持通过 Range 分片下载。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/download

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

/// 下载素材请求
#[derive(Debug)]
pub struct DownloadMediaRequest {
    config: Config,
    /// 素材文件 token
    pub file_token: String,
    /// 拓展参数（如多维表格高级权限下载鉴权）
    pub extra: Option<String>,
    /// Range HTTP header（可选），示例：bytes=0-1024
    pub range: Option<String>,
    /// 最大允许下载大小（字节）
    max_size: usize,
}

impl DownloadMediaRequest {
    /// 创建新的素材下载请求。
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            extra: None,
            range: None,
            max_size: DEFAULT_MAX_DOWNLOAD_SIZE,
        }
    }

    /// 设置扩展参数。
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// 设置 Range 请求头。
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
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
        // ===== 验证必填字段 =====
        validate_required!(self.file_token, "file_token 不能为空");
        // ===== 验证字段格式 =====
        if let Some(range) = &self.range
            && (!range.starts_with("bytes=") || !range.contains('-'))
        {
            return Err(openlark_core::error::validation_error(
                "range",
                "range 格式必须为 bytes=start-end（例如 bytes=0-1024）",
            ));
        }

        let api_endpoint = DriveApi::DownloadMedia(self.file_token.clone());
        let mut request =
            ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url()).query_opt("extra", self.extra);

        if let Some(r) = self.range {
            request = request.header("Range", &r);
        }

        let result = Transport::request(request, &self.config, Some(option)).await;
        match result {
            Ok(response) => {
                // 检查下载大小是否超过限制
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
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_download_media_request() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "media_token")
            .extra("extra")
            .range("bytes=0-100");

        assert_eq!(request.file_token, "media_token");
        assert_eq!(request.extra, Some("extra".to_string()));
        assert_eq!(request.range, Some("bytes=0-100".to_string()));
    }

    /// 测试 file_token 为空时的验证
    #[test]
    fn test_empty_file_token_validation() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 range 格式验证
    #[test]
    fn test_range_format_validation() {
        let config = Config::default();

        // 缺少 bytes= 前缀
        let request1 = DownloadMediaRequest::new(config.clone(), "token").range("0-100");

        let result1 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request1.execute().await;
            })
        })
        .join();

        assert!(result1.is_ok());

        // 缺少连字符
        let request2 = DownloadMediaRequest::new(config.clone(), "token").range("bytes=0100");

        let result2 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result2.is_ok());
    }

    /// 测试有效的 range 格式
    #[test]
    fn test_valid_range_formats() {
        let config = Config::default();

        // 标准格式
        let request1 = DownloadMediaRequest::new(config.clone(), "token").range("bytes=0-1024");
        assert_eq!(request1.range, Some("bytes=0-1024".to_string()));

        // 单字节
        let request2 = DownloadMediaRequest::new(config.clone(), "token").range("bytes=0-0");
        assert_eq!(request2.range, Some("bytes=0-0".to_string()));

        // 大范围
        let request3 = DownloadMediaRequest::new(config.clone(), "token").range("bytes=0-999999");
        assert_eq!(request3.range, Some("bytes=0-999999".to_string()));
    }

    /// 测试可选参数
    #[test]
    fn test_optional_parameters() {
        let config = Config::default();

        // 不带可选参数
        let request1 = DownloadMediaRequest::new(config.clone(), "token");
        assert!(request1.extra.is_none());
        assert!(request1.range.is_none());

        // 带 extra
        let request2 = DownloadMediaRequest::new(config.clone(), "token").extra("extra_param");
        assert_eq!(request2.extra, Some("extra_param".to_string()));

        // 带 range
        let request3 = DownloadMediaRequest::new(config, "token").range("bytes=0-100");
        assert_eq!(request3.range, Some("bytes=0-100".to_string()));
    }

    #[test]
    fn test_download_media_default_max_size() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "media_token");
        assert_eq!(request.max_size, 100 * 1024 * 1024);
    }

    #[test]
    fn test_download_media_custom_max_size() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "media_token").max_size(512);
        assert_eq!(request.max_size, 512);
    }
}
