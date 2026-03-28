//! 创建部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/create>

use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::{
        department::models::DepartmentResponse,
        user::models::{DepartmentIdType, UserIdType},
    },
    endpoints::CONTACT_V3_DEPARTMENTS,
};

/// 创建部门请求体
///
/// 说明：字段较多，这里仅显式建模必填字段，其余字段使用 `extra` 透传。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentBody {
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_department_id: String,
    /// 自定义部门 ID（department_id，可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 额外字段透传
    #[serde(default, flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl CreateDepartmentBody {
    pub fn new(name: impl Into<String>, parent_department_id: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parent_department_id: parent_department_id.into(),
            department_id: None,
            extra: HashMap::new(),
        }
    }
}

/// 创建部门请求
///
/// 用于在通讯录中创建新部门。
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
/// - `name`: 部门名称，必填
/// - `parent_department_id`: 父部门 ID，必填
/// - `department_id`: 自定义部门 ID（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateDepartmentBody::new("研发部", "parent_dept_1");
/// let request = CreateDepartmentRequest::new(config)
///     .user_id_type(UserIdType::OpenId)
///     .client_token("uuid");
/// ```
pub struct CreateDepartmentRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    client_token: Option<String>,
}

impl CreateDepartmentRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/department/create>
    pub async fn execute(self, body: CreateDepartmentBody) -> SDKResult<DepartmentResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateDepartmentBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DepartmentResponse> {
        // === 必填字段验证 ===
        validate_required!(body.name, "name 不能为空");
        validate_required!(body.parent_department_id, "parent_department_id 不能为空");
        if body.name.contains('/') {
            return Err(error::validation_error(
                "name 不可包含 /".to_string(),
                "部门名称不可包含斜杠（/）".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/departments
        let mut req: ApiRequest<DepartmentResponse> =
            ApiRequest::post(CONTACT_V3_DEPARTMENTS).body(serialize_params(&body, "创建部门")?);

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
        extract_response_data(resp, "创建部门")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_department_request_builder() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_create_department_request_with_user_id_type() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config).user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_create_department_body_builder() {
        let body = CreateDepartmentBody::new("研发部", "parent_dept_1");
        assert_eq!(body.name, "研发部");
        assert_eq!(body.parent_department_id, "parent_dept_1");
    }

    #[test]
    fn test_create_department_body_with_custom_department_id() {
        let body = CreateDepartmentBody::new("产品部", "parent_dept_2");
        let body_with_id = CreateDepartmentBody {
            department_id: Some("custom_dept_id".to_string()),
            ..body
        };
        assert_eq!(
            body_with_id.department_id,
            Some("custom_dept_id".to_string())
        );
    }

    #[test]
    fn test_create_department_request_with_all_options() {
        let config = Config::default();
        let request = CreateDepartmentRequest::new(config)
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
