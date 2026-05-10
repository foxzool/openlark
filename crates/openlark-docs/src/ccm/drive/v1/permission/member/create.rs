//! 添加协作者
//!
//! 为文件或文件夹添加协作者权限。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/permission/permission-member/create

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::PermissionMember;

/// 添加协作者请求
#[derive(Debug, Clone)]
pub struct CreatePermissionMemberRequest {
    config: Config,
    /// 文件token
    pub token: String,
    /// 云文档类型（query 参数 `type`，需要与 token 匹配）
    pub file_type: String,
    /// 是否发送通知（query 参数 `need_notification`，默认 false）
    pub need_notification: Option<bool>,

    /// 协作者 ID 类型，与 member_id 对应
    pub member_type: String,
    /// 协作者 ID
    pub member_id: String,
    /// 权限角色
    pub perm: String,
    /// 权限角色类型（默认 container）
    pub perm_type: Option<String>,
    /// 协作者类型（仅当 member_type 为 wikispaceid 时必填）
    pub member_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CreatePermissionMemberRequestBody {
    member_type: String,
    member_id: String,
    perm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    perm_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
}

impl CreatePermissionMemberRequest {
    /// 创建新的添加协作者请求。
    /// 创建添加协作者请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `file_type` - 云文档类型（query 参数 `type`）
    /// * `member_type` - 协作者 ID 类型
    /// * `member_id` - 协作者 ID
    /// * `perm` - 权限角色
    pub fn new(
        config: Config,
        token: impl Into<String>,
        file_type: impl Into<String>,
        member_type: impl Into<String>,
        member_id: impl Into<String>,
        perm: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            file_type: file_type.into(),
            need_notification: None,
            member_type: member_type.into(),
            member_id: member_id.into(),
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

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreatePermissionMemberResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePermissionMemberResponse> {
        // === 必填字段验证 ===
        validate_required!(self.token, "token 不能为空");
        validate_required!(self.file_type, "file_type 不能为空");
        validate_required!(self.member_type, "member_type 不能为空");
        validate_required!(self.member_id, "member_id 不能为空");
        validate_required!(self.perm, "perm 不能为空");

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

        let api_endpoint = DriveApi::CreatePermissionMember(self.token.clone());

        let mut api_request =
            ApiRequest::<CreatePermissionMemberResponse>::post(&api_endpoint.to_url())
                .query("type", &self.file_type);

        if let Some(need_notification) = self.need_notification {
            api_request = api_request.query("need_notification", need_notification.to_string());
        }

        let body = CreatePermissionMemberRequestBody {
            member_type: self.member_type,
            member_id: self.member_id,
            perm: self.perm,
            perm_type: self.perm_type,
            r#type: self.member_kind,
        };

        api_request = api_request.body(serialize_params(&body, "增加协作者权限")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "增加协作者权限")
    }
}

/// 协作者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionMemberResponse {
    /// 本次新增的协作者信息
    pub member: PermissionMember,
}

impl ApiResponseTrait for CreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_permission_member_request_builder() {
        let config = Config::default();

        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "openid",
            "ou_xxx",
            "view",
        )
        .need_notification(false)
        .perm_type("container");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.member_type, "openid");
        assert_eq!(request.member_id, "ou_xxx");
        assert_eq!(request.perm, "view");
        assert_eq!(request.need_notification, Some(false));
        assert_eq!(request.perm_type, Some("container".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_empty_token_validation() {
        let config = Config::default();
        let request =
            CreatePermissionMemberRequest::new(config, "", "docx", "openid", "ou_xxx", "view");
        assert_eq!(request.token, "");
    }

    #[test]
    fn test_empty_file_type_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "",
            "openid",
            "ou_xxx",
            "view",
        );
        assert_eq!(request.file_type, "");
    }

    #[test]
    fn test_invalid_file_type_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "invalid_type",
            "openid",
            "ou_xxx",
            "view",
        );
        assert_eq!(request.file_type, "invalid_type");
    }

    #[test]
    fn test_empty_member_type_validation() {
        let config = Config::default();
        let request =
            CreatePermissionMemberRequest::new(config, "file_token", "docx", "", "ou_xxx", "view");
        assert_eq!(request.member_type, "");
    }

    #[test]
    fn test_invalid_member_type_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "invalid_type",
            "ou_xxx",
            "view",
        );
        assert_eq!(request.member_type, "invalid_type");
    }

    #[test]
    fn test_empty_member_id_validation() {
        let config = Config::default();
        let request =
            CreatePermissionMemberRequest::new(config, "file_token", "docx", "openid", "", "view");
        assert_eq!(request.member_id, "");
    }

    #[test]
    fn test_empty_perm_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "openid",
            "ou_xxx",
            "",
        );
        assert_eq!(request.perm, "");
    }

    #[test]
    fn test_invalid_perm_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "openid",
            "ou_xxx",
            "invalid_perm",
        );
        assert_eq!(request.perm, "invalid_perm");
    }

    #[test]
    fn test_invalid_perm_type_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "openid",
            "ou_xxx",
            "view",
        )
        .perm_type("invalid_perm_type");
        assert_eq!(request.perm_type, Some("invalid_perm_type".to_string()));
    }

    #[test]
    fn test_invalid_member_kind_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "openid",
            "ou_xxx",
            "view",
        )
        .member_kind("invalid_kind");
        assert_eq!(request.member_kind, Some("invalid_kind".to_string()));
    }

    #[test]
    fn test_minutes_with_full_access_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "minutes",
            "openid",
            "ou_xxx",
            "full_access",
        );
        assert_eq!(request.file_type, "minutes");
        assert_eq!(request.perm, "full_access");
    }

    #[test]
    fn test_wikispaceid_without_member_kind_validation() {
        let config = Config::default();
        let request = CreatePermissionMemberRequest::new(
            config,
            "file_token",
            "docx",
            "wikispaceid",
            "ou_xxx",
            "view",
        );
        assert_eq!(request.member_type, "wikispaceid");
        assert_eq!(request.member_kind, None);
    }
}

