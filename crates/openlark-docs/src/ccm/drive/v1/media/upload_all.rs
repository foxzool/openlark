//! 上传素材
//!
//! 将文件、图片、视频等素材文件上传到指定云文档中。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/upload_all

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传素材请求
///
/// 将文件、图片、视频等素材文件上传到指定云文档中。
///
/// # 字段说明
///
/// - `file_name`: 要上传的素材的名称，长度必须在 1~250 字符之间
/// - `parent_type`: 上传点的类型，支持的值包括：
///   - `doc_image`, `docx_image`, `sheet_image`, `bitable_image` - 图片类型
///   - `doc_file`, `docx_file`, `sheet_file`, `bitable_file` - 文件类型
///   - `vc_virtual_background` - 虚拟背景
///   - `moments` - 动态
///   - `ccm_import_open` - 导入
/// - `parent_node`: 上传点的 token
/// - `size`: 文件大小（字节），必须在 1~20971520 之间（最大20MB）
/// - `checksum`: 文件的 Adler-32 校验和（可选）
/// - `extra`: 拓展参数（可选）
/// - `file`: 文件二进制内容，长度必须与 size 一致
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::media::upload_all::UploadAllMediaRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let file_data = vec![0u8; 100];
/// let request = UploadAllMediaRequest::new(
///     config,
///     "test.png",
///     "docx_image",
///     "doc_token",
///     100,
///     file_data
/// )
/// .extra("extra_info");
/// ```
#[derive(Debug)]
pub struct UploadAllMediaRequest {
    config: Config,
    /// 要上传的素材的名称
    pub file_name: String,
    /// 上传点的类型
    pub parent_type: String,
    /// 上传点的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: usize,
    /// 文件的 Adler-32 校验和
    pub checksum: Option<String>,
    /// 拓展参数
    pub extra: Option<String>,
    /// 文件二进制内容
    pub file: Vec<u8>,
}

impl UploadAllMediaRequest {
    /// 创建新的素材上传请求。
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
        parent_node: impl Into<String>,
        size: usize,
        file: Vec<u8>,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: parent_type.into(),
            parent_node: parent_node.into(),
            size,
            checksum: None,
            extra: None,
            file,
        }
    }

    /// 设置文件校验和（Adler-32）
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    /// 设置拓展参数
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UploadAllMediaResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadAllMediaResponse> {
        // === 必填字段验证 ===
        let file_name_len = self.file_name.chars().count();
        if file_name_len == 0 || file_name_len > 250 {
            return Err(openlark_core::error::validation_error(
                "file_name",
                "file_name 长度必须在 1~250 字符之间",
            ));
        }

        validate_required!(self.parent_node, "parent_node 不能为空");

        // === 枚举值验证 ===
        match self.parent_type.as_str() {
            "doc_image"
            | "docx_image"
            | "sheet_image"
            | "doc_file"
            | "docx_file"
            | "sheet_file"
            | "vc_virtual_background"
            | "bitable_image"
            | "bitable_file"
            | "moments"
            | "ccm_import_open" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "parent_type",
                    "parent_type 不在支持的取值范围内",
                ));
            }
        }

        // === 业务规则验证 ===
        if self.size == 0 || self.size > 20 * 1024 * 1024 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须在 1~20971520 字节之间",
            ));
        }
        if self.file.len() != self.size {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须与 file 的实际长度一致",
            ));
        }

        let api_endpoint = DriveApi::UploadMedia;

        #[derive(Serialize)]
        struct UploadMeta {
            file_name: String,
            parent_type: String,
            parent_node: String,
            size: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<String>,
        }

        let meta = UploadMeta {
            file_name: self.file_name,
            parent_type: self.parent_type,
            parent_node: self.parent_node,
            size: self.size,
            checksum: self.checksum,
            extra: self.extra,
        };

        let request = ApiRequest::<UploadAllMediaResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传")
    }
}

/// 上传素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAllMediaResponse {
    /// 素材文件的 token
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let request = UploadAllMediaRequest::new(
            config,
            "test.png",
            "docx_image",
            "doc_token",
            100,
            vec![0; 100],
        )
        .extra("extra_info");

        assert_eq!(request.file_name, "test.png");
        assert_eq!(request.parent_type, "docx_image");
        assert_eq!(request.extra, Some("extra_info".to_string()));
    }

    #[test]
    fn test_empty_file_name() {
        let config = Config::default();
        let request =
            UploadAllMediaRequest::new(config, "", "docx_image", "doc_token", 100, vec![0; 100]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("file_name"));
    }

    #[test]
    fn test_file_name_too_long() {
        let config = Config::default();
        let long_name = "a".repeat(251);
        let request = UploadAllMediaRequest::new(
            config,
            long_name,
            "docx_image",
            "doc_token",
            100,
            vec![0; 100],
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("250"));
    }

    #[test]
    fn test_empty_parent_node() {
        let config = Config::default();
        let request =
            UploadAllMediaRequest::new(config, "test.png", "docx_image", "", 100, vec![0; 100]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parent_node"));
    }

    #[test]
    fn test_invalid_parent_type() {
        let config = Config::default();
        let request = UploadAllMediaRequest::new(
            config,
            "test.png",
            "invalid_type",
            "doc_token",
            100,
            vec![0; 100],
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parent_type"));
    }

    #[test]
    fn test_invalid_size_zero() {
        let config = Config::default();
        let request =
            UploadAllMediaRequest::new(config, "test.png", "docx_image", "doc_token", 0, vec![]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("size"));
    }

    #[test]
    fn test_invalid_size_too_large() {
        let config = Config::default();
        let request = UploadAllMediaRequest::new(
            config,
            "test.png",
            "docx_image",
            "doc_token",
            20 * 1024 * 1024 + 1,
            vec![0; 20 * 1024 * 1024 + 1],
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("20971520"));
    }

    #[test]
    fn test_size_mismatch() {
        let config = Config::default();
        let request = UploadAllMediaRequest::new(
            config,
            "test.png",
            "docx_image",
            "doc_token",
            100,
            vec![0; 50],
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("一致"));
    }
}

