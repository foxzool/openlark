//! Minutes API 数据模型
//!
//! 注意：该模块仅存放模型定义，不视为 API 文件。

use serde::{Deserialize, Serialize};

/// 用户 ID 类型（用于 query 参数 `user_id_type`）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// open_id。
    OpenId,
    /// union_id。
    UnionId,
    /// user_id。
    UserId,
}

impl UserIdType {
    /// 返回请求参数使用的字符串值。
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UnionId => "union_id",
            UserIdType::UserId => "user_id",
        }
    }
}

/// 妙记基本信息（对应响应体 `data.minute`）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteInfo {
    /// 妙记 token
    pub token: String,
    /// 所有者 ID
    pub owner_id: String,
    /// 妙记创建时间 timestamp（ms 级别，字符串）
    pub create_time: String,
    /// 妙记标题
    pub title: String,
    /// 妙记封面链接
    pub cover: String,
    /// 妙记时长（ms 级别，字符串）
    pub duration: String,
    /// 妙记链接
    pub url: String,
}

/// 妙记音视频文件下载信息（对应响应体 `data.download_url`）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteMediaInfo {
    /// 音视频文件下载链接（有效期 1 天）
    pub download_url: String,
}

/// 妙记统计数据（对应响应体 `data.statistics`）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStatistics {
    /// 用户浏览数
    pub user_view_count: String,
    /// 页面浏览数量
    pub page_view_count: String,
    /// 用户浏览列表
    pub user_view_list: Vec<UserViewDetail>,
}

/// 用户浏览明细
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserViewDetail {
    /// 用户 ID
    pub user_id: String,
    /// 用户的最近查看时间 timestamp（ms 级别，字符串）
    pub view_time: String,
}
