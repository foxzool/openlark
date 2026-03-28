//! 分片上传文件-上传分片
//!
//! 根据预上传接口返回的 upload_id 和分片策略上传对应的文件分片。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/upload/multipart-upload-file-/upload_part>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 上传分片请求
///
/// 根据预上传接口返回的 upload_id 和分片策略上传对应的文件分片。
///
/// # 字段说明
///
/// - `upload_id`: 上传会话ID，由预上传接口返回
/// - `seq`: 分片序号，从 0 开始
/// - `size`: 分片大小（字节），必须在 1~4194304 之间
/// - `checksum`: 分片的 Adler-32 校验和（可选）
/// - `file`: 分片数据，长度必须与 size 一致
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::file::upload_part::UploadPartRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let file_data = vec![0u8; 1024];
/// let request = UploadPartRequest::new(
///     config,
///     "upload_id_xyz",
///     0, // 第一片
///     1024,
///     file_data
/// )
/// .checksum("3248270248");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartRequest {
    #[serde(skip)]
    config: Config,
    /// 上传会话ID
    pub upload_id: String,
    /// 分片序号
    pub seq: i32,
    /// 分片大小
    pub size: i32,
    /// 分片的 Adler-32 校验和
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// 分片数据
    #[serde(skip)]
    pub file: Vec<u8>,
}

impl UploadPartRequest {
    pub fn new(
        config: Config,
        upload_id: impl Into<String>,
        seq: i32,
        size: i32,
        file: Vec<u8>,
    ) -> Self {
        Self {
            config,
            upload_id: upload_id.into(),
            seq,
            size,
            checksum: None,
            file,
        }
    }

    /// 设置分片校验和（Adler-32）
    pub fn checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UploadPartResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadPartResponse> {
        // === 必填字段验证 ===
        if self.upload_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "upload_id",
                "upload_id 不能为空",
            ));
        }

        // === 业务规则验证 ===
        if self.seq < 0 {
            return Err(openlark_core::error::validation_error(
                "seq",
                "seq 不能为负数",
            ));
        }
        if self.size <= 0 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须为正整数",
            ));
        }
        // 平台固定以 4MB 大小进行分片（最后一片可小于 4MB）
        if self.size > 4 * 1024 * 1024 {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 不能超过 4194304 字节(4MB)",
            ));
        }
        if self.file.len() != self.size as usize {
            return Err(openlark_core::error::validation_error(
                "size",
                "size 必须与 file 的实际长度一致",
            ));
        }

        let api_endpoint = DriveApi::UploadPart;

        #[derive(Serialize)]
        struct PartMeta {
            upload_id: String,
            seq: i32,
            size: i32,
            #[serde(skip_serializing_if = "Option::is_none")]
            checksum: Option<String>,
        }

        let meta = PartMeta {
            upload_id: self.upload_id,
            seq: self.seq,
            size: self.size,
            checksum: self.checksum,
        };

        let request = ApiRequest::<UploadPartResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "分片上传文件-上传分片")
    }
}

/// 上传分片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartResponse {}

impl ApiResponseTrait for UploadPartResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_upload_part_request_builder() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "upload_id", 1, 1024, vec![0; 1024]);
        assert_eq!(request.upload_id, "upload_id");
        assert_eq!(request.seq, 1);
        assert_eq!(request.size, 1024);
        assert_eq!(request.file.len(), 1024);
    }

    #[test]
    fn test_empty_upload_id() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "", 1, 1024, vec![0; 1024]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("upload_id"));
    }

    #[test]
    fn test_negative_seq() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "upload_id", -1, 1024, vec![0; 1024]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("seq"));
    }

    #[test]
    fn test_invalid_size_zero() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "upload_id", 1, 0, vec![]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("size"));
    }

    #[test]
    fn test_invalid_size_too_large() {
        let config = Config::default();
        let request = UploadPartRequest::new(
            config,
            "upload_id",
            1,
            4 * 1024 * 1024 + 1,
            vec![0; 4 * 1024 * 1024 + 1],
        );

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("4194304"));
    }

    #[test]
    fn test_size_mismatch() {
        let config = Config::default();
        let request = UploadPartRequest::new(config, "upload_id", 1, 1024, vec![0; 512]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("一致"));
    }
}
