//! 动态订阅数据模型

use serde::{Deserialize, Serialize};

/// 动态订阅类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ActivitySubscriptionType {
    /// 任务创建
    #[default]
    TaskCreated,
    /// 任务更新
    TaskUpdated,
    /// 任务完成
    TaskCompleted,
    /// 任务删除
    TaskDeleted,
    /// 评论创建
    CommentCreated,
    /// 成员添加
    MemberAdded,
    /// 成员移除
    MemberRemoved,
}

/// 动态订阅目标类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ActivitySubscriptionTargetType {
    /// Webhook
    #[default]
    Webhook,
    /// 飞书群
    Chat,
}

/// 创建动态订阅请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateActivitySubscriptionBody {
    /// 订阅类型
    pub subscription_type: ActivitySubscriptionType,

    /// 目标类型
    pub target_type: ActivitySubscriptionTargetType,

    /// 目标 URL（当 target_type 为 webhook 时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,

    /// 飞书群 ID（当 target_type 为 chat 时）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

/// 更新动态订阅请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateActivitySubscriptionBody {
    /// 订阅类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<ActivitySubscriptionType>,

    /// 目标类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<ActivitySubscriptionTargetType>,

    /// 目标 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,

    /// 飞书群 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,
}

/// 动态订阅信息
#[derive(Debug, Clone, Deserialize)]
pub struct ActivitySubscription {
    /// 订阅 GUID
    pub subscription_guid: String,

    /// 订阅类型
    pub subscription_type: ActivitySubscriptionType,

    /// 目标类型
    pub target_type: ActivitySubscriptionTargetType,

    /// 目标 URL
    #[serde(default)]
    pub target_url: Option<String>,

    /// 飞书群 ID
    #[serde(default)]
    pub chat_id: Option<String>,

    /// 创建时间
    pub created_at: String,

    /// 更新时间
    pub updated_at: String,
}

/// 创建动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateActivitySubscriptionResponse {
    /// 订阅信息
    pub subscription: ActivitySubscription,
}

/// 获取动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetActivitySubscriptionResponse {
    /// 订阅信息
    pub subscription: ActivitySubscription,
}

/// 更新动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateActivitySubscriptionResponse {
    /// 订阅信息
    pub subscription: ActivitySubscription,
}

/// 删除动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteActivitySubscriptionResponse {
    /// 是否删除成功
    pub success: bool,

    /// 订阅 GUID
    pub subscription_guid: String,
}

/// 列取动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct ListActivitySubscriptionsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 总数
    #[serde(default)]
    pub total: Option<i32>,

    /// 订阅列表
    #[serde(default)]
    pub items: Vec<ActivitySubscription>,
}
