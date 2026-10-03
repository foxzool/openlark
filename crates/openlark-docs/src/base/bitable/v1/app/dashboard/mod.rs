/// App Dashboard API v1
///
/// 提供飞书多维表格仪表盘的完整管理功能，包括：
/// - 列出仪表盘列表
/// - 复制仪表盘
/// - 仪表盘权限管理
use serde::{Deserialize, Serialize};

pub mod copy;
pub mod list;

// 重新导出所有服务类型
// copy 模块显式导出
pub use copy::{CopyDashboardRequest, CopyDashboardResponse};
// list 模块显式导出
pub use list::{ListDashboardsRequest, ListDashboardsResponse};

/// 仪表盘信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Dashboard {
    /// 仪表盘ID
    pub block_id: String,
    /// 仪表盘名称
    pub name: String,
    /// 仪表盘描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 仪表盘URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 创建者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    /// 仪表盘权限信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Creator {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// 权限信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Permission {
    /// 权限类型
    pub permission_type: String,
    /// 权限实体
    pub entity: String,
    /// 权限值
    pub value: String,
}
