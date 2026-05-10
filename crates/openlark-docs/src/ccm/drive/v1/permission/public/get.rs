//! 获取云文档权限设置
//!
//! 获取指定云文档的公共访问与协作权限设置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-public/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionPublic;

/// 获取云文档权限设置请求
///
/// 用于获取指定云文档的公共访问与协作权限设置。
///
/// # 字段说明
///
/// - `token`: 云文档 token，不能为空
/// - `type`: 云文档类型，必须为 doc/sheet/file/wiki/bitable/docx/mindnote/minutes/slides 之一
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::permission::public::GetPublicPermissionRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = GetPublicPermissionRequest::new(config, "doc_token", "docx");
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct GetPublicPermissionRequest {
    config: Config,
    /// 云文档 token
    pub token: String,
    /// 云文档类型（需要与 token 匹配）
    pub r#type: String,
}

impl GetPublicPermissionRequest {
    /// 创建新的权限查询请求。
    pub fn new(config: Config, token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetPublicPermissionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetPublicPermissionResponse> {
        // === 必填字段验证 ===
        validate_required!(self.token, "token 不能为空");
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

        let api_endpoint = DriveApi::GetPublicPermission(self.token);
        let request = ApiRequest::<GetPublicPermissionResponse>::get(&api_endpoint.to_url())
            .query("type", self.r#type);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

/// 获取云文档权限设置响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicPermissionResponse {
    /// 返回的文档公共访问和协作权限设置
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_public_permission_request_builder() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "doc_token", "docx");

        assert_eq!(request.token, "doc_token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetPublicPermissionResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "", "docx");
        assert_eq!(request.token, "");
    }

    #[test]
    fn test_empty_type_validation() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "doc_token", "");
        assert_eq!(request.r#type, "");
    }

    #[test]
    fn test_invalid_type_validation() {
        let config = Config::default();
        let request = GetPublicPermissionRequest::new(config, "doc_token", "invalid_type");
        assert_eq!(request.r#type, "invalid_type");
    }

    #[test]
    fn test_valid_file_types() {
        let config = Config::default();
        let valid_types = vec![
            "doc", "sheet", "file", "wiki", "bitable", "docx", "mindnote", "minutes", "slides",
        ];

        for file_type in valid_types {
            let request = GetPublicPermissionRequest::new(config.clone(), "doc_token", file_type);
            assert_eq!(request.r#type, file_type);
        }
    }
}

