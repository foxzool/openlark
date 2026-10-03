//! 会议室历史版本相关响应结构
//!
//! 字段对齐飞书历史版文档 Response body example（apiSchema 为 GuideDocumentType，
//! 无结构化 schema，以官方 JSON 示例为准）。

use std::collections::HashMap;

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Building
// ---------------------------------------------------------------------------

/// 建筑物信息（list / batch_get 共用）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildingInfo {
    /// 建筑物 ID。
    pub building_id: String,
    /// 建筑物名称。
    #[serde(default)]
    pub name: Option<String>,
    /// 描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 楼层列表。
    #[serde(default)]
    pub floors: Option<Vec<String>>,
    /// 国家/地区 ID。
    #[serde(default)]
    pub country_id: Option<String>,
    /// 城市 ID。
    #[serde(default)]
    pub district_id: Option<String>,
}

/// 创建建筑物响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateBuildingResponse {
    /// 建筑物 ID。
    pub building_id: String,
}

impl ApiResponseTrait for CreateBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取建筑物列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListBuildingResponse {
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 建筑物列表。
    #[serde(default)]
    pub buildings: Vec<BuildingInfo>,
}

impl ApiResponseTrait for ListBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询建筑物详情响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetBuildingResponse {
    /// 建筑物列表。
    #[serde(default)]
    pub buildings: Vec<BuildingInfo>,
}

impl ApiResponseTrait for BatchGetBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 建筑物 ID 映射（按自定义 ID 查询）。
///
/// 注意：官方示例字段名为 `custom_bulding_id`（拼写缺 i），保持原样对齐。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuildingIdMapping {
    /// 建筑物 ID。
    pub building_id: String,
    /// 租户自定义建筑物 ID（官方字段名拼写为 bulding）。
    #[serde(default)]
    pub custom_bulding_id: Option<String>,
}

/// 查询建筑物 ID 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetBuildingIdResponse {
    /// 建筑物 ID 映射列表。
    #[serde(default)]
    pub buildings: Vec<BuildingIdMapping>,
}

impl ApiResponseTrait for BatchGetBuildingIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新建筑物响应（官方示例无 `data` 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpdateBuildingResponse {}

impl ApiResponseTrait for UpdateBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

/// 删除建筑物响应（官方示例无 `data` 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DeleteBuildingResponse {}

impl ApiResponseTrait for DeleteBuildingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

// ---------------------------------------------------------------------------
// Room
// ---------------------------------------------------------------------------

/// 会议室信息（list / batch_get 共用）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomInfo {
    /// 会议室 ID。
    pub room_id: String,
    /// 所属建筑物 ID。
    #[serde(default)]
    pub building_id: Option<String>,
    /// 所属建筑物名称。
    #[serde(default)]
    pub building_name: Option<String>,
    /// 容量。
    #[serde(default)]
    pub capacity: Option<u32>,
    /// 描述。
    #[serde(default)]
    pub description: Option<String>,
    /// 展示 ID。
    #[serde(default)]
    pub display_id: Option<String>,
    /// 楼层名称。
    #[serde(default)]
    pub floor_name: Option<String>,
    /// 是否停用。
    #[serde(default)]
    pub is_disabled: Option<bool>,
    /// 会议室名称。
    #[serde(default)]
    pub name: Option<String>,
}

/// 创建会议室响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoomResponse {
    /// 会议室 ID。
    pub room_id: String,
}

impl ApiResponseTrait for CreateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取会议室列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRoomResponse {
    /// 分页标记。
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多。
    #[serde(default)]
    pub has_more: Option<bool>,
    /// 会议室列表。
    #[serde(default)]
    pub rooms: Vec<RoomInfo>,
}

impl ApiResponseTrait for ListRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量查询会议室详情响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRoomResponse {
    /// 会议室列表。
    #[serde(default)]
    pub rooms: Vec<RoomInfo>,
}

impl ApiResponseTrait for BatchGetRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 会议室 ID 映射（按自定义 ID 查询）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoomIdMapping {
    /// 会议室 ID。
    pub room_id: String,
    /// 租户自定义会议室 ID。
    #[serde(default)]
    pub custom_room_id: Option<String>,
}

/// 查询会议室 ID 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetRoomIdResponse {
    /// 会议室 ID 映射列表。
    #[serde(default)]
    pub rooms: Vec<RoomIdMapping>,
}

impl ApiResponseTrait for BatchGetRoomIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新会议室响应（官方示例无 `data` 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoomResponse {}

impl ApiResponseTrait for UpdateRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

/// 删除会议室响应（官方示例无 `data` 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DeleteRoomResponse {}

impl ApiResponseTrait for DeleteRoomResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

// ---------------------------------------------------------------------------
// Country / District
// ---------------------------------------------------------------------------

