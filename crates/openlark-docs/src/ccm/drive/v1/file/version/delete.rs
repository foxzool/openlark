//! 删除文档版本
//!
//! 删除指定源文档的指定版本。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 删除文档版本请求
#[derive(Debug, Clone)]
pub struct DeleteFileVersionRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 版本文档的版本标识
    pub version_id: String,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl DeleteFileVersionRequest {
    /// 创建新的文档版本删除请求。
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        version_id: impl Into<String>,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            version_id: version_id.into(),
            obj_type: obj_type.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeleteFileVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteFileVersionResponse> {
        // ========== 参数校验 ==========

        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.version_id, "version_id 不能为空");
        match self.obj_type.as_str() {
            "docx" | "sheet" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "obj_type",
                    "obj_type 仅支持 docx/sheet",
                ));
            }
        }

        // ========== 构建 API 请求 ==========
        let api_endpoint = DriveApi::DeleteFileVersion(self.file_token, self.version_id);
        let request = ApiRequest::<DeleteFileVersionResponse>::delete(&api_endpoint.to_url())
            .query("obj_type", self.obj_type)
            .query_opt("user_id_type", self.user_id_type);

        // ========== 发送请求并返回响应 ==========
        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除")
    }
}

/// 删除文档版本响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteFileVersionResponse {}

impl ApiResponseTrait for DeleteFileVersionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_file_version_request_builder() {
        let config = Config::default();
        let request = DeleteFileVersionRequest::new(config, "file_token", "fnJfyX", "docx")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.version_id, "fnJfyX");
        assert_eq!(request.obj_type, "docx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DeleteFileVersionResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_delete_file_version_request_empty_fields() {
        let config = Config::default();
        let request = DeleteFileVersionRequest::new(config.clone(), "", "fnJfyX", "docx");
        assert!(request.file_token.is_empty());

        let request2 = DeleteFileVersionRequest::new(config, "token", "", "docx");
        assert!(request2.version_id.is_empty());
    }
}

