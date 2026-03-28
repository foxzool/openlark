//! 分片上传素材-上传分片
//!
//! 根据预上传接口返回的 upload_id 和分片策略上传对应的素材分片。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_part>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传素材-上传分片请求
///
/// 根据预上传接口返回的 upload_id 和分片策略上传对应的素材分片。
///
/// # 字段说明
///
/// - `upload_id`: 分片上传事务的 ID，由预上传接口返回
/// - `seq`: 块号，从 0 开始计数，不能为负数
/// - `size`: 块大小（字节），必须在 1~4194304 之间
/// - `checksum`: 素材文件的 Adler-32 校验和（可选）
/// - `file`: 素材文件分片的二进制内容，长度必须与 size 一致
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::media::upload_part::UploadPartMediaRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let file_data = vec![0u8; 1024];
/// let request = UploadPartMediaRequest::new(
///     config,
///     "upload_id_xyz",
///     0, // 第一片
///     1024,
///     file_data
/// )
/// .checksum("3248270248");
/// ```
#[derive(Debug)]
pub struct UploadPartMediaRequest {
    config: Config,
    /// 分片上传事务的 ID
    pub upload_id: String,
    /// 块号，从 0 开始计数
    pub seq: i32,
    /// 块大小（字节）
    pub size: i32,
    /// 素材文件的 Adler-32 校验和
    pub checksum: Option<String>,
    /// 素材文件分片的二进制内容
    pub file: Vec<u8>,
}

impl UploadPartMediaRequest {
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

    pub async fn execute(self) -> SDKResult<UploadPartMediaResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadPartMediaResponse> {
        // === 必填字段验证 ===
        if self.upload_id.is_empty() {
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

        let api_endpoint = DriveApi::UploadMediaPart;

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

        let request = ApiRequest::<UploadPartMediaResponse>::post(&api_endpoint.to_url())
            .json_body(&meta)
            .file_content(self.file);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传")
    }
}

/// 分片上传素材-上传分片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadPartMediaResponse {}

impl ApiResponseTrait for UploadPartMediaResponse {
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
        let request = UploadPartMediaRequest::new(config, "upload_id", 0, 1024, vec![0; 1024])
            .checksum("3248270248");

        assert_eq!(request.upload_id, "upload_id");
        assert_eq!(request.seq, 0);
        assert_eq!(request.size, 1024);
        assert_eq!(request.file.len(), 1024);
        assert_eq!(request.checksum, Some("3248270248".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UploadPartMediaResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_empty_upload_id() {
        let config = Config::default();
        let request = UploadPartMediaRequest::new(config, "", 0, 1024, vec![0; 1024]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("upload_id"));
    }

    #[test]
    fn test_negative_seq() {
        let config = Config::default();
        let request = UploadPartMediaRequest::new(config, "upload_id", -1, 1024, vec![0; 1024]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("seq"));
    }

    #[test]
    fn test_invalid_size_zero() {
        let config = Config::default();
        let request = UploadPartMediaRequest::new(config, "upload_id", 0, 0, vec![]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("size"));
    }

    #[test]
    fn test_invalid_size_too_large() {
        let config = Config::default();
        let request = UploadPartMediaRequest::new(
            config,
            "upload_id",
            0,
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
        let request = UploadPartMediaRequest::new(config, "upload_id", 0, 1024, vec![0; 512]);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("一致"));
    }
}
