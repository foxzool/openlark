//! 分片上传素材-预上传
//!
//! 发送初始化请求，以获取上传事务 ID 和分片策略，为上传分片做准备。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_prepare

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传素材-预上传请求
///
/// 发送初始化请求，以获取上传事务 ID 和分片策略，为上传分片做准备。
///
/// # 字段说明
///
/// - `file_name`: 素材的文件名称，长度必须在 1~250 字符之间
/// - `parent_type`: 上传点的类型，支持的值包括：
///   - `doc_image`, `docx_image`, `sheet_image`, `bitable_image` - 图片类型
///   - `doc_file`, `docx_file`, `sheet_file`, `bitable_file` - 文件类型
///   - `vc_virtual_background` - 虚拟背景
///   - `moments` - 动态
///   - `ccm_import_open` - 导入
/// - `parent_node`: 上传点的 token
/// - `size`: 文件大小（字节），不能为负数
/// - `extra`: 拓展参数（可选）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::media::upload_prepare::UploadPrepareMediaRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = UploadPrepareMediaRequest::new(
///     config,
///     "demo.jpeg",
///     "docx_image",
///     "doccnFivLCfJfblZjGZtxgabcef",
///     1024
/// )
/// .extra("{\"drive_route_token\":\"doxcnXgNGAtaAraIRVeCfmabcef\"}");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareMediaRequest {
    #[serde(skip)]
    config: Config,
    /// 素材的文件名称
    pub file_name: String,
    /// 上传点的类型
    pub parent_type: String,
    /// 上传点的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: i64,
    /// 拓展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

impl UploadPrepareMediaRequest {
    /// 创建新的预上传请求。
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_type: impl Into<String>,
        parent_node: impl Into<String>,
        size: i64,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: parent_type.into(),
            parent_node: parent_node.into(),
            size,
            extra: None,
        }
    }

    /// 设置扩展参数。
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UploadPrepareMediaResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadPrepareMediaResponse> {
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
        if self.size < 0 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 不能为负数",
            ));
        }

        let api_endpoint = DriveApi::UploadMediaPrepare;
        let request = ApiRequest::<UploadPrepareMediaResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "分片上传素材-预上传")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传")
    }
}

/// 分片上传素材准备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareMediaResponse {
    /// 分片上传事务 ID
    pub upload_id: String,
    /// 分片大小策略
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_upload_prepare_request() {
        let config = Config::default();
        let request = UploadPrepareMediaRequest::new(
            config,
            "demo.jpeg",
            "docx_image",
            "doccnFivLCfJfblZjGZtxgabcef",
            1024,
        )
        .extra("{\"drive_route_token\":\"doxcnXgNGAtaAraIRVeCfmabcef\"}");

        assert_eq!(request.file_name, "demo.jpeg");
        assert_eq!(request.parent_type, "docx_image");
        assert_eq!(request.size, 1024);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UploadPrepareMediaResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_file_name() {
        let config = Config::default();
        let request = UploadPrepareMediaRequest::new(
            config,
            "",
            "docx_image",
            "doccnFivLCfJfblZjGZtxgabcef",
            1024,
        );

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
        let request = UploadPrepareMediaRequest::new(
            config,
            long_name,
            "docx_image",
            "doccnFivLCfJfblZjGZtxgabcef",
            1024,
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
        let request = UploadPrepareMediaRequest::new(config, "demo.jpeg", "docx_image", "", 1024);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parent_node"));
    }

    #[test]
    fn test_invalid_parent_type() {
        let config = Config::default();
        let request = UploadPrepareMediaRequest::new(
            config,
            "demo.jpeg",
            "invalid_type",
            "doccnFivLCfJfblZjGZtxgabcef",
            1024,
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parent_type"));
    }

    #[test]
    fn test_negative_size() {
        let config = Config::default();
        let request = UploadPrepareMediaRequest::new(
            config,
            "demo.jpeg",
            "docx_image",
            "doccnFivLCfJfblZjGZtxgabcef",
            -1,
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("size"));
    }

    #[test]
    fn test_valid_parent_types() {
        let valid_types = vec![
            "doc_image",
            "docx_image",
            "sheet_image",
            "bitable_image",
            "doc_file",
            "docx_file",
            "sheet_file",
            "bitable_file",
            "vc_virtual_background",
            "moments",
            "ccm_import_open",
        ];

        for parent_type in valid_types {
            let config = Config::default();
            let request = UploadPrepareMediaRequest::new(
                config,
                "demo.jpeg",
                parent_type.to_string(),
                "doccnFivLCfJfblZjGZtxgabcef",
                1024,
            );

            assert_eq!(request.parent_type, parent_type);
        }
    }
}

