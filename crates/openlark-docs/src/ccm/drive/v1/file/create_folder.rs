//! 新建文件夹
//!
//! 在用户云空间的指定文件夹中创建一个新的空文件夹。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/create_folder>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use serde::{Deserialize, Serialize};

/// 新建文件夹请求
///
/// 用于在用户云空间的指定文件夹中创建一个新的空文件夹。
///
/// # 字段说明
///
/// - `name`: 文件夹名称，长度必须在 1~256 字节之间
/// - `folder_token`: 父文件夹的 token。参数为空字符串时，表示在根目录下创建文件夹
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::file::CreateFolderRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = CreateFolderRequest::new(config, "新建文件夹", "parent_token");
/// let response = request.execute().await?;
/// println!("新建文件夹 token: {}", response.token);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CreateFolderRequest {
    #[serde(skip)]
    config: Config,
    /// 文件夹名称
    pub name: String,
    /// 父文件夹的 token。参数为空字符串时，表示在根目录下创建文件夹
    pub folder_token: String,
}

/// 新建文件夹响应
///
/// 包含新建文件夹的基本信息。
///
/// # 字段说明
///
/// - `token`: 新建的文件夹的唯一标识符
/// - `url`: 新建的文件夹的 URL 链接，可直接在浏览器中打开
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderResponse {
    /// 新建的文件夹的 token
    pub token: String,
    /// 新建的文件夹的 URL 链接
    pub url: String,
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateFolderRequest {
    /// 创建新建文件夹请求
    ///
    /// # 参数
    ///
    /// - `config`: SDK 配置实例
    /// - `name`: 文件夹名称，长度必须在 1~256 字节之间
    /// - `folder_token`: 父文件夹 token，空字符串表示根目录
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let request = CreateFolderRequest::new(config, "我的文件夹", "parent_token");
    /// ```
    pub fn new(config: Config, name: impl Into<String>, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            name: name.into(),
            folder_token: folder_token.into(),
        }
    }

    /// 执行请求（使用默认选项）
    ///
    /// 发送新建文件夹请求到飞书服务器。
    ///
    /// # 返回
    ///
    /// 返回包含新建文件夹 token 和 URL 的响应
    ///
    /// # 错误
    ///
    /// - 名称长度不在 1~256 字节之间时返回校验错误
    /// - 网络错误、认证失败等返回相应的错误类型
    pub async fn execute(self) -> SDKResult<CreateFolderResponse> {
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
    ) -> SDKResult<CreateFolderResponse> {
        // 验证文件夹名称长度
        let name_len = self.name.len();
        if name_len == 0 || name_len > 256 {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 长度必须在 1~256 字节之间",
            ));
        }

        let api_endpoint = DriveApi::CreateFolder;
        let request = ApiRequest::<CreateFolderResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "新建文件夹")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件夹")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_folder_request() {
        let config = Config::default();
        let request = CreateFolderRequest::new(config, "test_folder", "parent_token");
        assert_eq!(request.name, "test_folder");
        assert_eq!(request.folder_token, "parent_token");
    }

    #[test]
    fn test_create_folder_with_empty_name() {
        let config = Config::default();
        let request = CreateFolderRequest::new(config, "", "parent_token");
        // 空名称应在 execute_with_options 时被校验拦截
        // 此测试验证请求结构可以构建，但执行时会失败
        assert_eq!(request.name, "");
    }

    #[test]
    fn test_create_folder_with_too_long_name() {
        let config = Config::default();
        let long_name = "a".repeat(257); // 超过256字节
        let request = CreateFolderRequest::new(config, long_name, "parent_token");
        // 过长名称应在 execute_with_options 时被校验拦截
        assert!(request.name.len() > 256);
    }

    #[test]
    fn test_create_folder_with_max_length_name() {
        let config = Config::default();
        // UTF-8 中每个中文字符占 3 字节，256 字节约 85 个中文字符
        let max_name = "a".repeat(256); // ASCII 字符，1 字节一个
        let request = CreateFolderRequest::new(config, max_name.clone(), "parent_token");
        assert_eq!(request.name, max_name);
        assert_eq!(request.name.len(), 256);
    }

    #[test]
    fn test_create_folder_in_root() {
        let config = Config::default();
        // 空字符串表示根目录
        let request = CreateFolderRequest::new(config, "test_folder", "");
        assert_eq!(request.folder_token, "");
    }
}
