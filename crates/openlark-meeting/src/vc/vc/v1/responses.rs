//! 会议相关响应结构
//!
//! 定义视频会议 API 的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    /// 新创建的会议室 ID。
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询会议室详情请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomRequest {
    /// 用户 ID 类型。
    pub user_id_type: Option<String>,
    /// 用户 ID。
    pub user_id: Option<String>,
}

/// 查询会议室详情响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoomResponse {
    /// 会议室 ID。
    pub room_id: String,
    /// 会议室名称。
    pub room_name: String,
    /// 容量。
    pub capacity: u32,
    /// 设备列表。
    pub devices: Option<Vec<DeviceInfo>>,
    /// 楼层。
    pub floor: Option<String>,
    /// 描述。
    pub description: Option<String>,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议室设备信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// 设备 ID。
    pub device_id: String,
    /// 设备名称。
    pub device_name: String,
    /// 设备类型。
    pub device_type: String,
}

/// 查询会议室列表响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoomResponse {
    /// 会议室列表。
    pub rooms: Vec<RoomInfo>,
    /// 是否还有更多数据。
    pub has_more: Option<bool>,
    /// 分页令牌。
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议室摘要信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    /// 会议室 ID。
    pub room_id: String,
    /// 会议室名称。
    pub room_name: String,
    /// 容量。
    pub capacity: u32,
    /// 状态。
    pub status: String,
}

/// 批量查询会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgetRoomResponse {
    /// 批量查询返回的会议室列表。
    pub rooms: Vec<RoomInfo>,
}

impl ApiResponseTrait for MgetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRoomResponse {
    /// 更新后的会议室 ID。
    pub room_id: String,
}

impl ApiResponseTrait for PatchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRoomResponse {
    /// 搜索结果列表。
    pub rooms: Vec<RoomInfo>,
    /// 是否还有更多数据。
    pub has_more: Option<bool>,
    /// 分页令牌。
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoomResponse {}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
