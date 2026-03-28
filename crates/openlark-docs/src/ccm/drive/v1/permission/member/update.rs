//! 更新协作者权限
//!
//! 更新文件或文件夹中指定协作者的权限。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/permission/permission-member/update>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionMember;

/// 更新协作者权限请求
///
/// 用于更新文件或文件夹中指定协作者的权限。
///
/// # 字段说明
///
/// - `token`: 文件 token，不能为空
/// - `member_id`: 成员 ID，不能为空
/// - `file_type`: 云文档类型，必须为 doc/sheet/file/wiki/bitable/docx/folder/mindnote/minutes/slides 之一
/// - `need_notification`: 是否发送通知，默认 false
/// - `member_type`: 协作者 ID 类型，必须为 email/openid/unionid/openchat/opendepartmentid/userid/groupid/wikispaceid 之一
/// - `perm`: 权限角色，必须为 view/edit/full_access 之一
/// - `perm_type`: 权限角色类型，可选值为 container/single_page
/// - `member_kind`: 协作者类型，当 member_type 为 wikispaceid 时必填
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::permission::member::UpdatePermissionMemberRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = UpdatePermissionMemberRequest::new(
///     config,
///     "file_token",
///     "member_id",
///     "docx",
///     "openid",
///     "edit",
/// )
/// .need_notification(false)
/// .perm_type("container");
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct UpdatePermissionMemberRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员ID
    pub member_id: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 是否发送通知（query 参数 `need_notification`，默认 false）
    pub need_notification: Option<bool>,

    /// 协作者 ID 类型，与 member_id 对应
    pub member_type: String,
    /// 权限角色
    pub perm: String,
    /// 权限角色类型（默认 container）
    pub perm_type: Option<String>,
    /// 协作者类型（仅当 member_type 为 wikispaceid 时必填）
    pub member_kind: Option<String>,
}

impl UpdatePermissionMemberRequest {
    /// 创建更新协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `member_type` - 协作者 ID 类型
    /// * `perm` - 权限角色
    pub fn new(
        config: Config,
        token: impl Into<String>,
        member_id: impl Into<String>,
        file_type: impl Into<String>,
        member_type: impl Into<String>,
        perm: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            member_id: member_id.into(),
            file_type: file_type.into(),
            need_notification: None,
            member_type: member_type.into(),
            perm: perm.into(),
            perm_type: None,
            member_kind: None,
        }
    }

    /// 设置是否发送通知（默认 false）
    pub fn need_notification(mut self, need_notification: bool) -> Self {
        self.need_notification = Some(need_notification);
        self
    }

    /// 设置权限角色类型（默认 container，知识库文档有效）
    pub fn perm_type(mut self, perm_type: impl Into<String>) -> Self {
        self.perm_type = Some(perm_type.into());
        self
    }

    /// 设置协作者类型
    ///
    /// **注意**：当 `member_type` 为 `wikispaceid` 时必填，且必须在
    /// `wiki_space_member`、`wiki_space_viewer`、`wiki_space_editor` 中选择。
    pub fn member_kind(mut self, member_kind: impl Into<String>) -> Self {
        self.member_kind = Some(member_kind.into());
        self
    }

    pub async fn execute(self) -> SDKResult<UpdatePermissionMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdatePermissionMemberResponse> {
        // === 必填字段验证 ===
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 不能为空",
            ));
        }
        if self.member_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "member_id",
                "member_id 不能为空",
            ));
        }
        if self.file_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 不能为空",
            ));
        }
        if self.member_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "member_type",
                "member_type 不能为空",
            ));
        }
        if self.perm.is_empty() {
            return Err(openlark_core::error::validation_error(
                "perm",
                "perm 不能为空",
            ));
        }

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
        match self.perm.as_str() {
            "view" | "edit" | "full_access" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "perm",
                    "perm 必须为 view/edit/full_access",
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
        if self.file_type == "minutes" && self.perm == "full_access" {
            return Err(openlark_core::error::validation_error(
                "perm",
                "当 file_type=minutes 时，不支持 full_access",
            ));
        }
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
            DriveApi::UpdatePermissionMember(self.token.clone(), self.member_id.clone());

        #[derive(Serialize)]
        struct UpdatePermissionMemberRequestBody {
            member_type: String,
            perm: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            perm_type: Option<String>,
            #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
            r#type: Option<String>,
        }

        let mut api_request: ApiRequest<UpdatePermissionMemberResponse> =
            ApiRequest::put(&api_endpoint.to_url()).query("type", &self.file_type);

        if let Some(need_notification) = self.need_notification {
            api_request = api_request.query("need_notification", need_notification.to_string());
        }

        let body = UpdatePermissionMemberRequestBody {
            member_type: self.member_type,
            perm: self.perm,
            perm_type: self.perm_type,
            r#type: self.member_kind,
        };

        api_request = api_request.body(serialize_params(&body, "更新协作者权限")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新")
    }
}

/// 更新协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionMemberResponse {
    /// 更新后的协作者信息
    pub member: PermissionMember,
}

impl ApiResponseTrait for UpdatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_permission_member_request_builder() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "openid",
            "edit",
        )
        .need_notification(false)
        .perm_type("container");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.member_id, "member_id");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.member_type, "openid");
        assert_eq!(request.perm, "edit");
        assert_eq!(request.need_notification, Some(false));
        assert_eq!(request.perm_type, Some("container".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UpdatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request =
            UpdatePermissionMemberRequest::new(config, "", "member_id", "docx", "openid", "edit");
        assert_eq!(request.token, "");
    }

    #[test]
    fn test_empty_member_id_validation() {
        let config = Config::default();
        let request =
            UpdatePermissionMemberRequest::new(config, "file_token", "", "docx", "openid", "edit");
        assert_eq!(request.member_id, "");
    }

    #[test]
    fn test_empty_file_type_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "",
            "openid",
            "edit",
        );
        assert_eq!(request.file_type, "");
    }

    #[test]
    fn test_invalid_file_type_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "invalid_type",
            "openid",
            "edit",
        );
        assert_eq!(request.file_type, "invalid_type");
    }

    #[test]
    fn test_empty_member_type_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "",
            "edit",
        );
        assert_eq!(request.member_type, "");
    }

    #[test]
    fn test_invalid_member_type_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "invalid_type",
            "edit",
        );
        assert_eq!(request.member_type, "invalid_type");
    }

    #[test]
    fn test_empty_perm_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "openid",
            "",
        );
        assert_eq!(request.perm, "");
    }

    #[test]
    fn test_invalid_perm_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "openid",
            "invalid_perm",
        );
        assert_eq!(request.perm, "invalid_perm");
    }

    #[test]
    fn test_minutes_with_full_access_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "minutes",
            "openid",
            "full_access",
        );
        assert_eq!(request.file_type, "minutes");
        assert_eq!(request.perm, "full_access");
    }

    #[test]
    fn test_wikispaceid_without_member_kind_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "wikispaceid",
            "edit",
        );
        assert_eq!(request.member_type, "wikispaceid");
        assert_eq!(request.member_kind, None);
    }

    #[test]
    fn test_invalid_perm_type_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "openid",
            "edit",
        )
        .perm_type("invalid_perm_type");
        assert_eq!(request.perm_type, Some("invalid_perm_type".to_string()));
    }

    #[test]
    fn test_invalid_member_kind_validation() {
        let config = Config::default();
        let request = UpdatePermissionMemberRequest::new(
            config,
            "file_token",
            "member_id",
            "docx",
            "openid",
            "edit",
        )
        .member_kind("invalid_kind");
        assert_eq!(request.member_kind, Some("invalid_kind".to_string()));
    }
}
