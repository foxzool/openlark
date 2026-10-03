use serde::{Deserialize, Serialize};

/// 用户信息（用于记录的 created_by/last_modified_by 等字段）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Person {
    /// 用户 id，id 类型等于 user_id_type 所指定的类型
    pub id: String,
    /// 用户的中文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户的英文名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 用户的邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 头像链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// 记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    /// 记录 ID
    pub record_id: String,
    /// 数据表字段（字段名到字段值的映射）
    pub fields: serde_json::Value,
    /// 该记录的创建人（需开启 automatic_fields 才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Person>,
    /// 该记录的创建时间（毫秒时间戳，需开启 automatic_fields 才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    /// 该记录最新一次更新的修改人（需开启 automatic_fields 才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<Person>,
    /// 该记录最近一次的更新时间（毫秒时间戳，需开启 automatic_fields 才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<i64>,
    /// 记录分享链接（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_url: Option<String>,
    /// 记录链接（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_url: Option<String>,
}
