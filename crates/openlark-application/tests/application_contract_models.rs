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
            "user_id": "ou_d317f090b7258ad0372aa53963cda70d",
            "version": "1664360599355",
            "extra": "{}",
            "pc": { "web_app": 1, "gadget": 2 },
            "mobile": { "web_app": 1, "gadget": 2 }
        }));
        assert_eq!(body.user_id, "ou_d317f090b7258ad0372aa53963cda70d");
        assert_eq!(body.version, "1664360599355");
        assert_eq!(body.extra.as_deref(), Some("{}"));
        assert_eq!(body.pc.as_ref().unwrap().web_app, Some(1));
        assert_eq!(body.mobile.as_ref().unwrap().gadget, Some(2));
    }
}

#[cfg(feature = "workplace")]
mod workplace_tests {
    use super::{assert_json_contract, parse_contract};
    use openlark_application::workplace::workplace::v1::custom_workplace_access_data::search::{
        AccessDataSearchCustomResponse, CustomWorkplaceAccessData,
    };
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
            "all_workplace": { "pv": 1024, "uv": 256 },
            "default_workplace": { "pv": 512, "uv": 128 }
        }));
        assert_eq!(data.date, "2026-04-20");
        assert_eq!(data.all_workplace.pv, 1024);
        assert_eq!(data.all_workplace.uv, 256);
        assert_eq!(data.default_workplace.pv, 512);
        assert_eq!(data.default_workplace.uv, 128);

        assert_json_contract(
            &data,
            json!({
                "date": "2026-04-20",
                "all_workplace": { "pv": 1024, "uv": 256 },
                "default_workplace": { "pv": 512, "uv": 128 }
            }),
        );
    }

    #[test]
    fn workplace_access_data_search_response_contract() {
        let response: AccessDataSearchWorkplaceResponse = parse_contract(json!({
            "items": [
                {
                    "date": "2026-04-18",
                    "all_workplace": { "pv": 512, "uv": 128 },
                    "default_workplace": { "pv": 400, "uv": 100 }
                },
                {
                    "date": "2026-04-19",
                    "all_workplace": { "pv": 768, "uv": 192 },
                    "default_workplace": { "pv": 600, "uv": 150 }
                },
                {
                    "date": "2026-04-20",
                    "all_workplace": { "pv": 1024, "uv": 256 },
                    "default_workplace": { "pv": 800, "uv": 200 }
                }
            ],
            "has_more": true,
            "page_token": "ddowkdkl9w2d"
        }));
        assert_eq!(response.items.len(), 3);
        assert_eq!(response.items[0].date, "2026-04-18");
        assert_eq!(response.items[2].all_workplace.pv, 1024);
        assert!(response.has_more);
        assert_eq!(response.page_token.as_deref(), Some("ddowkdkl9w2d"));
    }

    #[test]
    fn custom_workplace_access_data_contract() {
        let data: CustomWorkplaceAccessData = parse_contract(json!({
            "custom_workplace_id": "tpl_645b003aaa40001435b2ddw",
            "access_data": { "pv": 100, "uv": 30 },
            "date": "2026-04-20",
            "custom_workplace_name": [ { "language": "zh", "name": "名字" } ]
        }));
        assert_eq!(data.custom_workplace_id, "tpl_645b003aaa40001435b2ddw");
        assert_eq!(data.access_data.pv, 100);
        assert_eq!(data.access_data.uv, 30);
        assert_eq!(data.date, "2026-04-20");
        let names = data.custom_workplace_name.as_ref().unwrap();
        assert_eq!(names[0].language, "zh");
        assert_eq!(names[0].name, "名字");

        assert_json_contract(
            &data,
            json!({
                "custom_workplace_id": "tpl_645b003aaa40001435b2ddw",
                "access_data": { "pv": 100, "uv": 30 },
                "date": "2026-04-20",
                "custom_workplace_name": [ { "language": "zh", "name": "名字" } ]
            }),
        );
    }

    #[test]
    fn custom_workplace_access_data_search_response_contract() {
        let response: AccessDataSearchCustomResponse = parse_contract(json!({
            "items": [
                {
                    "custom_workplace_id": "tpl_645b003aaa40001435b2ddw",
                    "access_data": { "pv": 100, "uv": 30 },
                    "date": "2026-04-20"
                }
            ],
            "has_more": false
        }));
        assert_eq!(response.items.len(), 1);
        assert_eq!(
            response.items[0].custom_workplace_id,
            "tpl_645b003aaa40001435b2ddw"
        );
        assert_eq!(response.items[0].access_data.uv, 30);
        assert!(!response.has_more);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn block_access_data_contract() {
        let data: BlockAccessData = parse_contract(json!({
            "date": "2026-04-20",
            "block_id": "283438293839422334",
            "access_data": { "pv": 300, "uv": 80 }
        }));
        assert_eq!(data.date, "2026-04-20");
        assert_eq!(data.block_id, "283438293839422334");
        assert_eq!(data.access_data.pv, 300);
        assert_eq!(data.access_data.uv, 80);

        assert_json_contract(
            &data,
            json!({
                "date": "2026-04-20",
                "block_id": "283438293839422334",
                "access_data": { "pv": 300, "uv": 80 }
            }),
        );
    }

    #[test]
    fn block_access_data_search_response_contract() {
        let response: AccessDataSearchBlockResponse = parse_contract(json!({
            "items": [
                {
                    "date": "2026-04-20",
                    "block_id": "283438293839422334",
                    "access_data": { "pv": 300, "uv": 80 }
                },
                {
                    "date": "2026-04-20",
                    "block_id": "283438293839422335",
                    "access_data": { "pv": 150, "uv": 45 }
                }
            ],
            "has_more": true,
            "page_token": "ddowkdkl9w2d"
        }));
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].access_data.pv, 300);
        assert_eq!(response.items[1].block_id, "283438293839422335");
        assert!(response.has_more);
        assert_eq!(response.page_token.as_deref(), Some("ddowkdkl9w2d"));
    }
}
