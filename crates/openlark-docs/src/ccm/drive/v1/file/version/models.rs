use openlark_core::api::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 版本文档信息（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileVersionInfo {
    /// 版本文档的标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 版本文档的版本标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// 当前版本对应的源文档的 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 版本文档的所有者的 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// 版本文档的创建者的 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// 版本文档的创建时间（Unix 时间戳，单位秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 版本文档的更新时间（创建版本时可能不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 版本文档的状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 版本文档的类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    /// 源文档的类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
}

/// 获取文件版本列表响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFileVersionsData {
    /// 版本文档列表
    #[serde(default)]
    pub items: Vec<FileVersionInfo>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for FileVersionInfo {}
impl ApiResponseTrait for ListFileVersionsData {}
