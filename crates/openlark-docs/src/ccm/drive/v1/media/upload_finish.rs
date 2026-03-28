//! 分片上传素材-完成上传
//!
//! 上传分片全部完成后，调用该接口触发完成上传。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/media/multipart-upload-media/upload_finish>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 分片上传素材-完成上传请求
///
/// 上传分片全部完成后，调用该接口触发完成上传。
///
/// # 字段说明
///
/// - `upload_id`: 分片上传事务 ID，由预上传接口返回
/// - `block_num`: 分片数量，必须为正整数
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::ccm::drive::v1::media::upload_finish::UploadFinishMediaRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = UploadFinishMediaRequest::new(
///     config,
///     "upload_id_xyz",
///     10 // 分成了10片
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishMediaRequest {
    #[serde(skip)]
    config: Config,
    /// 分片上传事务 ID
    pub upload_id: String,
    /// 分片数量
    pub block_num: i32,
}

impl UploadFinishMediaRequest {
    pub fn new(config: Config, upload_id: impl Into<String>, block_num: i32) -> Self {
        Self {
            config,
            upload_id: upload_id.into(),
            block_num,
        }
    }

    pub async fn execute(self) -> SDKResult<UploadFinishMediaResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadFinishMediaResponse> {
        // === 必填字段验证 ===
        if self.upload_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "upload_id",
                "upload_id 不能为空",
            ));
        }

        // === 业务规则验证 ===
        if self.block_num <= 0 {
            return Err(openlark_core::error::validation_error(
                "block_num",
                "block_num 必须为正整数",
            ));
        }

        let api_endpoint = DriveApi::UploadMediaFinish;
        let request = ApiRequest::<UploadFinishMediaResponse>::post(&api_endpoint.to_url())
            .body(serialize_params(&self, "分片上传素材-完成上传")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "上传")
    }
}

/// 分片上传素材-完成上传响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFinishMediaResponse {
    /// 新创建文件的 token
    pub file_token: String,
}

impl ApiResponseTrait for UploadFinishMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_upload_finish_request_builder() {
        let config = Config::default();
        let request = UploadFinishMediaRequest::new(config, "upload_id", 10);
        assert_eq!(request.upload_id, "upload_id");
        assert_eq!(request.block_num, 10);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UploadFinishMediaResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_upload_id() {
        let config = Config::default();
        let request = UploadFinishMediaRequest::new(config, "", 10);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("upload_id"));
    }

    #[test]
    fn test_invalid_block_num_zero() {
        let config = Config::default();
        let request = UploadFinishMediaRequest::new(config, "upload_id", 0);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("block_num"));
    }

    #[test]
    fn test_invalid_block_num_negative() {
        let config = Config::default();
        let request = UploadFinishMediaRequest::new(config, "upload_id", -1);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("block_num"));
    }

    #[test]
    fn test_valid_block_num_boundary() {
        let config = Config::default();
        let request = UploadFinishMediaRequest::new(config, "upload_id", 1);

        let _rt = test_runtime();
        // 只验证请求构建成功，不实际执行
        assert_eq!(request.block_num, 1);
    }
}
