//! 批量获取用户信息
//!
//! docPath: https://open.feishu.cn/document/contact-v3/user/batch

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::models::{DepartmentIdType, User, UserIdType},
    endpoints::CONTACT_V3_USERS_BATCH,
};

/// 批量获取用户信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetUsersResponse {
    /// 用户列表。
    #[serde(default)]
    pub items: Vec<User>,
}

impl ApiResponseTrait for BatchGetUsersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取用户信息请求
///
/// 用于通过用户 ID 批量获取用户详细信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `user_ids`: 用户 ID 列表，必填（1-50 个）
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `department_id_type`: 部门 ID 类型（可选）
///
/// # 示例
///
/// ```rust,ignore
/// let request = BatchGetUsersRequest::new(config)
///     .push_user_id("user_1")
///     .push_user_id("user_2")
///     .user_id_type(UserIdType::OpenId);
/// ```
pub struct BatchGetUsersRequest {
    /// 配置信息。
    config: Config,
    /// 用户 ID 列表。
    user_ids: Vec<String>,
    /// 用户 ID 类型。
    user_id_type: Option<UserIdType>,
    /// 部门 ID 类型。
    department_id_type: Option<DepartmentIdType>,
}

impl BatchGetUsersRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_ids: Vec::new(),
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 追加一个用户 ID（查询参数 user_ids，可多次传递，1-50 个）
    pub fn push_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
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
    /// docPath: https://open.feishu.cn/document/contact-v3/user/batch
    pub async fn execute(self) -> SDKResult<BatchGetUsersResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetUsersResponse> {
        // === 必填字段验证 ===
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空");

        // url: GET:/open-apis/contact/v3/users/batch
        let mut req: ApiRequest<BatchGetUsersResponse> = ApiRequest::get(CONTACT_V3_USERS_BATCH);

        for user_id in self.user_ids {
            req = req.query("user_ids", user_id);
        }
        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "批量获取用户信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_users_request_builder() {
        let config = Config::default();
        let request = BatchGetUsersRequest::new(config)
            .push_user_id("user_1")
            .push_user_id("user_2");
        assert_eq!(request.user_ids.len(), 2);
    }

    #[test]
    fn test_batch_get_users_request_with_user_id_type() {
        let config = Config::default();
        let request = BatchGetUsersRequest::new(config)
            .push_user_id("user_1")
            .user_id_type(UserIdType::OpenId);
        assert_eq!(request.user_id_type, Some(UserIdType::OpenId));
    }

    #[test]
    fn test_batch_get_users_request_with_department_id_type() {
        let config = Config::default();
        let request = BatchGetUsersRequest::new(config)
            .push_user_id("user_1")
            .department_id_type(DepartmentIdType::DepartmentId);
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::DepartmentId)
        );
    }

    #[test]
    fn test_batch_get_users_request_default_values() {
        let config = Config::default();
        let request = BatchGetUsersRequest::new(config);
        assert_eq!(request.user_ids.len(), 0);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_batch_get_users_request_with_all_options() {
        let config = Config::default();
        let request = BatchGetUsersRequest::new(config)
            .push_user_id("user_1")
            .push_user_id("user_2")
            .user_id_type(UserIdType::UnionId)
            .department_id_type(DepartmentIdType::OpenDepartmentId);
        assert_eq!(request.user_ids.len(), 2);
        assert_eq!(request.user_id_type, Some(UserIdType::UnionId));
        assert_eq!(
            request.department_id_type,
            Some(DepartmentIdType::OpenDepartmentId)
        );
    }
}
