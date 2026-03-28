//! 修改用户部分信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/patch>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    contact::contact::v3::user::create::UserResponse,
    contact::contact::v3::user::models::{DepartmentIdType, UserIdType},
    endpoints::CONTACT_V3_USERS,
};

/// 修改用户部分信息请求
///
/// 用于更新用户的指定字段，不影响其他字段。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id`: 用户 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let body = serde_json::json!({
///     "name": "新用户名",
///     "mobile": "13900139000",
/// });
/// let request = PatchUserRequest::new(config)
///     .user_id("user_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct PatchUserRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
}

impl PatchUserRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/patch>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<UserResponse> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserResponse> {
        // === 必填字段验证 ===
        validate_required!(self.user_id, "user_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/users/:user_id
        let mut req: ApiRequest<UserResponse> =
            ApiRequest::patch(format!("{}/{}", CONTACT_V3_USERS, self.user_id))
                .body(serialize_params(&body, "修改用户部分信息")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "修改用户部分信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_user_request_builder() {
        let config = Config::default();
        let request = PatchUserRequest::new(config).user_id("user_xxx");
        assert_eq!(request.user_id, "user_xxx");
    }

    #[test]
    fn test_patch_user_request_with_user_id_type() {
        let config = Config::default();
        let request = PatchUserRequest::new(config)
            .user_id("user_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_patch_user_request_default_values() {
        let config = Config::default();
        let request = PatchUserRequest::new(config);
        assert_eq!(request.user_id, "");
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_patch_user_request_with_all_options() {
        let config = Config::default();
        let request = PatchUserRequest::new(config)
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