/// 国家/地区信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountryInfo {
    /// 国家/地区 ID。
    pub country_id: String,
    /// 名称。
    #[serde(default)]
    pub name: Option<String>,
}

/// 获取国家/地区列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCountryResponse {
    /// 国家/地区列表。
    #[serde(default)]
    pub countries: Vec<CountryInfo>,
}

impl ApiResponseTrait for ListCountryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 城市信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DistrictInfo {
    /// 城市 ID。
    pub district_id: String,
    /// 名称。
    #[serde(default)]
    pub name: Option<String>,
}

/// 获取城市列表响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListDistrictResponse {
    /// 城市列表。
    #[serde(default)]
    pub districts: Vec<DistrictInfo>,
}

impl ApiResponseTrait for ListDistrictResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ---------------------------------------------------------------------------
// Freebusy
// ---------------------------------------------------------------------------

/// 日程组织者信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeBusyOrganizer {
    /// 组织者姓名。
    #[serde(default)]
    pub name: Option<String>,
    /// 组织者 open_id。
    #[serde(default)]
    pub open_id: Option<String>,
}

/// 会议室忙闲时段。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreeBusySlot {
    /// 开始时间（RFC3339）。
    #[serde(default)]
    pub start_time: Option<String>,
    /// 结束时间（RFC3339）。
    #[serde(default)]
    pub end_time: Option<String>,
    /// 日程 UID。
    #[serde(default)]
    pub uid: Option<String>,
    /// 重复日程原始时间（Unix 秒；非重复为 0）。
    #[serde(default)]
    pub original_time: Option<i64>,
    /// 组织者信息。
    #[serde(default)]
    pub organizer_info: Option<FreeBusyOrganizer>,
}

/// 查询会议室忙闲响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetFreebusyResponse {
    /// 查询时间上限。
    #[serde(default)]
    pub time_max: Option<String>,
    /// 查询时间下限。
    #[serde(default)]
    pub time_min: Option<String>,
    /// 按会议室 ID 索引的忙闲时段列表。
    #[serde(default)]
    pub free_busy: HashMap<String, Vec<FreeBusySlot>>,
}

impl ApiResponseTrait for BatchGetFreebusyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ---------------------------------------------------------------------------
// Instance reply
// ---------------------------------------------------------------------------

/// 回复会议室日程实例响应（官方示例无 `data` 字段）。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ReplyInstanceResponse {}

impl ApiResponseTrait for ReplyInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    fn empty_success() -> Option<Self> {
        Some(Self {})
    }
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// 视频会议信息（日程主题详情）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummaryVchat {
    /// 视频会议类型（`vc` / `third_party` / `no_meeting` 等）。
    #[serde(default)]
    pub vc_type: Option<String>,
    /// 第三方视频会议 icon 类型。
    #[serde(default)]
    pub icon_type: Option<String>,
    /// 第三方视频会议文案。
    #[serde(default)]
    pub description: Option<String>,
    /// 视频会议 URL。
    #[serde(default)]
    pub meeting_url: Option<String>,
}

/// 会议室日程主题与会议详情。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummaryEventInfo {
    /// 重复日程原始时间。
    #[serde(default)]
    pub original_time: Option<i64>,
    /// 日程主题。
    #[serde(default)]
    pub summary: Option<String>,
    /// 日程 UID。
    #[serde(default)]
    pub uid: Option<String>,
    /// 视频会议信息。
    #[serde(default)]
    pub vchat: Option<SummaryVchat>,
}

/// 查询失败的日程信息（`ErrorEventUids` 元素）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SummaryErrorEvent {
    /// 日程 UID。
    #[serde(default)]
    pub uid: Option<String>,
    /// 重复日程原始时间。
    #[serde(default)]
    pub original_time: Option<i64>,
    /// 错误信息。
    #[serde(default)]
    pub error_msg: Option<String>,
}

/// 查询会议室日程主题和会议详情响应。
///
/// 官方文档使用 PascalCase 字段名（`ErrorEventUids` / `EventInfos`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetSummaryResponse {
    /// 没有查询到的日程信息。
    #[serde(rename = "ErrorEventUids", default)]
    pub error_event_uids: Vec<SummaryErrorEvent>,
    /// 事件详情列表。
    #[serde(rename = "EventInfos", default)]
    pub event_infos: Vec<SummaryEventInfo>,
}

