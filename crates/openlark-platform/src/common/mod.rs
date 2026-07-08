//! 通用数据模型

use serde::{Deserialize, Serialize};

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppInfo {
    /// 应用 ID
    pub app_id: String,
    /// 应用名称
    pub app_name: String,
    /// 应用描述
    pub description: Option<String>,
    /// 应用状态
    pub status: String,
}

/// 目录项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryItem {
    /// 项 ID
    pub id: String,
    /// 项名称
    pub name: String,
    /// 项类型
    pub item_type: String,
}

/// 系统设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    /// 设置键
    pub key: String,
    /// 设置值
    pub value: String,
    /// 设置描述
    pub description: Option<String>,
}

// API 端点定义
pub mod api_endpoints;
pub mod api_utils;
pub mod constants;
