//! 移除云文档协作者权限
//!
//! 移除文件或文件夹中指定协作者的权限。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/delete

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 移除协作者权限请求
///
/// 用于移除文件或文件夹中指定协作者的权限。
///
/// # 字段说明
///
/// - `token`: 文件 token，不能为空
/// - `member_id`: 成员 ID，不能为空
/// - `file_type`: 云文档类型，必须为 doc/sheet/file/wiki/bitable/docx/folder/mindnote/minutes/slides 之一
/// - `member_type`: 协作者 ID 类型，必须为 email/openid/unionid/openchat/opendepartmentid/userid/groupid/wikispaceid 之一
/// - `perm_type`: 权限角色类型，可选值为 container/single_page
/// - `member_kind`: 协作者类型，当 member_type 为 wikispaceid 时通常需要指定
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::permission::member::DeletePermissionMemberRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = DeletePermissionMemberRequest::new(
///     config,
///     "file_token",
///     "member_id",
///     "docx",
///     "openid",
/// )
/// .perm_type("container");
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct DeletePermissionMemberRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员ID
    pub member_id: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 协作者 ID 类型（query 参数 `member_type`）
    pub member_type: String,
    /// 权限角色类型（可选，request body 参数 `perm_type`）
    pub perm_type: Option<String>,
    /// 协作者类型（可选，request body 参数 `type`）
    pub member_kind: Option<String>,
}

impl DeletePermissionMemberRequest {
    /// 创建新的移除协作者请求。
    /// 创建移除协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `member_type` - 协作者 ID 类型（query 参数 `member_type`）
    pub fn new(
        config: Config,
        token: impl Into<String>,
        member_id: impl Into<String>,
        file_type: impl Into<String>,
        member_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            member_id: member_id.into(),
            file_type: file_type.into(),
            member_type: member_type.into(),
            perm_type: None,
            member_kind: None,
        }
    }

    /// 设置权限角色类型（知识库文档有效）
    pub fn perm_type(mut self, perm_type: impl Into<String>) -> Self {
        self.perm_type = Some(perm_type.into());
        self
    }

    /// 设置协作者类型
    ///
    /// **注意**：当 `member_type` 为 `wikispaceid` 时，通常需要同时传入
    /// `wiki_space_member`、`wiki_space_viewer`、`wiki_space_editor` 之一。
    pub fn member_kind(mut self, member_kind: impl Into<String>) -> Self {
        self.member_kind = Some(member_kind.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<DeletePermissionMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeletePermissionMemberResponse> {
        // === 必填字段验证 ===
        validate_required!(self.token, "token 不能为空");
        validate_required!(self.member_id, "member_id 不能为空");
        validate_required!(self.file_type, "file_type 不能为空");
        validate_required!(self.member_type, "member_type 不能为空");

        // === 枚举值验证 ===
        match self.file_type.as_str() {
            "doc" | "sheet" | "file" | "wiki" | "bitable" | "docx" | "folder" | "mindnote"
            | "minutes" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_type",
                    "file_type 必须为 doc/sheet/file/wiki/bitable/docx/folder/mindnote/minutes/slides",
                ));
            }
        }
        match self.member_type.as_str() {
            "email" | "openid" | "unionid" | "openchat" | "opendepartmentid" | "userid"
            | "groupid" | "wikispaceid" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "member_type",
                    "member_type 必须为 email/openid/unionid/openchat/opendepartmentid/userid/groupid/wikispaceid",
                ));
            }
        }
        if let Some(perm_type) = &self.perm_type {
            match perm_type.as_str() {
                "container" | "single_page" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "perm_type",
                        "perm_type 必须为 container/single_page",
                    ));
                }
            }
        }
        if let Some(member_kind) = &self.member_kind {
            match member_kind.as_str() {
                "user" | "chat" | "department" | "group" | "wiki_space_member"
                | "wiki_space_viewer" | "wiki_space_editor" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "type",
                        "type 必须为 user/chat/department/group/wiki_space_member/wiki_space_viewer/wiki_space_editor",
                    ));
                }
            }
        }

        // === 业务规则验证 ===
        if self.member_type == "wikispaceid" {
            match self.member_kind.as_deref() {
                Some("wiki_space_member" | "wiki_space_viewer" | "wiki_space_editor") => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "type",
                        "当 member_type=wikispaceid 时，type 必须为 wiki_space_member/wiki_space_viewer/wiki_space_editor",
                    ));
                }
            }
        }

        let api_endpoint =
            DriveApi::DeletePermissionMember(self.token.clone(), self.member_id.clone());

        #[derive(Serialize)]
        struct DeletePermissionMemberRequestBody {
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            r#type: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            perm_type: Option<String>,
        }

        let body = DeletePermissionMemberRequestBody {
            r#type: self.member_kind,
            perm_type: self.perm_type,
        };

        let api_request =
            ApiRequest::<DeletePermissionMemberResponse>::delete(&api_endpoint.to_url())
                .query("type", &self.file_type)
                .query("member_type", &self.member_type)
                .body(serialize_params(&body, "移除云文档协作者权限")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除")
    }
}

/// 移除协作者权限响应 data。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeletePermissionMemberResponse {}

impl ApiResponseTrait for DeletePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_permission_member_request_builder() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "member_id", "docx", "openid")
                .perm_type("container");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.member_id, "member_id");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.member_type, "openid");
        assert_eq!(request.perm_type, Some("container".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DeletePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request = DeletePermissionMemberRequest::new(config, "", "member_id", "docx", "openid");
        assert_eq!(request.token, "");
    }

    #[test]
    fn test_empty_member_id_validation() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "", "docx", "openid");
        assert_eq!(request.member_id, "");
    }

    #[test]
    fn test_empty_file_type_validation() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "member_id", "", "openid");
        assert_eq!(request.file_type, "");
    }

    #[test]
    fn test_invalid_file_type_validation() {
        let config = Config::default();
        let request = DeletePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "invalid_type",
            "openid",
        );
        assert_eq!(request.file_type, "invalid_type");
    }

    #[test]
    fn test_empty_member_type_validation() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "member_id", "docx", "");
        assert_eq!(request.member_type, "");
    }

    #[test]
    fn test_invalid_member_type_validation() {
        let config = Config::default();
        let request = DeletePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "invalid_type",
        );
        assert_eq!(request.member_type, "invalid_type");
    }

    #[test]
    fn test_invalid_perm_type_validation() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "member_id", "docx", "openid")
                .perm_type("invalid_perm_type");
        assert_eq!(request.perm_type, Some("invalid_perm_type".to_string()));
    }

    #[test]
    fn test_invalid_member_kind_validation() {
        let config = Config::default();
        let request =
            DeletePermissionMemberRequest::new(config, "file_token", "member_id", "docx", "openid")
                .member_kind("invalid_kind");
        assert_eq!(request.member_kind, Some("invalid_kind".to_string()));
    }

    #[test]
    fn test_wikispaceid_without_member_kind_validation() {
        let config = Config::default();
        let request = DeletePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "wikispaceid",
        );
        assert_eq!(request.member_type, "wikispaceid");
        assert_eq!(request.member_kind, None);
    }
}
