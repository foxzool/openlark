//! ehr tests 集成测试。
#![cfg(feature = "ehr")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::ehr::ehr::v1::{attachment, employee};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

fn test_config(base_url: &str) -> Config {
    Config::builder()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn auth_option() -> RequestOption {
    RequestOption::builder()
        .user_access_token("test_token")
        .build()
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    macro_rules! smoke_builder {
        ($name:ident, $expr:expr) => {
            #[test]
            fn $name() {
                let _request = $expr;
            }
        };
    }

    smoke_builder!(
        test_attachment_get_builder,
        attachment::get::GetRequest::new(
            test_config("https://open.feishu.cn"),
            "att_1".to_string(),
            "ou_1".to_string(),
        )
    );
    smoke_builder!(
        test_employee_list_builder,
        employee::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_1".to_string())
            .user_ids(vec!["ou_1".to_string(), "ou_2".to_string()])
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_attachment_get_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/attachments/att_1"))
            .and(query_param("user_id", "ou_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = attachment::get::GetRequest::new(
            test_config(&mock_server.uri()),
            "att_1".to_string(),
            "ou_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .err()
        .unwrap()
        .to_string();
        assert!(err.contains("下载人员附件响应数据为空"));
    }

    #[tokio::test]
    async fn test_employee_list_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/employees"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = employee::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(err.contains("批量获取员工花名册响应数据为空"));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_attachment_get_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/attachments/att_1"))
            .and(query_param("user_id", "ou_1"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{
                    "name":"resume.pdf",
                    "file_type":"application/pdf",
                    "size":1024,
                    "download_url":"https://example.com/download/att_1",
                    "token":"download_token"
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = attachment::get::GetRequest::new(
            test_config(&mock_server.uri()),
            "att_1".to_string(),
            "ou_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.token, "download_token");
    }

    #[tokio::test]
    async fn test_employee_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/employees"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_1"))
            .and(query_param("user_ids", "ou_1,ou_2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{
                    "items":[{
                        "user_id":"ou_1",
                        "employee_no":"E001",
                        "name":"张三",
                        "avatar":"https://example.com/avatar.jpg",
                        "department_ids":["d_1"],
                        "position_id":"p_1",
                        "position_name":"工程师",
                        "employment_status":2,
                        "hire_date":1735689600,
                        "termination_date":null,
                        "working_hours_type_id":"w_1",
                        "cost_center_id":"c_1",
                        "direct_manager_id":"ou_mgr",
                        "custom_fields":{"level":"P6"}
                    }],
                    "has_more":false,
                    "page_token":"next_2"
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = employee::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_1".to_string())
            .user_ids(vec!["ou_1".to_string(), "ou_2".to_string()])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].position_name, Some("工程师".to_string()));
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    macro_rules! roundtrip_eq {
        ($name:ident, $ty:ty, $value:expr) => {
            #[test]
            fn $name() {
                let origin: $ty = $value;
                let raw = serde_json::to_string(&origin).unwrap();
                let decoded: $ty = serde_json::from_str(&raw).expect("JSON 反序列化失败");
                assert_eq!(origin, decoded);
            }
        };
    }

    roundtrip_eq!(
        test_attachment_get_response_serialization,
        attachment::get::GetResponse,
        attachment::get::GetResponse {
            name: "resume.pdf".to_string(),
            file_type: "application/pdf".to_string(),
            size: 1024,
            download_url: "https://example.com/download/att_1".to_string(),
            token: "download_token".to_string(),
        }
    );

    roundtrip_eq!(
        test_employee_list_response_serialization,
        employee::list::ListResponse,
        employee::list::ListResponse {
            items: vec![employee::list::EmployeeProfile {
                user_id: "ou_1".to_string(),
                employee_no: Some("E001".to_string()),
                name: Some("张三".to_string()),
                avatar: Some("https://example.com/avatar.jpg".to_string()),
                department_ids: Some(vec!["d_1".to_string()]),
                position_id: Some("p_1".to_string()),
                position_name: Some("工程师".to_string()),
                employment_status: Some(2),
                hire_date: Some(1735689600),
                termination_date: None,
                working_hours_type_id: Some("w_1".to_string()),
                cost_center_id: Some("c_1".to_string()),
                direct_manager_id: Some("ou_mgr".to_string()),
                custom_fields: Some(json!({"level":"P6"})),
            }],
            has_more: false,
            page_token: Some("next_2".to_string()),
        }
    );
}
