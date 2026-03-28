//! 下载文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/file/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_FILES};

/// 下载文件请求
///
/// 用于从飞书服务器下载文件。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `file_key`: 文件 key，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetFileRequest::new(config)
///     .file_key("file_key_xxx")
///     .execute().await?;
/// ```
pub struct GetFileRequest {
    config: Config,
    file_key: String,
}

impl GetFileRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            file_key: String::new(),
        }
    }

    /// 文件 key（路径参数）
    pub fn file_key(mut self, file_key: impl Into<String>) -> Self {
        self.file_key = file_key.into();
        self
    }

    /// 执行请求（返回二进制内容）
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/file/get>
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Vec<u8>> {
        // === 必填字段验证 ===
        validate_required!(self.file_key, "file_key 不能为空");

        // url: GET:/open-apis/im/v1/files/:file_key
        let req: ApiRequest<Vec<u8>> =
            ApiRequest::get(format!("{}/{}", IM_V1_FILES, self.file_key));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "下载文件")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_request_builder() {
        let config = Config::default();
        let request = GetFileRequest::new(config).file_key("file_key_xxx");
        assert_eq!(request.file_key, "file_key_xxx");
    }

    #[test]
    fn test_get_file_request_default_values() {
        let config = Config::default();
        let request = GetFileRequest::new(config);
        assert_eq!(request.file_key, "");
    }

    #[test]
    fn test_get_file_request_with_different_keys() {
        let config = Config::default();
        let request1 = GetFileRequest::new(config.clone()).file_key("file_111");
        let request2 = GetFileRequest::new(config).file_key("file_222");
        assert_eq!(request1.file_key, "file_111");
        assert_eq!(request2.file_key, "file_222");
    }
}
