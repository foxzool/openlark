//! 更新用户 ID
//!
//! docPath: <https://open.feishu.cn/document/contact-v3/user/update_user_id>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS,
};

/// 更新用户 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserIdBody {
    pub new_user_id: String,
}

impl UpdateUserIdBody {
    pub fn new(new_user_id: impl Into<String>) -> Self {
        Self {
            new_user_id: new_user_id.into(),
        }
    }
}

/// 更新用户 ID 请求
///
/// 用于更新用户的自定义 ID。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_id`: 用户 ID，必填
/// - `user_id_type`: 用户 ID 类型（可选）
///
/// # 请求体字段
///
/// - `new_user_id`: 新用户 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let body = UpdateUserIdBody::new("new_user_id_123");
/// let request = UpdateUserIdRequest::new(config)
///     .user_id("old_user_id")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct UpdateUserIdRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
}

impl UpdateUserIdRequest {
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
    /// docPath: <https://open.feishu.cn/document/contact-v3/user/update_user_id>
    pub async fn execute(self, body: UpdateUserIdBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: UpdateUserIdBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.user_id, "user_id 不能为空");
        validate_required!(body.new_user_id, "new_user_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/users/:user_id/update_user_id
        let mut req: ApiRequest<EmptyData> = ApiRequest::patch(format!(
            "{}/{}/update_user_id",
            CONTACT_V3_USERS, self.user_id
        ))
        .body(serialize_params(&body, "更新用户 ID")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "更新用户 ID")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_update_user_id_request_builder() {
        let config = Config::default();
        let request = UpdateUserIdRequest::new(config).user_id("old_user_id");
        assert_eq!(request.user_id, "old_user_id");
    }

    #[test]
    fn test_update_user_id_request_with_user_id_type() {
        let config = Config::default();
        let request = UpdateUserIdRequest::new(config)
            .user_id("old_user_id")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_update_user_id_body_builder() {
        let body = UpdateUserIdBody::new("new_user_id_123");
        assert_eq!(body.new_user_id, "new_user_id_123");
    }

    #[test]
    fn test_update_user_id_request_default_values() {
        let config = Config::default();
        let request = UpdateUserIdRequest::new(config);
        assert_eq!(request.user_id, "");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_update_user_id_request_with_all_options() {
        let config = Config::default();
        let request = UpdateUserIdRequest::new(config)
            .user_id("user_123")
            .user_id_type(UserIdType::UnionId);
        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
    }
}
