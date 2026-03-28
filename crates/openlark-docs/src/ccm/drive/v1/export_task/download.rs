//! 下载导出文件
//!
//! 下载导出的文件内容。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/download>

use crate::common::api_endpoints::DriveApi;
use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 下载导出文件请求
#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    config: Config,
    /// 文件token
    pub file_token: String,
}

impl DownloadExportRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    /// 执行下载请求，返回二进制内容
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行下载请求，返回二进制内容（带请求选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response<Vec<u8>>> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::DownloadExportFile(self.file_token.clone());

        let api_request = ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, Some(option)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_download_export_request_builder() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token");
        assert_eq!(request.file_token, "file_token");
    }

    /// 测试 file_token 为空时的验证
    #[test]
    fn test_empty_file_token_validation() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
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
}
