//! 删除用户
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS,
};

/// 删除用户请求
///
/// 用于删除通讯录中的用户。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id`: 用户 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeleteUserRequest::new(config)
///     .user_id("user_xxx")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct DeleteUserRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
}

impl DeleteUserRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
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

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/user/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.user_id, "user_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/users/:user_id
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_USERS, self.user_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除用户")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_user_request_builder() {
        let config = Config::default();
        let request = DeleteUserRequest::new(config).user_id("user_xxx");
        assert_eq!(request.user_id, "user_xxx");
    }

    #[test]
    fn test_delete_user_request_with_user_id_type() {
        let config = Config::default();
        let request = DeleteUserRequest::new(config)
            .user_id("user_xxx")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_delete_user_request_default_values() {
        let config = Config::default();
        let request = DeleteUserRequest::new(config);
        assert_eq!(request.user_id, "");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_delete_user_request_with_all_options() {
        let config = Config::default();
        let request = DeleteUserRequest::new(config)
            .user_id("user_123")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}
