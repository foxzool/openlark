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

    /// 官方 200 成功响应为文件二进制流；此处验证下载失败时飞书返回业务错误
    /// envelope（JSON）会被识别为错误，而非当成二进制内容返回。
    #[tokio::test]
    async fn test_attachment_get_business_error_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/attachments/att_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 230002,
                "msg": "attachment token invalid"
            })))
            .mount(&mock_server)
            .await;

        let err =
            attachment::get::GetRequest::new(test_config(&mock_server.uri()), "att_1".to_string())
                .execute_with_options(auth_option())
                .await
                .err()
                .unwrap()
                .to_string();
        assert!(
            err.contains("attachment token invalid") || err.contains("230002"),
            "unexpected err: {err}"
        );
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
        assert!(
            err.contains("批量获取员工花名册响应数据为空")
                || err.contains("成功响应缺少必需的 data")
                || err.contains("api_response_data"),
            "unexpected err: {err}"
        );
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_attachment_get_http_mock() {
        let mock_server = MockServer::start().await;
        let file_bytes = b"%PDF-1.4 fake-pdf-content".to_vec();
        Mock::given(method("GET"))
            .and(path("/open-apis/ehr/v1/attachments/att_1"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-disposition", "attachment; filename=\"resume.pdf\"")
                    .set_body_bytes(file_bytes.clone()),
            )
            .mount(&mock_server)
            .await;

        let resp =
            attachment::get::GetRequest::new(test_config(&mock_server.uri()), "att_1".to_string())
                .execute_with_options(auth_option())
                .await
                .unwrap();
        assert_eq!(resp.file_name, "resume.pdf");
        assert_eq!(resp.content, file_bytes);

        // 官方请求仅路径参数 token，无任何查询参数
        let received = mock_server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert!(
            received[0].url.query().unwrap_or("").is_empty(),
            "官方请求无查询参数: {:?}",
            received[0].url.query()
        );
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
                    // 官方响应示例形状：item 顶层仅 user_id + system_fields + custom_fields
                    "items":[{
                        "user_id":"ou_1",
                        "system_fields":{
                            "name":"张三",
                            "en_name":"Tony Zhang",
                            "email":"a@b.com",
                            "mobile":"(+86) 13812345678",
                            "department_id":"od-4d551617a5da3cec26666d33175dc8ca",
                            "manager":{
                                "user_id":"ou_mgr",
                                "name":"李四",
                                "en_name":"Tom Li"
                            },
                            "job":{"id":1695838220091399i64,"name":"测试工程师"},
                            "job_level":{"id":1695838220091399i64,"name":"P6"},
                            "work_location":{"id":1695838220091399i64,"name":"武汉"},
                            "employee_no":"E001",
                            "status":2,
                            "hire_date":"2020-01-01"
                        },
                        "custom_fields":[
                            {"key":"field_xxxxxxxx","label":"自定义字段 1","type":"date","value":"2021-01-13"}
                        ]
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
        let profile = &resp.items[0];
        let system_fields = profile.system_fields.as_ref().unwrap();
        assert_eq!(system_fields.name.as_deref(), Some("张三"));
        assert_eq!(
            system_fields
                .manager
                .as_ref()
                .and_then(|m| m.user_id.as_deref()),
            Some("ou_mgr")
        );
        assert_eq!(
            system_fields.job.as_ref().and_then(|j| j.name.as_deref()),
            Some("测试工程师")
        );
        assert_eq!(system_fields.hire_date.as_deref(), Some("2020-01-01"));
        assert_eq!(
            profile.custom_fields.as_ref().unwrap()[0].label.as_deref(),
            Some("自定义字段 1")
        );
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
            file_name: "resume.pdf".to_string(),
            content: b"%PDF-1.4 fake-pdf-content".to_vec(),
        }
    );

    roundtrip_eq!(
        test_employee_list_response_serialization,
        employee::list::ListResponse,
        employee::list::ListResponse {
            items: vec![employee::list::EmployeeProfile {
                user_id: Some("ou_1".to_string()),
                system_fields: Some(employee::list::SystemFields {
                    name: Some("张三".to_string()),
                    en_name: Some("Tony Zhang".to_string()),
                    email: Some("a@b.com".to_string()),
                    mobile: None,
                    department_id: Some("od_1".to_string()),
                    manager: Some(employee::list::ManagerRef {
                        user_id: Some("ou_mgr".to_string()),
                        name: Some("李四".to_string()),
                        en_name: Some("Tom Li".to_string()),
                    }),
                    job: Some(employee::list::IdNameObject {
                        id: Some(1695838220091399),
                        name: Some("测试工程师".to_string()),
                    }),
                    job_level: None,
                    work_location: None,
                    gender: Some(1),
                    birthday: None,
                    native_region: None,
                    ethnicity: None,
                    marital_status: None,
                    political_status: None,
                    entered_workforce_date: None,
                    id_type: Some(1),
                    id_number: None,
                    hukou_type: None,
                    hukou_location: None,
                    bank_account_number: None,
                    bank_name: None,
                    social_security_account: None,
                    provident_fund_account: None,
                    employee_no: Some("E001".to_string()),
                    employee_type: Some(1),
                    status: Some(2),
                    hire_date: Some("2020-01-01".to_string()),
                    probation_months: Some(2.0),
                    conversion_date: None,
                    application: None,
                    application_status: None,
                    last_day: None,
                    departure_type: None,
                    departure_reason: None,
                    departure_notes: None,
                    contract_company: None,
                    contract_type: None,
                    contract_start_date: None,
                    contract_expiration_date: None,
                    contract_sign_times: None,
                    personal_email: None,
                    family_address: None,
                    primary_emergency_contact: None,
                    emergency_contact: Some(vec![employee::list::EmergencyContact {
                        name: Some("张三".to_string()),
                        relationship: Some(1),
                        mobile: Some("(+86) 13812345678".to_string()),
                    }]),
                    highest_level_of_edu: None,
                    education: None,
                    former_work_exp: None,
                    work_exp: None,
                    id_photo_po_side: None,
                    id_photo_em_side: None,
                    id_photo: None,
                    diploma_photo: None,
                    graduation_cert: None,
                    cert_of_merit: None,
                    offboarding_file: None,
                    cancel_onboarding_reason: None,
                    cancel_onboarding_notes: None,
                    employee_form_status: None,
                    create_time: Some(1608690517811),
                    update_time: Some(1608690517811),
                }),
                custom_fields: Some(vec![employee::list::CustomField {
                    key: Some("field_xxxxxxxx".to_string()),
                    label: Some("自定义字段 1".to_string()),
                    r#type: Some("date".to_string()),
                    value: Some("2021-01-13".to_string()),
                }]),
            }],
            has_more: false,
            page_token: Some("next_2".to_string()),
        }
    );
}