impl ApiResponseTrait for BatchGetSummaryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserialize_list_building_from_official_example() {
        let v = json!({
            "page_token": "1",
            "has_more": true,
            "buildings": [{
                "building_id": "omb_8ec170b937536a5d87c23b418b83f9bb",
                "description": "Some description",
                "floors": ["F1"],
                "name": "Building name",
                "country_id": "country id",
                "district_id": "district id"
            }]
        });
        let resp: ListBuildingResponse = serde_json::from_value(v).unwrap();
        assert!(resp.has_more.unwrap());
        assert_eq!(resp.buildings[0].name.as_deref(), Some("Building name"));
        assert_eq!(
            resp.buildings[0].floors.as_ref().unwrap(),
            &vec!["F1".to_string()]
        );
    }

    #[test]
    fn deserialize_batch_get_building_id_preserves_official_typo() {
        let v = json!({
            "buildings": [{
                "building_id": "omb_xxx",
                "custom_bulding_id": "test01"
            }]
        });
        let resp: BatchGetBuildingIdResponse = serde_json::from_value(v).unwrap();
        assert_eq!(
            resp.buildings[0].custom_bulding_id.as_deref(),
            Some("test01")
        );
    }

    #[test]
    fn deserialize_room_list_from_official_example() {
        let v = json!({
            "page_token": "1",
            "has_more": true,
            "rooms": [{
                "room_id": "omm_eada1d61a550955240c28757e7dec3af",
                "building_id": "omb_8ec170b937536a5d87c23b418b83f9bb",
                "building_name": "Building name",
                "capacity": 14,
                "description": "Some description",
                "display_id": "FM537532166",
                "floor_name": "F1",
                "is_disabled": false,
                "name": "Room name"
            }]
        });
        let resp: ListRoomResponse = serde_json::from_value(v).unwrap();
        assert_eq!(resp.rooms[0].capacity, Some(14));
        assert_eq!(resp.rooms[0].is_disabled, Some(false));
    }

    #[test]
    fn deserialize_freebusy_map_keys_are_room_ids() {
        let v = json!({
            "time_max": "2019-09-04T09:45:00+08:00",
            "time_min": "2019-09-04T08:45:00+08:00",
            "free_busy": {
                "omm_83d09ad4f6896e02029a6a075f71c9d1": [{
                    "end_time": "2019-09-04T09:30:00+08:00",
                    "start_time": "2019-09-04T09:00:00+08:00",
                    "original_time": 0,
                    "uid": "bff6b51f-b7c1-40c6-b8ef-aef966c9ffc7",
                    "organizer_info": {
                        "name": "张三",
                        "open_id": "ou_xxx"
                    }
                }]
            }
        });
        let resp: BatchGetFreebusyResponse = serde_json::from_value(v).unwrap();
        let slots = resp
            .free_busy
            .get("omm_83d09ad4f6896e02029a6a075f71c9d1")
            .unwrap();
        assert_eq!(slots[0].original_time, Some(0));
        assert_eq!(
            slots[0].organizer_info.as_ref().unwrap().name.as_deref(),
            Some("张三")
        );
    }

    #[test]
    fn deserialize_summary_pascal_case_fields() {
        let v = json!({
            "ErrorEventUids": [{
                "uid": "missing-uid",
                "original_time": 0,
                "error_msg": "not found"
            }],
            "EventInfos": [{
                "original_time": 0,
                "summary": "test",
                "uid": "a04dbea1-86b9-4372-aa8d-64ebe801be2a",
                "vchat": {
                    "meeting_url": "https://vc.feishu.cn/j/935314044",
                    "vc_type": "third_party",
                    "icon_type": "default",
                    "description": "外部会议"
                }
            }]
        });
        let resp: BatchGetSummaryResponse = serde_json::from_value(v).unwrap();
        assert_eq!(resp.error_event_uids.len(), 1);
        assert_eq!(
            resp.error_event_uids[0].error_msg.as_deref(),
            Some("not found")
        );
        assert_eq!(resp.event_infos[0].summary.as_deref(), Some("test"));
        let vchat = resp.event_infos[0].vchat.as_ref().unwrap();
        assert_eq!(vchat.vc_type.as_deref(), Some("third_party"));
        assert_eq!(vchat.icon_type.as_deref(), Some("default"));
        assert_eq!(vchat.description.as_deref(), Some("外部会议"));
    }

    #[test]
    fn deserialize_empty_delete_response() {
        let resp: DeleteBuildingResponse = serde_json::from_value(json!({})).unwrap();
        let _ = resp;
        let resp2 = DeleteBuildingResponse::default();
        assert_eq!(resp2, DeleteBuildingResponse {});
    }

    #[test]
    fn deserialize_country_and_district_list() {
        let countries: ListCountryResponse = serde_json::from_value(json!({
            "countries": [{"country_id": "1814991", "name": "中国"}]
        }))
        .unwrap();
        assert_eq!(countries.countries[0].name.as_deref(), Some("中国"));

        let districts: ListDistrictResponse = serde_json::from_value(json!({
            "districts": [{"district_id": "1796236", "name": "上海"}]
        }))
        .unwrap();
        assert_eq!(districts.districts[0].district_id, "1796236");
    }
}
