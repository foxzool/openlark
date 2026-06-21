//! 门禁用户管理 API（门面）
//!
//! [`UsersService`] 是轻量门面，返回各端点的 Request 构建器（真实请求逻辑在
//! `super::user::*` 下，走 `openlark_core::Transport`）。

use openlark_core::config::Config;

/// 用户管理服务
///
/// 不直接发请求，仅返回端点构建器。
#[derive(Debug, Clone)]
pub struct UsersService {
    config: Config,
}

impl UsersService {
    /// 创建新的用户管理服务实例。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取单个用户信息。
    pub fn get(&self, user_id: impl Into<String>) -> super::user::get::GetUserRequest {
        super::user::get::GetUserRequest::new(self.config.clone(), user_id)
    }

    /// 获取用户列表。
    pub fn list(&self) -> super::user::list::ListUsersRequest {
        super::user::list::ListUsersRequest::new(self.config.clone())
    }

    /// 修改用户部分信息。
    pub fn patch(&self, user_id: impl Into<String>) -> super::user::patch::PatchUserRequest {
        super::user::patch::PatchUserRequest::new(self.config.clone(), user_id)
    }

    /// 创建用户。
    pub fn create(&self) -> super::user::create::CreateUserRequest {
        super::user::create::CreateUserRequest::new(self.config.clone())
    }

    /// 删除用户。
    pub fn delete(&self, user_id: impl Into<String>) -> super::user::delete::DeleteUserRequest {
        super::user::delete::DeleteUserRequest::new(self.config.clone(), user_id)
    }
}
