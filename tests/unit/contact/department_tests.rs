//! contact 部门能力的可执行覆盖测试。
//!
//! 替代旧的 旧占位骨架，聚焦当前仓库中已经稳定存在、且无需 live 网络即可
//! 验证的关键路径：
//!
//! - 单部门查询请求的路径 / 查询参数 / 响应解析
//! - 子部门列表请求的分页与层级字段解析
//! - 必填参数缺失时的显式校验失败
//! - 部门模型的 contract 兼容性（含 i18n / extra 透传）

use open_lark::{
    CoreConfig, RequestOption,
    communication::contact::contact::v3::{
        department::{
            children::ListDepartmentChildrenRequest,
            get::GetDepartmentRequest,
            models::{DepartmentListResponse, DepartmentResponse},
        },
        user::models::{DepartmentIdType, UserIdType},
    },
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

fn test_config(base_url: &str) -> CoreConfig {
    CoreConfig::builder()
        .app_id("cli_test_app")
        .app_secret("cli_test_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn test_option() -> RequestOption {
    RequestOption::builder()
        .tenant_access_token("tenant_token")
        .build()
}

fn parse_contract<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).expect("contract payload should deserialize")
}

#[tokio::test]
async fn get_department_request_hits_expected_endpoint_and_parses_response() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/departments/dept_root"))
        .and(query_param("user_id_type", "open_id"))
        .and(query_param("department_id_type", "open_department_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "request_id": "req-contact-dept-get",
            "data": {
                "department": {
                    "department_id": "dept_root",
                    "open_department_id": "od_root",
                    "name": "平台研发",
                    "leader_user_id": "ou_lead",
                    "tenant_label": "core"
                }
            }
        })))
        .mount(&server)
        .await;

    let response = GetDepartmentRequest::new(test_config(&server.uri()))
        .department_id("dept_root")
        .user_id_type(UserIdType::OpenId)
        .department_id_type(DepartmentIdType::OpenDepartmentId)
        .execute_with_options(test_option())
        .await
        .expect("department request should succeed against mock server");

    assert_eq!(
        response.department.department_id.as_deref(),
        Some("dept_root")
    );
    assert_eq!(
        response.department.open_department_id.as_deref(),
        Some("od_root")
    );
    assert_eq!(response.department.name.as_deref(), Some("平台研发"));
    assert_eq!(response.department.extra["tenant_label"], "core");
}

#[tokio::test]
async fn list_department_children_request_hits_expected_endpoint_and_parses_pagination() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/departments/dept_root/children"))
        .and(query_param("user_id_type", "user_id"))
        .and(query_param("department_id_type", "department_id"))
        .and(query_param("page_size", "20"))
        .and(query_param("page_token", "token_next"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "request_id": "req-contact-dept-children",
            "data": {
                "has_more": true,
                "page_token": "next_cursor",
                "items": [
                    {
                        "department_id": "dept_child",
                        "name": "客户端平台",
                        "parent_department_id": "dept_root",
                        "level_marker": 2
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    let response = ListDepartmentChildrenRequest::new(test_config(&server.uri()))
        .department_id("dept_root")
        .user_id_type(UserIdType::UserId)
        .department_id_type(DepartmentIdType::DepartmentId)
        .page_size(20)
        .page_token("token_next")
        .execute_with_options(test_option())
        .await
        .expect("department children request should succeed against mock server");

    assert!(response.has_more);
    assert_eq!(response.page_token.as_deref(), Some("next_cursor"));
    assert_eq!(response.items.len(), 1);
    assert_eq!(
        response.items[0].department_id.as_deref(),
        Some("dept_child")
    );
    assert_eq!(
        response.items[0].parent_department_id.as_deref(),
        Some("dept_root")
    );
    assert_eq!(response.items[0].extra["level_marker"], 2);
}

#[tokio::test]
async fn get_department_request_rejects_missing_department_id() {
    let err = GetDepartmentRequest::new(test_config("https://example.invalid"))
        .execute_with_options(test_option())
        .await
        .expect_err("missing department_id should fail validation before network");

    let message = err.to_string();
    assert!(message.contains("department_id"));
}

#[test]
fn department_response_contract_preserves_i18n_and_extra_fields() {
    let response: DepartmentResponse = parse_contract(json!({
        "department": {
            "department_id": "dept_contract",
            "name": "国际化部门",
            "i18n_name": {
                "zh_cn": "国际化部门",
                "en_us": "Global Team",
                "display_code": "global-team"
            },
            "owner_scope": "cross_region"
        }
    }));

    assert_eq!(
        response.department.department_id.as_deref(),
        Some("dept_contract")
    );
    assert_eq!(
        response
            .department
            .i18n_name
            .as_ref()
            .and_then(|v| v.en_us.as_deref()),
        Some("Global Team")
    );
    assert_eq!(
        response
            .department
            .i18n_name
            .as_ref()
            .expect("i18n should exist")
            .extra["display_code"],
        "global-team"
    );
    assert_eq!(response.department.extra["owner_scope"], "cross_region");

    let list_response: DepartmentListResponse = parse_contract(json!({
        "has_more": false,
        "items": [
            {
                "department_id": "dept_contract",
                "name": "国际化部门"
            }
        ]
    }));
    assert!(!list_response.has_more);
    assert_eq!(list_response.items[0].name.as_deref(), Some("国际化部门"));
}
