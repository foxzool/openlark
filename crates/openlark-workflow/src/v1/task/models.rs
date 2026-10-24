//! 任务 API v1 共享数据模型
//!
//! 对齐 task-v1 官方字段结构（due / origin / follower / collaborator 对象形态），
//! 供创建任务、更新任务、新增关注者/执行者等接口复用。

use serde::{Deserialize, Serialize};

/// 任务时间对象（due，v1）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskDueV1 {
    /// 表示时间的 Unix 时间戳（单位为秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    /// 时区，需符合 IANA Time Zone Database 标准，默认 Asia/Shanghai
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    /// 是否全天任务（全天任务的时间为当天 UTC 时间的 0 点）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

/// 任务来源链接（origin.href，v1）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskHrefV1 {
    /// 链接地址（支持 http/https/applink 协议）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// 链接标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 任务第三方来源信息（origin，v1）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskOriginV1 {
    /// 任务来源名称（多语言字典 JSON 字符串；origin 存在时必填）
    pub platform_i18n_name: String,

    /// 来源平台详情页链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<TaskHrefV1>,
}

/// 判断 origin 是否为未设置的默认空值（用于序列化时跳过未设置的 origin）
pub fn is_default_origin(origin: &TaskOriginV1) -> bool {
    origin.platform_i18n_name.trim().is_empty() && origin.href.is_none()
}

/// 任务关注者对象（follower，v1）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskFollowerV1 {
    /// 任务关注人 ID（已废弃，为向前兼容保留，推荐使用 id_list）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 关注人 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}

/// 任务执行者对象（collaborator，v1）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskCollaboratorV1 {
    /// 任务执行者 ID（已废弃，为向前兼容保留，推荐使用 id_list）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// 执行者用户 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
}
