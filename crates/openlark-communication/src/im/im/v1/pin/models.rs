//! Pin 消息相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// Pin 消息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pin {
    /// 消息 ID。
    pub message_id: String,
    /// 群聊 ID。
    pub chat_id: String,
    /// 操作者 ID。
    pub operator_id: String,
    /// 操作者 ID 类型。
    pub operator_id_type: String,
    /// 创建时间。
    pub create_time: String,
}

/// Pin 消息请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePinBody {
    /// 消息 ID。
    pub message_id: String,
}

/// Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePinResponse {
    /// Pin 消息详情。
    pub pin: Pin,
}

impl ApiResponseTrait for CreatePinResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群内 Pin 消息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPinsResponse {
    /// Pin 消息列表。
    #[serde(default)]
    pub items: Option<Vec<Pin>>,
    /// 是否还有更多数据。
    pub has_more: bool,
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListPinsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
