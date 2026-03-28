//! 分片上传文件-预上传
//!
//! 发送初始化请求，以获取上传事务 ID 和分片策略，为上传分片做准备。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_prepare>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传预备请求
///
/// 用于初始化分片上传，获取上传事务 ID 和分片策略。
///
/// # 字段说明
///
/// - `file_name`: 文件名，长度不能超过 250 字符
/// - `parent_type`: 上传点的类型，固定为 "explorer"（云空间）
/// - `parent_node`: 云空间中文件夹的 token
/// - `size`: 文件大小（字节），不能为负数
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::file::upload_prepare::UploadPrepareRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = UploadPrepareRequest::new(
///     config,
///     "large_file.zip",
///     "folder_token_xyz",
///     104857600 // 100MB
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareRequest {
    #[serde(skip)]
    config: Config,
    /// 文件名
    pub file_name: String,
    /// 上传点的类型：固定为 explorer（云空间）
    pub parent_type: String,
    /// 云空间中文件夹的 token
    pub parent_node: String,
    /// 文件大小（字节）
    pub size: i64,
}

impl UploadPrepareRequest {
    pub fn new(
        config: Config,
        file_name: impl Into<String>,
        parent_node: impl Into<String>,
        size: i64,
    ) -> Self {
        Self {
            config,
            file_name: file_name.into(),
            parent_type: "explorer".to_string(),
            parent_node: parent_node.into(),
            size,
        }
    }

    /// 覆盖上传点类型（文档固定为 explorer，一般无需设置）
    pub fn parent_type(mut self, parent_type: impl Into<String>) -> Self {
        self.parent_type = parent_type.into();
        self
    }

    pub async fn execute(self) -> SDKResult<UploadPrepareResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadPrepareResponse> {
        // === 必填字段验证 ===
        validate_required!(self.file_name.trim(), "file_name 不能为空");
        validate_required!(self.parent_node.trim(), "parent_node 不能为空");

        // === 业务规则验证 ===
        // 自定义验证逻辑
        let file_name_len = self.file_name.chars().count();
        if file_name_len > 250 {
            return Err(openlark_core::error::validation_error(
                "file_name",
                "file_name 长度不能超过 250 字符",
            ));
        }

        if self.parent_type != "explorer" {
            return Err(openlark_core::error::validation_error(
                "parent_type",
                "parent_type 仅支持固定值 explorer",
            ));
        }

        if self.size < 0 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 不能为负数",
            ));
        }

        let api_endpoint = DriveApi::UploadPrepare;
        let request = ApiRequest::<UploadPrepareResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "分片上传文件-预上传")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "分片上传文件-预上传")
    }
}

/// 分片上传预备响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPrepareResponse {
    /// 上传会话ID
    pub upload_id: String,
    /// 分片大小（固定为 4MB）
    pub block_size: i32,
    /// 分片数量
    pub block_num: i32,
}

impl ApiResponseTrait for UploadPrepareResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_upload_prepare_request_builder() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "test.txt", "folder_token", 1024);
        assert_eq!(request.file_name, "test.txt");
        assert_eq!(request.parent_node, "folder_token");
        assert_eq!(request.size, 1024);
    }

    #[test]
    fn test_empty_file_name() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "", "folder_token", 1024);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("file_name"));
    }

    #[test]
    fn test_empty_parent_node() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "test.txt".to_string(), "", 1024);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("parent_node"));
    }

    #[test]
    fn test_file_name_too_long() {
        let config = Config::default();
        let long_name = "a".repeat(251);
        let request = UploadPrepareRequest::new(config, long_name, "folder_token", 1024);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("250"));
    }

    #[test]
    fn test_negative_size() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "test.txt", "folder_token", -1);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("size"));
    }

    #[test]
    fn test_invalid_parent_type() {
        let config = Config::default();
        let request = UploadPrepareRequest::new(config, "test.txt", "folder_token", 1024)
            .parent_type("invalid_type");

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("explorer"));
    }
}
