//! 恢复已删除用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/resurrect>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    contact::contact::v3::user::models::{DepartmentIdType, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 恢复用户部门信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDepartmentInfo {
    /// 部门 ID。
    pub department_id: String,
    /// 部门内排序。
    pub department_order: i32,
    /// 用户在部门中的排序。
    pub user_order: i32,
}

/// 恢复已删除用户请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResurrectUserBody {
    /// 部门信息列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<UserDepartmentInfo>>,
    /// 订阅 ID 列表。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<String>>,
}

/// 恢复已删除用户请求
///
/// 用于恢复已删除的用户账号。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id`: 用户 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 请求体字段
///
/// - `departments`: 部门信息列表（可选）
/// - `subscription_ids`: 订阅 ID 列表（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let dept_info = UserDepartmentInfo {
///     department_id: "dept_1".to_string(),
///     department_order: 1,
///     user_order: 1,
/// };
/// let body = ResurrectUserBody {
///     departments: Some(vec![dept_info]),
///     subscription_ids: None,
/// };
/// let request = ResurrectUserRequest::new(config)
///     .user_id("user_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct ResurrectUserRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID。
    user_id: String,
    /// 用户 ID 类型。
    user_id_type: Option<UserIdType>,
    /// 部门 ID 类型。
    department_id_type: Option<DepartmentIdType>,
}

impl ResurrectUserRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 用户 ID（路径参数）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
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
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/resurrect>
    pub async fn execute(self, body: ResurrectUserBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: ResurrectUserBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.user_id, "user_id 不能为空");

        // url: POST:/open-apis/contact/v3/users/:user_id/resurrect
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::post(format!("{}/{}/resurrect", CONTACT_V3_USERS, self.user_id))
                .body(serialize_params(&body, "恢复已删除用户")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "恢复已删除用户")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_resurrect_user_request_builder() {
        let config = Config::default();
        let request = ResurrectUserRequest::new(config).user_id("user_xxx");
        assert_eq!(request.user_id, "user_xxx");
    }

    #[test]
    fn test_resurrect_user_request_with_user_id_type() {
        let config = Config::default();
        let request = ResurrectUserRequest::new(config)
            .user_id("user_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_resurrect_user_body_builder() {
        let dept_info = UserDepartmentInfo {
            department_id: "dept_1".to_string(),
            department_order: 1,
            user_order: 1,
        };
        let body = ResurrectUserBody {
            departments: Some(vec![dept_info]),
            subscription_ids: None,
        };
        assert_eq!(body.departments.as_ref().map(Vec::len), Some(1));
        assert_eq!(
            body.departments
                .as_ref()
                .and_then(|departments| departments.first())
                .map(|department| department.department_id.as_str()),
            Some("dept_1")
        );
    }

    #[test]
    fn test_resurrect_user_request_default_values() {
        let config = Config::default();
        let request = ResurrectUserRequest::new(config);
        assert_eq!(request.user_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_resurrect_user_request_with_all_options() {
        let config = Config::default();
        let request = ResurrectUserRequest::new(config)
            .user_id("user_123")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }
}
