//! 消息流卡片相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 即时提醒请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedCardTimeSensitiveBody {
    /// 是否开启即时提醒。
    pub time_sensitive: bool,
    /// 目标用户 ID 列表。
    pub user_ids: Vec<String>,
}

/// 单个用户失败原因
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FailedUserReason {
    /// 错误码。
    pub error_code: i32,
    /// 错误信息。
    pub error_message: String,
    /// 失败用户 ID。
    pub user_id: String,
}

/// 即时提醒响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedCardActionResponse {
    /// 失败用户原因列表。
    #[serde(default)]
    pub failed_user_reasons: Option<Vec<FailedUserReason>>,
}

impl ApiResponseTrait for FeedCardActionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
