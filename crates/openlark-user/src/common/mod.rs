//! 通用数据模型

use serde::{Deserialize, Serialize};

/// 用户设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSetting {
    /// 设置键
    pub key: String,
    /// 设置值
    pub value: String,
    /// 设置类型
    pub setting_type: String,
}

/// 用户偏好
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    /// 偏好键
    pub key: String,
    /// 偏好值
    pub value: String,
    /// 偏好类别
    pub category: Option<String>,
}
