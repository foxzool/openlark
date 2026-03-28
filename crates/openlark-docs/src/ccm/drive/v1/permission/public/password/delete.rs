//! 停用云文档密码
//!
//! 该接口用于根据 filetoken 停用云文档密码。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/permission/permission-public/permission-public-password/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordRequest {
    #[serde(skip)]
    config: Config,
    pub token: String,
    pub r#type: String,
}

impl DeletePermissionPublicPasswordRequest {
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeletePermissionPublicPasswordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeletePermissionPublicPasswordResponse> {
        // === 必填字段验证 ===
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "type",
                "type 不能为空",
            ));
        }

        // === 枚举值验证 ===
        match self.r#type.as_str() {
            "doc" | "sheet" | "file" | "wiki" | "bitable" | "docx" | "mindnote" | "minutes"
            | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 必须为 doc/sheet/file/wiki/bitable/docx/mindnote/minutes/slides",
                ));
            }
        }

        // === 业务规则验证 ===
        if self.r#type == "minutes" {
            return Err(openlark_core::error::validation_error(
                "type",
                "type=minutes 暂不支持停用云文档密码",
            ));
        }

        let api_endpoint = DriveApi::DeletePublicPassword(self.token);
        let api_request: ApiRequest<DeletePermissionPublicPasswordResponse> =
            ApiRequest::delete(&api_endpoint.to_url()).query("type", self.r#type);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionPublicPasswordResponse {
    // Empty response body usually implies success
}

impl ApiResponseTrait for DeletePermissionPublicPasswordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_delete_permission_public_password_request_builder() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "token", "docx");
        assert_eq!(request.token, "token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DeletePermissionPublicPasswordResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "", "docx");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("token"));
    }

    #[test]
    fn test_empty_type() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "token", "");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("type"));
    }

    #[test]
    fn test_invalid_type() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "token", "invalid_type");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("type"));
    }

    #[test]
    fn test_minutes_not_supported() {
        let config = Config::default();
        let request = DeletePermissionPublicPasswordRequest::new(config, "token", "minutes");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("minutes"));
    }
}
