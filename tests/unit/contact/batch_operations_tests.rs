//! contact 批量操作能力的可执行覆盖测试。
//!
//! 替代旧的批量操作 旧占位骨架，聚焦当前可稳定验证的关键路径：
//!
//! - 批量部门查询的 endpoint / query / 响应解析
//! - 批量用户查询的 endpoint / query / 嵌套结构解析
//! - 批量请求在缺少 ID 集合时的显式校验失败

use open_lark::{
    CoreConfig, RequestOption,
    communication::contact::contact::v3::{
        department::batch::BatchGetDepartmentsRequest,
        user::{
            batch::{BatchGetUsersRequest, BatchGetUsersResponse},
            models::{DepartmentIdType, UserIdType},
        },
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
async fn batch_get_departments_request_hits_batch_endpoint_and_parses_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/departments/batch"))
        .and(query_param("department_ids", "dept_batch_1"))
        .and(query_param("department_id_type", "open_department_id"))
        .and(query_param("user_id_type", "union_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "request_id": "req-contact-dept-batch",
            "data": {
                "items": [
                    {
                        "department_id": "dept_batch_1",
                        "open_department_id": "od_batch_1",
                        "name": "基础平台",
                        "batch_label": "seed"
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    let response = BatchGetDepartmentsRequest::new(test_config(&server.uri()))
        .push_department_id("dept_batch_1")
        .department_id_type(DepartmentIdType::OpenDepartmentId)
        .user_id_type(UserIdType::UnionId)
        .execute_with_options(test_option())
        .await
        .expect("batch department request should succeed against mock server");

    assert_eq!(response.items.len(), 1);
    assert_eq!(
        response.items[0].department_id.as_deref(),
        Some("dept_batch_1")
    );
    assert_eq!(
        response.items[0].open_department_id.as_deref(),
        Some("od_batch_1")
    );
    assert_eq!(response.items[0].extra["batch_label"], "seed");
}

#[tokio::test]
async fn batch_get_users_request_hits_batch_endpoint_and_parses_nested_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/contact/v3/users/batch"))
        .and(query_param("user_ids", "ou_user_1"))
        .and(query_param("user_id_type", "open_id"))
        .and(query_param("department_id_type", "open_department_id"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "request_id": "req-contact-user-batch",
            "data": {
                "items": [
                    {
                        "open_id": "ou_user_1",
                        "user_id": "user_1",
                        "name": "张三",
                        "avatar": {
                            "avatar_72": "https://img.example/avatar72.png",
                            "avatar_tag": "primary"
                        },
                        "employment_type": "full_time"
                    }
                ]
            }
        })))
        .mount(&server)
        .await;

    let response = BatchGetUsersRequest::new(test_config(&server.uri()))
        .push_user_id("ou_user_1")
        .user_id_type(UserIdType::OpenId)
        .department_id_type(DepartmentIdType::OpenDepartmentId)
        .execute_with_options(test_option())
        .await
        .expect("batch user request should succeed against mock server");

    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].open_id.as_deref(), Some("ou_user_1"));
    assert_eq!(response.items[0].name.as_deref(), Some("张三"));
    assert_eq!(
        response.items[0]
            .avatar
            .as_ref()
            .and_then(|avatar| avatar.avatar_72.as_deref()),
        Some("https://img.example/avatar72.png")
    );
    assert_eq!(
        response.items[0]
            .avatar
            .as_ref()
            .expect("avatar should exist")
            .extra["avatar_tag"],
        "primary"
    );
    assert_eq!(response.items[0].extra["employment_type"], "full_time");
}

#[tokio::test]
async fn batch_requests_reject_empty_id_sets_before_network() {
    let dept_err = BatchGetDepartmentsRequest::new(test_config("https://example.invalid"))
        .execute_with_options(test_option())
        .await
        .expect_err("empty department_ids should fail validation");
    assert!(dept_err.to_string().contains("department_ids"));

    let user_err = BatchGetUsersRequest::new(test_config("https://example.invalid"))
        .execute_with_options(test_option())
        .await
        .expect_err("empty user_ids should fail validation");
    assert!(user_err.to_string().contains("user_ids"));
}

#[test]
fn batch_user_response_contract_preserves_nested_avatar_and_extra_fields() {
    let response: BatchGetUsersResponse = parse_contract(json!({
        "items": [
            {
                "open_id": "ou_contract",
                "user_id": "user_contract",
                "name": "李四",
                "avatar": {
                    "avatar_240": "https://img.example/avatar240.png",
                    "avatar_scene": "directory"
                },
                "tenant_role": "maintainer"
            }
        ]
    }));

    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].user_id.as_deref(), Some("user_contract"));
    assert_eq!(
        response.items[0]
            .avatar
            .as_ref()
            .and_then(|avatar| avatar.avatar_240.as_deref()),
        Some("https://img.example/avatar240.png")
    );
    assert_eq!(
        response.items[0]
            .avatar
            .as_ref()
            .expect("avatar should exist")
            .extra["avatar_scene"],
        "directory"
    );
    assert_eq!(response.items[0].extra["tenant_role"], "maintainer");
}
