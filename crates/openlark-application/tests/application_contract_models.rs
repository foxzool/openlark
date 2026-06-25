//! Representative contract tests for Application request/response models.
#![cfg(feature = "v1")]

use openlark_application::application::application::v1::app::GetAppResponse;
use openlark_application::workplace::workplace::v1::workplace_access_data::search::{
    AccessDataSearchWorkplaceResponse, WorkplaceAccessData,
};
use openlark_application::workplace::workplace::v1::workplace_block_access_data::search::{
    AccessDataSearchBlockResponse, BlockAccessData,
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

#[test]
fn get_app_response_contract() {
    let response: GetAppResponse = parse_contract(json!({
        "app_id": "cli_a5xxxxxxxx",
        "app_name": "测试应用",
        "app_type": "market",
        "description": "用于集成测试的应用"
    }));
    assert_eq!(response.app_id, "cli_a5xxxxxxxx");
    assert_eq!(response.app_name, "测试应用");
    assert_eq!(response.app_type, "market");
    assert_eq!(response.description.as_deref(), Some("用于集成测试的应用"));

    let response_no_desc: GetAppResponse = parse_contract(json!({
        "app_id": "cli_b3xxxxxxxx",
        "app_name": "简单应用",
        "app_type": "custom"
    }));
    assert_eq!(response_no_desc.description, None);
}

#[test]
fn get_app_response_roundtrip() {
    let original = json!({
        "app_id": "cli_a5xxxxxxxx",
        "app_name": "测试应用",
        "app_type": "market",
        "description": "用于集成测试的应用"
    });
    let response: GetAppResponse = parse_contract(original);
    assert_eq!(response.app_id, "cli_a5xxxxxxxx");
    assert_eq!(response.app_name, "测试应用");
    assert_eq!(response.app_type, "market");
    assert_eq!(response.description.as_deref(), Some("用于集成测试的应用"));
}

#[test]
fn workplace_access_data_contract() {
    let data: WorkplaceAccessData = parse_contract(json!({
        "date": "2026-04-20",
        "visit_count": 1024,
        "visitor_count": 256
    }));
    assert_eq!(data.date, "2026-04-20");
    assert_eq!(data.visit_count, 1024);
    assert_eq!(data.visitor_count, 256);

    assert_json_contract(
        &data,
        json!({
            "date": "2026-04-20",
            "visit_count": 1024,
            "visitor_count": 256
        }),
    );
}

#[test]
fn workplace_access_data_search_response_contract() {
    let response: AccessDataSearchWorkplaceResponse = parse_contract(json!({
        "items": [
            {
                "date": "2026-04-18",
                "visit_count": 512,
                "visitor_count": 128
            },
            {
                "date": "2026-04-19",
                "visit_count": 768,
                "visitor_count": 192
            },
            {
                "date": "2026-04-20",
                "visit_count": 1024,
                "visitor_count": 256
            }
        ]
    }));
    assert_eq!(response.items.len(), 3);
    assert_eq!(response.items[0].date, "2026-04-18");
    assert_eq!(response.items[2].visit_count, 1024);

    assert_json_contract(
        &response,
        json!({
            "items": [
                {
                    "date": "2026-04-18",
                    "visit_count": 512,
                    "visitor_count": 128
                },
                {
                    "date": "2026-04-19",
                    "visit_count": 768,
                    "visitor_count": 192
                },
                {
                    "date": "2026-04-20",
                    "visit_count": 1024,
                    "visitor_count": 256
                }
            ]
        }),
    );
}

#[test]
fn block_access_data_contract() {
    let data: BlockAccessData = parse_contract(json!({
        "date": "2026-04-20",
        "block_id": "blk_abc123",
        "block_name": "待办事项",
        "visit_count": 300,
        "visitor_count": 80
    }));
    assert_eq!(data.date, "2026-04-20");
    assert_eq!(data.block_id, "blk_abc123");
    assert_eq!(data.block_name, "待办事项");
    assert_eq!(data.visit_count, 300);
    assert_eq!(data.visitor_count, 80);

    assert_json_contract(
        &data,
        json!({
            "date": "2026-04-20",
            "block_id": "blk_abc123",
            "block_name": "待办事项",
            "visit_count": 300,
            "visitor_count": 80
        }),
    );
}

#[test]
fn block_access_data_search_response_contract() {
    let response: AccessDataSearchBlockResponse = parse_contract(json!({
        "items": [
            {
                "date": "2026-04-20",
                "block_id": "blk_abc123",
                "block_name": "待办事项",
                "visit_count": 300,
                "visitor_count": 80
            },
            {
                "date": "2026-04-20",
                "block_id": "blk_def456",
                "block_name": "审批中心",
                "visit_count": 150,
                "visitor_count": 45
            }
        ]
    }));
    assert_eq!(response.items.len(), 2);
    assert_eq!(response.items[0].block_name, "待办事项");
    assert_eq!(response.items[1].block_id, "blk_def456");

    assert_json_contract(
        &response,
        json!({
            "items": [
                {
                    "date": "2026-04-20",
                    "block_id": "blk_abc123",
                    "block_name": "待办事项",
                    "visit_count": 300,
                    "visitor_count": 80
                },
                {
                    "date": "2026-04-20",
                    "block_id": "blk_def456",
                    "block_name": "审批中心",
                    "visit_count": 150,
                    "visitor_count": 45
                }
            ]
        }),
    );
}
