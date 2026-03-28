//! 更新用户组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    contact::contact::v3::user::models::{DepartmentIdType, UserIdType},
    endpoints::CONTACT_V3_GROUP,
};

/// 更新用户组请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchGroupBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PatchGroupBody {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

/// 更新用户组请求
///
/// 用于更新用户组的部分信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `group_id`: 用户组 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `name`: 用户组名称（可选）
/// - `description`: 用户组描述（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = PatchGroupBody::new()
///     .name("新用户组名称");
/// let request = PatchGroupRequest::new(config)
///     .group_id("group_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct PatchGroupRequest {
    config: Config,
    group_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl PatchGroupRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/patch>
    pub async fn execute(self, body: PatchGroupBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: PatchGroupBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.group_id, "group_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/group/:group_id
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}", CONTACT_V3_GROUP, self.group_id))
                .body(serialize_params(&body, "更新用户组")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新用户组")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_group_request_builder() {
        let config = Config::default();
        let request = PatchGroupRequest::new(config).group_id("group_xxx");
        assert_eq!(request.group_id, "group_xxx");
    }

    #[test]
    fn test_patch_group_request_with_user_id_type() {
        let config = Config::default();
        let request = PatchGroupRequest::new(config)
            .group_id("group_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_patch_group_body_builder() {
        let body = PatchGroupBody::new().name("新名称");
        assert_eq!(body.name, Some("新名称".to_string()));
    }

    #[test]
    fn test_patch_group_request_default_values() {
        let config = Config::default();
        let request = PatchGroupRequest::new(config);
        assert_eq!(request.group_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_patch_group_request_with_all_options() {
        let config = Config::default();
        let request = PatchGroupRequest::new(config)
            .group_id("group_123")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(request.group_id, "group_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }
}
