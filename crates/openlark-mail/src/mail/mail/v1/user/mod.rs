//! 用户邮箱模块

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱服务
#[derive(Clone)]
pub struct User {
    /// 导航 struct，accessor 待补（见 #274/#275 范式），本 change 不接线。
    #[expect(dead_code)]
    config: Arc<Config>,
}

impl User {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}
pub mod query;
