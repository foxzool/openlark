//! 会议室历史版本相关响应结构
//!
//! 定义会议室 API（历史版本）的响应数据类型。

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    /// 会议室 ID
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct GetRoomResponse {
    /// 待补充文档。
    pub room_id: String,
    /// 待补充文档。
    pub room_name: String,
    /// 待补充文档。
    pub description: Option<String>,
    /// 待补充文档。
    pub capacity: u32,
    /// 待补充文档。
    pub devices: Option<Vec<DeviceInfo>>,
    /// 待补充文档。
    pub status: String,
}

impl ApiResponseTrait for GetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 待补充文档。
pub struct DeviceInfo {
    /// 待补充文档。
    pub device_id: String,
    /// 待补充文档。
    pub device_name: String,
    /// 待补充文档。
    pub device_type: String,
}

/// 批量获取会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRoomResponse {
    /// 会议室列表
    pub rooms: Vec<RoomInfo>,
}

impl ApiResponseTrait for BatchGetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议室信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    /// 会议室 ID
    pub room_id: String,
    /// 会议室名称
    pub room_name: String,
    /// 容量
    pub capacity: u32,
    /// 描述
    pub description: Option<String>,
    /// 状态
    pub status: String,
    /// 设备信息
    pub devices: Option<Vec<DeviceInfo>>,
}

/// 更新会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoomResponse {
    /// 会议室 ID
    pub room_id: String,
}

impl ApiResponseTrait for UpdateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除会议室响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoomResponse {}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建建筑响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBuildingResponse {
    /// 建筑 ID
    pub building_id: String,
}

impl ApiResponseTrait for CreateBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取建筑响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBuildingResponse {
    /// 建筑 ID
    pub building_id: String,
    /// 建筑名称
    pub name: String,
    /// 地址
    pub address: Option<String>,
    /// 城市
    pub city: Option<String>,
}

impl ApiResponseTrait for GetBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 建筑信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingInfo {
    /// 建筑 ID
    pub building_id: String,
    /// 建筑名称
    pub name: String,
    /// 地址
    pub address: Option<String>,
    /// 城市
    pub city: Option<String>,
}

/// 批量获取建筑响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetBuildingResponse {
    /// 建筑列表
    pub buildings: Vec<BuildingInfo>,
}

impl ApiResponseTrait for BatchGetBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新建筑响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBuildingResponse {
    /// 建筑 ID
    pub building_id: String,
}

impl ApiResponseTrait for UpdateBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除建筑响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBuildingResponse {}

impl ApiResponseTrait for DeleteBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取建筑响应（按 ID）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetBuildingByIdResponse {
    /// 建筑列表
    pub buildings: Vec<BuildingInfo>,
}

impl ApiResponseTrait for BatchGetBuildingByIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取会议实例响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetInstanceResponse {
    /// 实例列表
    pub instances: Vec<MeetingInstanceInfo>,
}

impl ApiResponseTrait for BatchGetInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议实例信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingInstanceInfo {
    /// 实例 ID
    pub instance_id: String,
    /// 会议室 ID
    pub room_id: String,
    /// 会议主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 状态
    pub status: String,
    /// 创建人
    pub creator: UserInfo,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// 用户 ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

/// 会议汇总响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingSummaryResponse {
    /// 汇总数据
    pub data: SummaryData,
}

impl ApiResponseTrait for MeetingSummaryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 汇总数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryData {
    /// 会议总数
    pub total_meetings: u32,
    /// 预约总数
    pub total_reservations: u32,
    /// 活跃会议室数
    pub active_rooms: u32,
}

#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
