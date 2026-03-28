//! 更新云文档权限设置
//!
//! 更新指定云文档的公共访问与协作权限设置。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/permission/permission-public/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionPublic;

/// 更新云文档权限设置请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPublicRequest {
    /// 是否允许内容被分享到组织外
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_access: Option<bool>,
    /// 谁可以复制内容、创建副本、打印、下载
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_entity: Option<String>,
    /// 谁可以评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_entity: Option<String>,
    /// 谁可以查看、添加、移除协作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_entity: Option<String>,
    /// 链接分享设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_share_entity: Option<String>,
    /// 是否允许非「可管理权限」的人分享到组织外
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_external: Option<bool>,
}

/// 更新云文档权限设置请求
#[derive(Debug, Clone)]
pub struct PatchPublicPermissionRequest {
    config: Config,
    /// 云文档 token
    pub token: String,
    /// 云文档类型（需要与 token 匹配）
    pub r#type: String,
    /// 权限设置
    pub permission_public_request: PermissionPublicRequest,
}

impl PatchPublicPermissionRequest {
    pub fn new(
        config: Config,
        token: impl Into<String>,
        r#type: impl Into<String>,
        permission_public_request: PermissionPublicRequest,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            r#type: r#type.into(),
            permission_public_request,
        }
    }

    pub async fn execute(self) -> SDKResult<PatchPublicPermissionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchPublicPermissionResponse> {
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

        if let Some(security_entity) = &self.permission_public_request.security_entity {
            match security_entity.as_str() {
                "anyone_can_view" | "anyone_can_edit" | "only_full_access" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "security_entity",
                        "security_entity 必须为 anyone_can_view/anyone_can_edit/only_full_access",
                    ));
                }
            }
        }
        if let Some(comment_entity) = &self.permission_public_request.comment_entity {
            match comment_entity.as_str() {
                "anyone_can_view" | "anyone_can_edit" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "comment_entity",
                        "comment_entity 必须为 anyone_can_view/anyone_can_edit",
                    ));
                }
            }
        }
        if let Some(share_entity) = &self.permission_public_request.share_entity {
            match share_entity.as_str() {
                "anyone" | "same_tenant" | "only_full_access" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "share_entity",
                        "share_entity 必须为 anyone/same_tenant/only_full_access",
                    ));
                }
            }
        }
        if let Some(link_share_entity) = &self.permission_public_request.link_share_entity {
            match link_share_entity.as_str() {
                "tenant_readable" | "tenant_editable" | "anyone_readable" | "anyone_editable"
                | "closed" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "link_share_entity",
                        "link_share_entity 必须为 tenant_readable/tenant_editable/anyone_readable/anyone_editable/closed",
                    ));
                }
            }
        }

        // === 业务规则验证 ===
        if self.r#type == "wiki" {
            if self.permission_public_request.external_access.is_some() {
                return Err(openlark_core::error::validation_error(
                    "external_access",
                    "当 type=wiki 时不支持 external_access",
                ));
            }
            if self.permission_public_request.share_entity.is_some() {
                return Err(openlark_core::error::validation_error(
                    "share_entity",
                    "当 type=wiki 时不支持 share_entity",
                ));
            }
            if self.permission_public_request.invite_external.is_some() {
                return Err(openlark_core::error::validation_error(
                    "invite_external",
                    "当 type=wiki 时不支持 invite_external",
                ));
            }
            if matches!(
                self.permission_public_request.link_share_entity.as_deref(),
                Some("anyone_readable" | "anyone_editable")
            ) {
                return Err(openlark_core::error::validation_error(
                    "link_share_entity",
                    "当 type=wiki 时不支持 anyone_readable/anyone_editable",
                ));
            }
        }

        let api_endpoint = DriveApi::UpdatePublicPermission(self.token);
        let request = ApiRequest::<PatchPublicPermissionResponse>::patch(&api_endpoint.to_url())
            .query("type", self.r#type)
            .body(serialize_params(
                &self.permission_public_request,
                "更新云文档权限设置",
            )?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新")
    }
}

/// 更新云文档权限设置响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPublicPermissionResponse {
    /// 本次更新后的文档权限设置
    pub permission_public: PermissionPublic,
}

impl ApiResponseTrait for PatchPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_patch_public_permission_request_builder() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: Some(true),
            security_entity: Some("anyone_can_view".to_string()),
            comment_entity: Some("anyone_can_view".to_string()),
            share_entity: Some("anyone".to_string()),
            link_share_entity: Some("tenant_readable".to_string()),
            invite_external: Some(true),
        };

        let request = PatchPublicPermissionRequest::new(config, "doc_token", "docx", body);
        assert_eq!(request.token, "doc_token");
        assert_eq!(request.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            PatchPublicPermissionResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: Some(true),
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "", "docx", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("token"));
    }

    #[test]
    fn test_empty_type() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: Some(true),
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "doc_token", "", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("type"));
    }

    #[test]
    fn test_invalid_type() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: None,
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "doc_token", "invalid_type", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("type"));
    }

    #[test]
    fn test_invalid_security_entity() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: None,
            security_entity: Some("invalid_entity".to_string()),
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "doc_token", "docx", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("security_entity"));
    }

    #[test]
    fn test_wiki_with_external_access() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: Some(true),
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: None,
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "wiki_token", "wiki", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("external_access"));
    }

    #[test]
    fn test_wiki_with_anyone_readable() {
        let config = Config::default();
        let body = PermissionPublicRequest {
            external_access: None,
            security_entity: None,
            comment_entity: None,
            share_entity: None,
            link_share_entity: Some("anyone_readable".to_string()),
            invite_external: None,
        };

        let request = PatchPublicPermissionRequest::new(config, "wiki_token", "wiki", body);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("link_share_entity"));
    }
}
