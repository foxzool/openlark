//! CoreHR 地点相关模型
//!
//! 包含创建、删除、查询、更新地点等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 地点基础数据结构

/// 地点信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// 地点 ID
    pub location_id: String,
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
}

// 创建地点相关模型

/// 官方 i18n 多语结构：`{lang, value}`
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18n {
    /// 语言编码，如 `zh-CN`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 `hiberarchy_common` 结构（地点基本信息：名称、编码、启用状态、上级地点等）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct HiberarchyCommon {
    /// 上级地点 ID（地点用途为“办公地点”时必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 地点名称（i18n 多语，必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18n>>,
    /// 启用状态（必填，true 为启用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 地点编码（不能与其他记录的编码重复）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18n>>,
}

/// 创建地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 地点名称（必填）
    pub name: String,
    /// 地点基本信息（必填：名称、启停用状态等）
    pub hiberarchy_common: HiberarchyCommon,
    /// 版本生效日期（格式：YYYY-MM-DD 00:00:00，必填）
    pub effective_time: String,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// 创建地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 地点 ID
    pub location_id: String,
}

// 删除地点相关模型

/// 删除地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 删除结果
    pub result: bool,
}

// 查询单个地点相关模型

/// 查询单个地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 地点信息
    pub location: Location,
}

// 批量查询地点相关模型

/// 批量查询地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRequestBody {
    /// 分页大小（1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 批量查询地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 地点列表
    pub location_list: Vec<Location>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// 更新地点相关模型

/// 更新地点请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 地点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 地点类型
    /// - 1: 总部
    /// - 2: 分公司
    /// - 3: 办事处
    /// - 4: 其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_type: Option<i32>,
    /// 详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// 城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 状态
    /// - 1: 启用
    /// - 2: 停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 更新地点响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 更新结果
    pub result: bool,
}
