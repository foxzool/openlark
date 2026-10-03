/// 文件订阅（subscription）相关模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use openlark_core::api::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 订阅信息（get/create/patch 的 data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    /// 订阅关系 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// 订阅类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    /// 文档响应字段拼写为 is_subcribe（注意不是 is_subscribe）
    #[serde(rename = "is_subcribe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribe: Option<bool>,
    /// 文件类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
}

impl ApiResponseTrait for Subscription {}
