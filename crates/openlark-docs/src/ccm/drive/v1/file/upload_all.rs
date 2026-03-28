//! 上传文件（一次性上传）
//!
//! 上传指定文件到指定目录中，支持单次上传文件。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/upload_all>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传文件请求
///
/// 用于一次性上传指定文件到指定目录中。适用于小文件上传（最大 20MB）。
///
/// # 字段说明
///
/// - `file_name`: 文件名，长度必须在 1~250 字符之间
/// - `parent_node`: 父文件夹 token，不能为空
/// - `parent_type`: 父节点类型，目前仅支持固定值 "explorer"
/// - `size`: 文件大小，必须在 1~20971520 字节之间
/// - `checksum`: 文件的 Adler-32 校验和（可选）
/// - `file`: 文件内容，字节数组
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::file::UploadAllRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let file_content = std::fs::read("test.txt")?;
/// let request = UploadAllRequest::new(
///     config,
///     "test.txt",
///     "folder_token",
///     "explorer",
///     file_content.len(),
///     file_content,
/// );
/// let response = request.execute().await?;
/// println!("文件 token: {}", response.file_token);
/// ```
#[derive(Debug)]
pub struct UploadAllRequest {
    config: Config,
    /// 文件名
    pub file_name: String,
    /// 父文件夹token
    pub parent_node: String,
    /// 父节点类型: explorer (文件夹)
    pub parent_type: String,
    /// 文件大小
    pub size: usize,
    /// 文件的 Adler-32 校验和
    pub checksum: Option<String>,
    /// 文件内容
    pub file: Vec<u8>,
}

/// 上传文件响应
///
/// 包含上传成功后的文件标识信息。
///
/// # 字段说明
///
/// - `file_token`: 上传成功后生成的文件唯一标识符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllResponse {
    /// 文件token
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UploadAllRequest {
    /// 创建上传文件请求
    ///
    /// # 参数
    ///
    /// - `config`: SDK 配置实例
    /// - `file_name`: 文件名，长度必须在 1~250 字符之间
    /// - `parent_node`: 父文件夹 token，不能为空
    /// - `parent_type`: 父节点类型，目前仅支持 "explorer"
    /// - `size`: 文件大小（字节）
    /// - `file`: 文件内容
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_node: impl Into<String>,
        parent_type: impl Into<String>,
        size: usize,
        file: Vec<u8>,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_node: parent_node.into(),
            parent_type: parent_type.into(),
            size,
            checksum: None,
            file,
        }
    }

    /// 设置文件校验和（Adler-32）
    ///
    /// 用于验证文件完整性的可选校验和。
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// 执行请求（使用默认选项）
    ///
    /// 发送文件上传请求到飞书服务器。
    pub async fn execute(self) -> SDKResult<UploadAllResponse> {
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
    ) -> SDKResult<UploadAllResponse> {
        // 验证文件名长度
        let file_name_len = self.file_name.chars().count();
        if file_name_len == 0 || file_name_len > 250 {
            return Err(openlark_core::error::validation_error(
                "file_name",
                "file_name 长度必须在 1~250 字符之间",
            ));
        }

        // 验证父节点类型
        if self.parent_type != "explorer" {
            return Err(openlark_core::error::validation_error(
                "parent_type",
                "parent_type 仅支持固定值 explorer",
            ));
        }

        // 验证父节点 token (必填)
        if self.parent_node.is_empty() {
            return Err(openlark_core::error::validation_error(
                "parent_node",
                "parent_node 不能为空",
            ));
        }

        // 验证文件大小范围
        if self.size == 0 || self.size > 20 * 1024 * 1024 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须在 1~20971520 字节之间",
            ));
        }

        // 验证文件大小与实际内容一致
        if self.file.len() != self.size {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须与 file 的实际长度一致",
            ));
        }

        let api_endpoint = DriveApi::UploadFile;

        // 构建 multipart 所需的元数据
        #[derive(Serialize)]
        struct UploadMeta {
            file_name: String,
            parent_type: String,
            parent_node: String,
            size: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
        }

        let meta = UploadMeta {
            file_name: self.file_name,
            parent_type: self.parent_type,
            parent_node: self.parent_node,
            size: self.size,
            checksum: self.checksum,
        };

        // 使用 json_body 传递元数据，使用 file_content 传递文件
        // UnifiedRequestBuilder 会将其组合成 multipart 请求
        let request = ApiRequest::<UploadAllResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传文件")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_all_request() {
        let config = Config::default();
        let request =
            UploadAllRequest::new(config, "test.txt", "folder_token", "explorer", 100, vec![]);
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.parent_node, "folder_token");
        assert_eq!(request.parent_type, "explorer");
    }

    #[test]
    fn test_upload_all_with_checksum() {
        let config = Config::default();
        let file = vec![1, 2, 3, 4];
        let request = UploadAllRequest::new(
            config,
            "test.txt",
            "folder_token",
            "explorer",
            4,
            file.clone(),
        )
        .checksum("abc123");
        assert_eq!(request.checksum, Some("abc123".to_string()));
        assert_eq!(request.file.len(), 4);
    }

    #[test]
    fn test_upload_all_empty_file_name() {
        let config = Config::default();
        let request = UploadAllRequest::new(config, "", "folder_token", "explorer", 0, vec![]);
        assert_eq!(request.file_name, "");
        // 空文件名应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_upload_all_invalid_parent_type() {
        let config = Config::default();
        let request =
            UploadAllRequest::new(config, "test.txt", "folder_token", "invalid", 100, vec![]);
        assert_eq!(request.parent_type, "invalid");
        // 无效的 parent_type 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_upload_all_empty_parent_node() {
        let config = Config::default();
        let request = UploadAllRequest::new(config, "test.txt", "", "explorer", 100, vec![]);
        assert_eq!(request.parent_node, "");
        // 空 parent_node 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_upload_all_max_file_size() {
        let config = Config::default();
        let max_size = 20 * 1024 * 1024; // 20MB
        let file = vec![0; max_size];
        let request = UploadAllRequest::new(
            config,
            "test.txt",
            "folder_token",
            "explorer",
            max_size,
            file,
        );
        assert_eq!(request.size, max_size);
        assert_eq!(request.file.len(), max_size);
    }

    #[test]
    fn test_upload_all_exceeds_max_size() {
        let config = Config::default();
        let too_large = 20 * 1024 * 1024 + 1; // 超过20MB
        let request = UploadAllRequest::new(
            config,
            "test.txt",
            "folder_token",
            "explorer",
            too_large,
            vec![],
        );
        assert!(request.size > 20 * 1024 * 1024);
        // 超大文件应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_upload_all_size_mismatch() {
        let config = Config::default();
        let file = vec![1, 2, 3]; // 实际3字节
        let request =
            UploadAllRequest::new(config, "test.txt", "folder_token", "explorer", 100, file);
        assert_eq!(request.size, 100);
        assert_eq!(request.file.len(), 3);
        // 大小不匹配应在 execute_with_options 时被校验拦截
    }
}
