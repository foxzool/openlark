//! 群公告相关模型（不算 API）

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 获取群公告信息响应 data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetChatAnnouncementResponse {
    /// 公告内容。
    pub content: String,
    /// 公告版本号。
    pub revision: String,
    /// 创建时间。
    pub create_time: String,
    /// 更新时间。
    pub update_time: String,
    /// 创建者 ID 类型。
    pub owner_id_type: String,
    /// 创建者 ID。
    pub owner_id: String,
    /// 更新者 ID 类型。
    pub modifier_id_type: String,
    /// 更新者 ID。
    pub modifier_id: String,
}

impl ApiResponseTrait for GetChatAnnouncementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新群公告信息请求体
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchChatAnnouncementBody {
    /// 当前公告版本号。
    pub revision: String,
    /// 公告内容编辑指令列表。
    pub requests: Vec<String>,
}
