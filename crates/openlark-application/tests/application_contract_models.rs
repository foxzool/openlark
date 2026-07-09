//! Representative contract tests for Application request/response models.
//!
//! v1 与 workplace 测试各自经独立 feature 门控（#312：版本独立门控，不再搭车）。

#![cfg(any(feature = "v1", feature = "workplace"))]

#[cfg(feature = "workplace")]
use serde::Serialize;
use serde::de::DeserializeOwned;
#[cfg(feature = "workplace")]
use serde_json::to_value;
use serde_json::{Value, from_value};

#[cfg(feature = "workplace")]
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

#[cfg(feature = "v1")]
mod v1_tests {
    use super::parse_contract;
    use openlark_application::application::application::v1::app_badge::set::SetAppBadgeBody;
    use serde_json::json;

    #[test]
    fn set_app_badge_body_contract() {
        let body: SetAppBadgeBody = parse_contract(json!({
            "app_id": "cli_a5xxxxxxxx",
            "badge": 3
        }));
        assert_eq!(body.app_id, "cli_a5xxxxxxxx");
        assert_eq!(body.badge, 3);
    }
}

#[cfg(feature = "workplace")]
mod workplace_tests {
    use super::{assert_json_contract, parse_contract};
    use openlark_application::workplace::workplace::v1::workplace_access_data::search::{
        AccessDataSearchWorkplaceResponse, WorkplaceAccessData,
    };
    use openlark_application::workplace::workplace::v1::workplace_block_access_data::search::{
        AccessDataSearchBlockResponse, BlockAccessData,
    };
    use serde_json::json;

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
                { "date": "2026-04-18", "visit_count": 512, "visitor_count": 128 },
                { "date": "2026-04-19", "visit_count": 768, "visitor_count": 192 },
                { "date": "2026-04-20", "visit_count": 1024, "visitor_count": 256 }
            ]
        }));
        assert_eq!(response.items.len(), 3);
        assert_eq!(response.items[0].date, "2026-04-18");
        assert_eq!(response.items[2].visit_count, 1024);
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
    }
}
