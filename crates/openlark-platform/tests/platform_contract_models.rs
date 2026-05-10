#![cfg(all(
    feature = "admin",
    feature = "v1",
    feature = "directory",
    feature = "app-engine",
    feature = "mdm",
    feature = "tenant",
    feature = "trust_party",
    feature = "spark"
))]
//! 各业务域 request/response 数据模型的代表性契约测试。

use openlark_platform::common::{AppInfo, DirectoryItem, SystemSettings};
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

fn assert_roundtrip<T>(payload: Value)
where
    T: DeserializeOwned + Serialize,
{
    let parsed: T = parse_contract(payload);
    let serialized = to_value(&parsed).unwrap();
    let reparsed: T = parse_contract(serialized.clone());
    assert_eq!(to_value(&reparsed).unwrap(), serialized);
}

// ── common ──────────────────────────────────────────────────────────────

#[test]
fn common_models_roundtrip() {
    let app = AppInfo {
        app_id: "cli_test".to_string(),
        app_name: "测试应用".to_string(),
        description: Some("应用描述".to_string()),
        status: "active".to_string(),
    };
    assert_json_contract(
        &app,
        json!({
            "app_id": "cli_test",
            "app_name": "测试应用",
            "description": "应用描述",
            "status": "active"
        }),
    );
    let roundtrip: AppInfo = parse_contract(to_value(&app).unwrap());
    assert_eq!(roundtrip.app_id, app.app_id);

    let dir_item = DirectoryItem {
        id: "dir_001".to_string(),
        name: "技术部".to_string(),
        item_type: "department".to_string(),
    };
    assert_json_contract(
        &dir_item,
        json!({
            "id": "dir_001",
            "name": "技术部",
            "item_type": "department"
        }),
    );

    let settings = SystemSettings {
        key: "max_members".to_string(),
        value: "500".to_string(),
        description: Some("最大成员数".to_string()),
    };
    let rt: SystemSettings = parse_contract(json!({
        "key": "max_members",
        "value": "500",
        "description": "最大成员数"
    }));
    assert_eq!(rt.key, settings.key);
    assert_eq!(rt.description, settings.description);
}

// ── admin: badge / badge_image ──────────────────────────────────────────

#[test]
fn admin_badge_contracts() {
    use openlark_platform::admin::admin::v1::badge::get::GetBadgeResponse;
    use openlark_platform::admin::admin::v1::badge::list::ListBadgeResponse;
    use openlark_platform::admin::admin::v1::badge_image::create::CreateBadgeImageResponse;

    let badge: GetBadgeResponse = parse_contract(json!({
        "badge_id": "b_001",
        "name": "优秀员工",
        "description": "年度优秀员工勋章",
        "icon_url": "https://cdn.example/badge.png",
        "create_time": "2026-01-15T10:00:00Z"
    }));
    assert_eq!(badge.badge_id, "b_001");
    assert_eq!(badge.name, "优秀员工");
    assert_json_contract(
        &badge,
        json!({
            "badge_id": "b_001",
            "name": "优秀员工",
            "description": "年度优秀员工勋章",
            "icon_url": "https://cdn.example/badge.png",
            "create_time": "2026-01-15T10:00:00Z"
        }),
    );

    let list_resp: ListBadgeResponse = parse_contract(json!({
        "items": [
            {
                "badge_id": "b_001",
                "name": "优秀员工",
                "description": null,
                "icon_url": "https://cdn.example/badge.png",
                "create_time": "2026-01-15T10:00:00Z"
            }
        ],
        "page_token": "next_page_token",
        "has_more": true
    }));
    assert_eq!(list_resp.items.len(), 1);
    assert!(list_resp.has_more);
    let rt: ListBadgeResponse = parse_contract(to_value(&list_resp).unwrap());
    assert_eq!(rt.items[0].badge_id, "b_001");

    let img: CreateBadgeImageResponse = parse_contract(json!({
        "image_id": "img_001",
        "image_url": "https://cdn.example/img.png"
    }));
    assert_eq!(img.image_id, "img_001");
}

#[test]
fn admin_badge_grant_contracts() {
    use openlark_platform::admin::admin::v1::badge::grant::list::ListBadgeGrantResponse;

    let resp: ListBadgeGrantResponse = parse_contract(json!({
        "items": [
            {
                "grant_id": "g_001",
                "badge_id": "b_001",
                "user_id": "ou_user001",
                "create_time": "2026-03-01T08:00:00Z"
            }
        ],
        "page_token": null,
        "has_more": false
    }));
    assert_eq!(resp.items.len(), 1);
    assert_eq!(resp.items[0].grant_id, "g_001");
    assert!(!resp.has_more);
}

// ── directory: department / employee (私有字段，仅测试 roundtrip) ──────

#[test]
fn directory_department_contracts() {
    use openlark_platform::directory::directory::v1::department::search::DepartmentSearchResponse;

    let payload = json!({
        "items": [
            { "department_id": "od_001", "name": "技术部", "parent_id": "od_root" },
            { "department_id": "od_002", "name": "产品部", "parent_id": null }
        ],
        "has_more": false,
        "page": 1,
        "page_size": 20
    });
    assert_roundtrip::<DepartmentSearchResponse>(payload);
}

#[test]
fn directory_employee_contracts() {
    use openlark_platform::directory::directory::v1::employee::search::EmployeeSearchResponse;

    let payload = json!({
        "items": [
            {
                "employee_id": "emp_001",
                "name": "张三",
                "mobile": "13800138000",
                "email": "zhangsan@example.com",
                "department_ids": ["od_001", "od_002"]
            }
        ],
        "has_more": false,
        "page": 1,
        "page_size": 10
    });
    assert_roundtrip::<EmployeeSearchResponse>(payload);
}

