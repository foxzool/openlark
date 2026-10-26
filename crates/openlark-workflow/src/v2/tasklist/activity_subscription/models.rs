//! 动态订阅数据模型

use crate::v2::tasklist::models::TasklistMemberDetail;
use serde::{Deserialize, Serialize};

/// 创建动态订阅请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateActivitySubscriptionBody {
    /// 订阅名称（必填，不能为空，最大 50 个字符）
    pub name: String,

    /// 订阅者列表（必填，最大 50 个；member 目前仅支持 chat 类型）
    pub subscribers: Vec<TasklistMemberDetail>,

    /// 订阅的清单动态 event key 列表（必填，可为空数组表示不通知任何动态；元素不可重复）
    pub include_keys: Vec<i32>,

    /// 该订阅是否停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// 要更新的订阅数据（tasklist_activity_subscription）
#[derive(Debug, Clone, Serialize, Default)]
pub struct ActivitySubscriptionInput {
    /// 订阅名称（如更新，不能为空，最大 50 个字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// 订阅者列表（如更新，最大 50 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TasklistMemberDetail>>,

    /// 要订阅的清单动态 event key 列表（可为空数组；元素不可重复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_keys: Option<Vec<i32>>,

    /// 该订阅是否停用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// 更新动态订阅请求体
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateActivitySubscriptionBody {
    /// 要更新的订阅数据
    pub activity_subscription: ActivitySubscriptionInput,

    /// 要更新的字段列表（1-20 个），可选值：name / include_keys / subscribers / disabled
    pub update_fields: Vec<String>,
}

/// 动态订阅信息（tasklist_activity_subscription）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ActivitySubscription {
    /// 订阅 GUID
    #[serde(default)]
    pub guid: Option<String>,

    /// 订阅名称
    #[serde(default)]
    pub name: Option<String>,

    /// 订阅者列表
    #[serde(default)]
    pub subscribers: Vec<TasklistMemberDetail>,

    /// 订阅的清单动态 event key 列表
    #[serde(default)]
    pub include_keys: Vec<i32>,

    /// 该订阅是否停用
    #[serde(default)]
    pub disabled: Option<bool>,
}

/// 创建动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct CreateActivitySubscriptionResponse {
    /// 清单动态订阅
    pub activity_subscription: ActivitySubscription,
}

/// 获取动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct GetActivitySubscriptionResponse {
    /// 清单动态订阅
    pub activity_subscription: ActivitySubscription,
}

/// 更新动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateActivitySubscriptionResponse {
    /// 更新后的清单动态订阅
    pub activity_subscription: ActivitySubscription,
}

/// 删除动态订阅响应
#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeleteActivitySubscriptionResponse {}

/// 列取动态订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct ListActivitySubscriptionsResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,

    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,

    /// 订阅列表
    #[serde(default)]
    pub items: Vec<ActivitySubscription>,
}
