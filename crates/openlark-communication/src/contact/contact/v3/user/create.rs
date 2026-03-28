//! 创建用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/create>

use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 创建用户请求体
///
/// 说明：字段较多，这里仅显式建模必填字段，其余字段使用 `extra` 透传。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserBody {
    /// 用户名
    pub name: String,
    /// 手机号
    pub mobile: String,
    /// 用户所属部门 ID 列表
    pub department_ids: Vec<String>,
    /// 员工类型
    pub employee_type: i32,
    /// 自定义 user_id（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 额外字段透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl CreateUserBody {
    pub fn new(
        name: impl Into<String>,
        mobile: impl Into<String>,
        department_ids: Vec<String>,
        employee_type: i32,
    ) -> Self {
        Self {
            name: name.into(),
            mobile: mobile.into(),
            department_ids,
            employee_type,
            user_id: None,
            extra: HashMap::new(),
        }
    }
}

/// 创建/更新/查询用户类接口通用响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: User,
}

impl ApiResponseTrait for UserResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建用户请求
///
/// 用于在通讯录中创建新用户。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
/// - `client_token`: 幂等 token（可选）
///
/// # 请求体字段
///
/// - `name`: 用户名，必填
/// - `mobile`: 手机号，必填
/// - `department_ids`: 用户所属部门 ID 列表，必填
/// - `employee_type`: 员工类型，必填
/// - `user_id`: 自定义 user_id（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateUserBody::new(
///     "张三",
///     "13800138000",
///     vec!["dept_1".to_string()],
///     1,
/// );
/// let request = CreateUserRequest::new(config)
///     .user_id_type(UserIdType::OpenId)
///     .client_token("uuid");
/// ```
pub struct CreateUserRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    client_token: Option<String>,
}

impl CreateUserRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            client_token: None,
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

    /// 幂等 token（查询参数，可选）
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/create>
    pub async fn execute(self, body: CreateUserBody) -> SDKResult<UserResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateUserBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");
        validate_required!(body.mobile, "mobile 不能为空");
        if body.department_ids.is_empty() {
            return Err(error::validation_error(
                "department_ids 不能为空".to_string(),
                "department_ids 至少需要包含 1 个部门 ID".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/users
        let mut req: ApiRequest<UserResponse> =
            ApiRequest::post(CONTACT_V3_USERS).body(serialize_params(&body, "创建用户")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(client_token) = self.client_token {
            req = req.query("client_token", client_token);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建用户")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_request_builder() {
        let config = Config::default();
        let request = CreateUserRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_user_request_with_user_id_type() {
        let config = Config::default();
        let request = CreateUserRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_user_body_builder() {
        let body = CreateUserBody::new("张三", "13800138000", vec!["dept_1".to_string()], 1);
        assert_eq!(body.name, "张三");
        assert_eq!(body.mobile, "13800138000");
        assert_eq!(body.department_ids.len(), 1);
        assert_eq!(body.employee_type, 1);
    }

    #[test]
    fn test_create_user_body_with_custom_user_id() {
        let body = CreateUserBody::new("李四", "13900139000", vec!["dept_2".to_string()], 2);
        let body_with_id = CreateUserBody {
            user_id: Some("custom_user_id".to_string()),
            ..body
        };
        assert_eq!(body_with_id.user_id, Some("custom_user_id".to_string()));
    }

    #[test]
    fn test_create_user_request_with_all_options() {
        let config = Config::default();
        let request = CreateUserRequest::new(config)
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::DepartmentId)
            .client_token("uuid123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
        assert_eq!(request.client_token, Some("uuid123".to_string()));
    }
}