// ── app_engine: seat / approval ─────────────────────────────────────────

#[test]
fn app_engine_seat_contracts() {
    use openlark_platform::app_engine::apaas::v1::seat_assignment::list::SeatAssignmentListResponse;

    let payload = json!({
        "items": [
            {
                "seat_id": "seat_001",
                "user_id": "ou_user001",
                "seat_type": "standard",
                "assigned_time": 1710000000,
                "expire_time": 1735689600
            }
        ],
        "has_more": false,
        "page": 1,
        "page_size": 20
    });
    assert_roundtrip::<SeatAssignmentListResponse>(payload);
}

#[test]
fn app_engine_approval_contracts() {
    use openlark_platform::app_engine::apaas::v1::approval_instance::list::ListInstanceResponse;

    let resp: ListInstanceResponse = parse_contract(json!({
        "items": [
            {
                "instance_id": "inst_001",
                "status": "PENDING",
                "initiator_id": "ou_001",
                "create_time": "2026-04-01T08:00:00Z"
            }
        ],
        "page_token": "next",
        "has_more": false
    }));
    assert_eq!(resp.items.len(), 1);
    assert_eq!(resp.items[0].instance_id, "inst_001");
}

// ── tenant ──────────────────────────────────────────────────────────────

#[test]
fn tenant_query_contracts() {
    use openlark_platform::tenant::v2::tenant::query::{I18nName, TenantQueryResponse};

    let i18n: I18nName = parse_contract(json!({
        "zh_cn": "飞书科技",
        "en_us": "Feishu Tech",
        "ja_jp": null
    }));
    assert_eq!(i18n.zh_cn.as_deref(), Some("飞书科技"));

    let tenant: TenantQueryResponse = parse_contract(json!({
        "name": "飞书科技",
        "tenant_code": "tenant_001",
        "icon": "https://cdn.example/icon.png",
        "i18n_name": {
            "zh_cn": "飞书科技",
            "en_us": "Feishu Tech",
            "ja_jp": null
        }
    }));
    assert_eq!(tenant.name, "飞书科技");
    assert_eq!(tenant.tenant_code.as_deref(), Some("tenant_001"));
    assert_json_contract(
        &tenant,
        json!({
            "name": "飞书科技",
            "tenant_code": "tenant_001",
            "icon": "https://cdn.example/icon.png",
            "i18n_name": {
                "zh_cn": "飞书科技",
                "en_us": "Feishu Tech",
                "ja_jp": null
            }
        }),
    );
}

#[test]
fn tenant_product_assign_contracts() {
    use openlark_platform::tenant::v2::tenant::product_assign_info::query::AssignInfoListQueryResponse;

    let resp: AssignInfoListQueryResponse = parse_contract(json!({
        "items": [
            {
                "product_id": "prod_001",
                "product_name": "飞书文档",
                "product_i18n_name": {
                    "zh_cn": "飞书文档",
                    "en_us": "Feishu Docs",
                    "ja_jp": null
                },
                "total_count": 100,
                "assigned_count": 80,
                "unassigned_count": 20,
                "expire_time": "2026-12-31T23:59:59Z"
            }
        ],
        "page_token": null,
        "has_more": false
    }));
    assert_eq!(resp.items.len(), 1);
    assert_eq!(resp.items[0].product_id, "prod_001");
    assert_eq!(resp.items[0].total_count, 100);
}

// ── trust_party ─────────────────────────────────────────────────────────

#[test]
fn trust_party_collaboration_contracts() {
    use openlark_platform::trust_party::v1::collaboration_tenant::list::CollaborationTenantListResponse;

    let resp: CollaborationTenantListResponse = parse_contract(json!({
        "items": [
            {
                "tenant_key": "tk_001",
                "tenant_name": "合作企业A",
                "status": "ACTIVE"
            }
        ],
        "page_token": "next_page",
        "has_more": true
    }));
    assert_eq!(resp.items.len(), 1);
    assert_eq!(resp.has_more, Some(true));
}

#[test]
fn trust_party_visible_org_contracts() {
    use openlark_platform::trust_party::v1::collaboration_tenant::visible_organization::VisibleOrganizationResponse;

    let resp: VisibleOrganizationResponse = parse_contract(json!({
        "departments": [
            { "department_id": "od_001", "name": "技术部" }
        ],
        "users": [
            { "user_id": "ou_001", "name": "李四" }
        ]
    }));
    assert_eq!(resp.departments.len(), 1);
    assert_eq!(resp.users[0].name, "李四");
}

// ── mdm ─────────────────────────────────────────────────────────────────

#[test]
fn mdm_country_region_contracts() {
    use openlark_platform::mdm::v3::country_region::list::CountryRegionListResponse;

    let payload = json!({
        "items": [
            {
                "mdm_code": "CN",
                "name": "中国",
                "i18n_name": {
                    "zh_cn": "中国",
                    "en_us": "China",
                    "ja_jp": "中国"
                },
                "phone_code": "+86"
            }
        ],
        "page_token": null,
        "has_more": false
    });
    assert_roundtrip::<CountryRegionListResponse>(payload);
}

// ── spark ───────────────────────────────────────────────────────────────

#[test]
fn spark_id_convert_contracts() {
    use openlark_platform::spark::spark::v1::directory::user::id_convert::DirectoryUserIdConvertResponse;

    let resp: DirectoryUserIdConvertResponse = parse_contract(json!({
        "items": [
            { "source_id": "spark_user_001", "target_id": "ou_001" },
            { "source_id": "spark_user_002", "target_id": "ou_002" }
        ]
    }));
    assert_eq!(resp.items.len(), 2);
    let rt: DirectoryUserIdConvertResponse = parse_contract(to_value(&resp).unwrap());
    assert_eq!(rt.items[0].source_id, "spark_user_001");
}
