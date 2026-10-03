//! 视频会议和日历核心 request/response 模型的序列化契约测试。
#![cfg(all(feature = "vc", feature = "calendar"))]

use openlark_meeting::calendar::calendar::v4::calendar::models::{
    Calendar, CalendarPermissions, CreateCalendarResponse, GetCalendarResponse,
};
use openlark_meeting::calendar::calendar::v4::responses::{
    BatchGetFreebusyResponse, CalendarInfo, CreateExchangeBindingResponse, DeleteCalendarResponse,
    DeleteExchangeBindingResponse, ExchangeBindingInfo, FreebusyItem, GetExchangeBindingResponse,
    ListCalendarResponse, ListFreebusyResponse, TimeRange,
};
use openlark_meeting::vc::vc::v1::reserve::apply::ApplyReserveResponse;
use openlark_meeting::vc::vc::v1::responses::{
    CreateRoomResponse, DeviceInfo, GetRoomResponse as VcGetRoomResponse, ListRoomResponse,
    MgetRoomResponse, PatchRoomResponse, RoomInfo, SearchRoomResponse,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, from_value, json, to_value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

// ── VC: 会议室响应（responses.rs）──────────────────────────

#[test]
fn vc_create_room_response_contract() {
    let resp = CreateRoomResponse {
        room_id: "room_abc123".to_string(),
    };
    assert_json_contract(&resp, json!({ "room_id": "room_abc123" }));

    let parsed: CreateRoomResponse = parse_contract(json!({ "room_id": "room_xyz" }));
    assert_eq!(parsed.room_id, "room_xyz");
}

#[test]
fn vc_get_room_response_contract() {
    let resp = VcGetRoomResponse {
        room_id: "room_001".to_string(),
        room_name: "大会议室A".to_string(),
        capacity: 20,
        devices: Some(vec![DeviceInfo {
            device_id: "dev_1".to_string(),
            device_name: "投影仪".to_string(),
            device_type: "projector".to_string(),
        }]),
        floor: Some("3F".to_string()),
        description: Some("可容纳20人".to_string()),
    };
    assert_json_contract(
        &resp,
        json!({
            "room_id": "room_001",
            "room_name": "大会议室A",
            "capacity": 20,
            "devices": [{ "device_id": "dev_1", "device_name": "投影仪", "device_type": "projector" }],
            "floor": "3F",
            "description": "可容纳20人"
        }),
    );

    let parsed: VcGetRoomResponse = parse_contract(json!({
        "room_id": "room_002",
        "room_name": "小会议室",
        "capacity": 6,
        "devices": null,
        "floor": null,
        "description": null
    }));
    assert_eq!(parsed.room_id, "room_002");
    assert!(parsed.devices.is_none());
}

#[test]
fn vc_list_room_response_contract() {
    let resp = ListRoomResponse {
        rooms: vec![
            RoomInfo {
                room_id: "r1".to_string(),
                room_name: "会议室1".to_string(),
                capacity: 10,
                status: "active".to_string(),
            },
            RoomInfo {
                room_id: "r2".to_string(),
                room_name: "会议室2".to_string(),
                capacity: 15,
                status: "inactive".to_string(),
            },
        ],
        has_more: Some(true),
        page_token: Some("next_page_token".to_string()),
    };
    assert_eq!(resp.rooms.len(), 2);
    assert_json_contract(
        &resp,
        json!({
            "rooms": [
                { "room_id": "r1", "room_name": "会议室1", "capacity": 10, "status": "active" },
                { "room_id": "r2", "room_name": "会议室2", "capacity": 15, "status": "inactive" }
            ],
            "has_more": true,
            "page_token": "next_page_token"
        }),
    );
}

#[test]
fn vc_mget_room_response_contract() {
    let resp = MgetRoomResponse {
        rooms: vec![RoomInfo {
            room_id: "r_batch".to_string(),
            room_name: "批量会议室".to_string(),
            capacity: 8,
            status: "active".to_string(),
        }],
    };
    assert_eq!(resp.rooms.len(), 1);
    assert_json_contract(
        &resp,
        json!({
            "rooms": [{ "room_id": "r_batch", "room_name": "批量会议室", "capacity": 8, "status": "active" }]
        }),
    );
}

#[test]
fn vc_patch_room_response_contract() {
    let resp = PatchRoomResponse {
        room_id: "room_patch".to_string(),
    };
    assert_json_contract(&resp, json!({ "room_id": "room_patch" }));
}

#[test]
fn vc_search_room_response_contract() {
    let resp = SearchRoomResponse {
        rooms: vec![],
        has_more: Some(false),
        page_token: None,
    };
    assert_json_contract(
        &resp,
        json!({ "rooms": [], "has_more": false, "page_token": null }),
    );
}

// ── VC: 预约会议响应 ──────────────────────────────────────

#[test]
fn vc_apply_reserve_response_contract() {
    let resp = ApplyReserveResponse {
        meeting_id: "mtg_001".to_string(),
        reserve_id: "res_001".to_string(),
    };
    assert_json_contract(
        &resp,
        json!({ "meeting_id": "mtg_001", "reserve_id": "res_001" }),
    );

    let parsed: ApplyReserveResponse =
        parse_contract(json!({ "meeting_id": "mtg_002", "reserve_id": "res_002" }));
    assert_eq!(
        parsed,
        ApplyReserveResponse {
            meeting_id: "mtg_002".to_string(),
            reserve_id: "res_002".to_string(),
        }
    );
}

// ── Calendar: 日历模型（models.rs）────────────────────────

#[test]
fn calendar_permissions_contract() {
    let perms = CalendarPermissions {
        is_readable: Some(true),
        is_writable: Some(false),
    };
    assert_json_contract(&perms, json!({ "is_readable": true, "is_writable": false }));

    let parsed: CalendarPermissions =
        parse_contract(json!({ "is_readable": null, "is_writable": null }));
    assert!(parsed.is_readable.is_none());
}

#[test]
fn calendar_model_contract() {
    let cal = Calendar {
        calendar_id: "cal_001".to_string(),
        summary: "工作日历".to_string(),
        description: Some("团队共享".to_string()),
        color: Some("#1890ff".to_string()),
        permissions: Some(CalendarPermissions {
            is_readable: Some(true),
            is_writable: Some(true),
        }),
        primary: Some(true),
        calendar_type: Some("shared".to_string()),
    };
    assert_json_contract(
        &cal,
        json!({
            "calendar_id": "cal_001",
            "summary": "工作日历",
            "description": "团队共享",
            "color": "#1890ff",
            "permissions": { "is_readable": true, "is_writable": true },
            "primary": true,
            "calendar_type": "shared"
        }),
    );

    let parsed: Calendar = parse_contract(json!({
        "calendar_id": "cal_min",
        "summary": "最小日历",
        "description": null,
        "color": null,
        "permissions": null,
        "primary": null,
        "calendar_type": null
    }));
    assert_eq!(parsed.calendar_id, "cal_min");
    assert!(parsed.description.is_none());
}

#[test]
fn calendar_get_and_create_response_roundtrip() {
    let get_resp = GetCalendarResponse {
        calendar: Calendar {
            calendar_id: "cal_get".to_string(),
            summary: "获取日历".to_string(),
            description: None,
            color: None,
            permissions: None,
            primary: None,
            calendar_type: None,
        },
    };
    let json_val = to_value(&get_resp).unwrap();
    let roundtrip: GetCalendarResponse = from_value(json_val).unwrap();
    assert_eq!(roundtrip, get_resp);

    let create_resp = CreateCalendarResponse {
        calendar: Calendar {
            calendar_id: "cal_new".to_string(),
            summary: "新建日历".to_string(),
            description: Some("项目日程".to_string()),
            color: Some("#52c41a".to_string()),
            permissions: None,
            primary: Some(false),
            calendar_type: None,
        },
    };
    assert_eq!(create_resp.calendar.calendar_id, "cal_new");
    assert_json_contract(
        &create_resp.calendar,
        json!({
            "calendar_id": "cal_new",
            "summary": "新建日历",
            "description": "项目日程",
            "color": "#52c41a",
            "permissions": null,
            "primary": false,
            "calendar_type": null
        }),
    );
}

// ── Calendar: responses.rs 中的响应结构 ────────────────────

#[test]
fn calendar_list_response_contract() {
    let resp = ListCalendarResponse {
        calendars: vec![CalendarInfo {
            calendar_id: "c1".to_string(),
            name: "日历1".to_string(),
            calendar_type: "primary".to_string(),
            description: None,
        }],
        has_more: Some(false),
        page_token: None,
    };
    assert_eq!(resp.calendars.len(), 1);
    assert_json_contract(
        &resp,
        json!({
            "calendars": [{ "calendar_id": "c1", "name": "日历1", "calendar_type": "primary", "description": null }],
            "has_more": false,
            "page_token": null
        }),
    );
}

#[test]
fn calendar_delete_response_contract() {
    let resp = DeleteCalendarResponse {};
    assert_json_contract(&resp, json!({}));
}

#[test]
fn calendar_freebusy_response_contracts() {
    let item = FreebusyItem {
        user_id: "ou_001".to_string(),
        time_ranges: vec![TimeRange {
            start: "2026-04-20T09:00:00+08:00".to_string(),
            end: "2026-04-20T10:00:00+08:00".to_string(),
        }],
    };
    assert_json_contract(
        &item,
        json!({
            "user_id": "ou_001",
            "time_ranges": [{ "start": "2026-04-20T09:00:00+08:00", "end": "2026-04-20T10:00:00+08:00" }]
        }),
    );

    let list_resp = ListFreebusyResponse {
        data: vec![item.clone()],
    };
    assert_eq!(list_resp.data.len(), 1);

    let batch_resp = BatchGetFreebusyResponse { data: vec![item] };
    let roundtrip: BatchGetFreebusyResponse = from_value(to_value(&batch_resp).unwrap()).unwrap();
    assert_eq!(roundtrip.data.len(), 1);
    assert_eq!(roundtrip.data[0].user_id, "ou_001");
}

#[test]
fn calendar_exchange_binding_contracts() {
    let create_resp = CreateExchangeBindingResponse {
        binding_id: "bind_001".to_string(),
    };
    assert_json_contract(&create_resp, json!({ "binding_id": "bind_001" }));

    let delete_resp = DeleteExchangeBindingResponse {};
    assert_json_contract(&delete_resp, json!({}));

    let get_resp = GetExchangeBindingResponse {
        binding: ExchangeBindingInfo {
            binding_id: "bind_002".to_string(),
            email: "user@example.com".to_string(),
            status: "active".to_string(),
        },
    };
    assert_json_contract(
        &get_resp,
        json!({
            "binding": { "binding_id": "bind_002", "email": "user@example.com", "status": "active" }
        }),
    );
}

// ── 双向一致性 roundtrip 测试 ───────────────────────────────

#[test]
fn roundtrip_vc_room_info() {
    let room_info = RoomInfo {
        room_id: "r_rt".to_string(),
        room_name: "Roundtrip会议室".to_string(),
        capacity: 12,
        status: "active".to_string(),
    };
    let json_val = to_value(&room_info).unwrap();
    let parsed: RoomInfo = from_value(json_val).unwrap();
    assert_eq!(parsed.room_id, room_info.room_id);
    assert_eq!(parsed.room_name, room_info.room_name);
    assert_eq!(parsed.capacity, room_info.capacity);
    assert_eq!(parsed.status, room_info.status);
}

#[test]
fn roundtrip_calendar() {
    let cal = Calendar {
        calendar_id: "cal_rt".to_string(),
        summary: "Roundtrip测试".to_string(),
        description: Some("双向一致性验证".to_string()),
        color: None,
        permissions: Some(CalendarPermissions {
            is_readable: Some(true),
            is_writable: Some(true),
        }),
        primary: Some(false),
        calendar_type: Some("primary".to_string()),
    };
    let json_val = to_value(&cal).unwrap();
    let parsed: Calendar = from_value(json_val).unwrap();
    assert_eq!(parsed, cal);
}

#[test]
fn roundtrip_apply_reserve_response() {
    let resp = ApplyReserveResponse {
        meeting_id: "mtg_rt".to_string(),
        reserve_id: "res_rt".to_string(),
    };
    let json_val = to_value(&resp).unwrap();
    let parsed: ApplyReserveResponse = from_value(json_val).unwrap();
    assert_eq!(parsed, resp);
}

#[test]
fn roundtrip_device_info() {
    let device = DeviceInfo {
        device_id: "dev_rt".to_string(),
        device_name: "白板".to_string(),
        device_type: "whiteboard".to_string(),
    };
    let json_val = to_value(&device).unwrap();
    let parsed: DeviceInfo = from_value(json_val).unwrap();
    assert_eq!(parsed.device_id, device.device_id);
    assert_eq!(parsed.device_name, device.device_name);
    assert_eq!(parsed.device_type, device.device_type);
}
