//! 创建用户组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/create>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        group::models::CreateGroupResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_GROUP,
};

/// 创建用户组请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupBody {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl CreateGroupBody {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            r#type: None,
            group_id: None,
        }
    }
}

/// 创建用户组请求
///
/// 用于在通讯录中创建新的用户组。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `name`: 用户组名称，必填
/// - `description`: 用户组描述（可选）
/// - `type`: 用户组类型（可选）
/// - `group_id`: 自定义用户组 ID（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateGroupBody::new("研发组");
/// let request = CreateGroupRequest::new(config)
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct CreateGroupRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl CreateGroupRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
        }
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/create>
    pub async fn execute(self, body: CreateGroupBody) -> SDKResult<CreateGroupResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateGroupBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateGroupResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");

        // url: POST:/open-apis/contact/v3/group
        let mut req: ApiRequest<CreateGroupResponse> =
            ApiRequest::post(CONTACT_V3_GROUP).body(serialize_params(&body, "创建用户组")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "创建用户组")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_group_request_builder() {
        let config = Config::default();
        let request = CreateGroupRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_group_request_with_user_id_type() {
        let config = Config::default();
        let request = CreateGroupRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_group_body_builder() {
        let body = CreateGroupBody::new("研发组");
        assert_eq!(body.name, "研发组");
    }

    #[test]
    fn test_create_group_body_with_all_fields() {
        let body = CreateGroupBody {
            name: "产品组".to_string(),
            description: Some("产品研发组".to_string()),
            r#type: Some(1),
            group_id: Some("custom_group_id".to_string()),
        };
        assert_eq!(body.name, "产品组");
        assert_eq!(body.description, Some("产品研发组".to_string()));
        assert_eq!(body.r#type, Some(1));
        assert_eq!(body.group_id, Some("custom_group_id".to_string()));
    }

    #[test]
    fn test_create_group_request_with_all_options() {
        let config = Config::default();
        let request = CreateGroupRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }
}
