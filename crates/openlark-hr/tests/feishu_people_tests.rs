//! feishu people tests 集成测试。
#![cfg(feature = "corehr")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::feishu_people::corehr::v1::{
    company, contract, country_region, department, employee, job_family, job_level, location,
};
use openlark_hr::feishu_people::corehr::v2::{employee as employee_v2, position};
use rstest::rstest;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
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
        test_builder_employee_v1_create,
        employee::CreateRequest::new(test_config("https://open.feishu.cn"))
            .name("张三".to_string())
    );
    smoke_builder!(
        test_builder_employee_v1_delete,
        employee::DeleteRequest::new(test_config("https://open.feishu.cn"))
            .employee_id("e_1".to_string())
    );
    smoke_builder!(
        test_builder_employee_v1_list,
        employee::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );
    smoke_builder!(
        test_builder_employee_v1_search,
        employee::SearchRequest::new(test_config("https://open.feishu.cn")).query("张".to_string())
    );
    smoke_builder!(
        test_builder_employee_v1_batch_get,
        employee::BatchGetRequest::new(test_config("https://open.feishu.cn"))
            .employee_ids(vec!["e_1".to_string()])
    );
    smoke_builder!(
        test_builder_employee_v2_create,
        employee_v2::create::CreateRequestBuilder::new(test_config("https://open.feishu.cn"))
            .name("李四".to_string())
    );
    smoke_builder!(
        test_builder_employee_v2_search,
        employee_v2::search::SearchRequestBuilder::new(test_config("https://open.feishu.cn"))
            .name("李".to_string())
    );
    smoke_builder!(
        test_builder_employee_v2_batch_get,
        employee_v2::batch_get::BatchGetRequestBuilder::new(
            test_config("https://open.feishu.cn"),
            vec!["e_2".to_string()]
        )
    );

    smoke_builder!(
        test_builder_department_create,
        department::CreateRequest::new(test_config("https://open.feishu.cn"))
            .name("研发部".to_string())
    );
    smoke_builder!(
        test_builder_department_get,
        department::GetRequest::new(test_config("https://open.feishu.cn"))
            .department_id("d_1".to_string())
    );
    smoke_builder!(
        test_builder_department_patch,
        department::PatchRequest::new(test_config("https://open.feishu.cn"))
            .department_id("d_1".to_string())
            .name("研发一部".to_string())
    );
    smoke_builder!(
        test_builder_department_delete,
        department::DeleteRequest::new(test_config("https://open.feishu.cn"))
            .department_id("d_1".to_string())
    );
    smoke_builder!(
        test_builder_department_list,
        department::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );
    smoke_builder!(
        test_builder_department_batch_get,
        department::BatchGetRequest::new(test_config("https://open.feishu.cn"))
            .department_ids(vec!["d_1".to_string()])
    );
    smoke_builder!(
        test_builder_department_search,
        department::SearchRequest::new(test_config("https://open.feishu.cn"))
            .query("研发".to_string())
    );
    smoke_builder!(
        test_builder_department_parents,
        department::ParentsRequest::new(test_config("https://open.feishu.cn"))
            .department_id("d_1".to_string())
    );

    smoke_builder!(
        test_builder_position_create,
        position::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_position_query,
        position::query::QueryRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_position_patch,
        position::patch::PatchRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_position_delete,
        position::del_position::DelPositionRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_position_active,
        position::active::ActiveRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_position_recent_change,
        position::query_recent_change::QueryRecentChangeRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );

    smoke_builder!(
        test_builder_contract_create,
        contract::CreateRequest::new(test_config("https://open.feishu.cn"))
            .employee_id("e_1".to_string())
    );
    smoke_builder!(
        test_builder_contract_get,
        contract::GetRequest::new(test_config("https://open.feishu.cn"))
            .contract_id("c_1".to_string())
    );
    smoke_builder!(
        test_builder_contract_patch,
        contract::PatchRequest::new(test_config("https://open.feishu.cn"))
            .contract_id("c_1".to_string())
    );
    smoke_builder!(
        test_builder_contract_delete,
        contract::DeleteRequest::new(test_config("https://open.feishu.cn"))
            .contract_id("c_1".to_string())
    );
    smoke_builder!(
        test_builder_contract_list,
        contract::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );

    smoke_builder!(
        test_builder_location_create,
        location::CreateRequest::new(test_config("https://open.feishu.cn"))
            .name("上海办公室".to_string())
    );
    smoke_builder!(
        test_builder_location_get,
        location::GetRequest::new(test_config("https://open.feishu.cn"))
            .location_id("l_1".to_string())
    );
    smoke_builder!(
        test_builder_location_patch,
        location::PatchRequest::new(test_config("https://open.feishu.cn"))
            .location_id("l_1".to_string())
    );
    smoke_builder!(
        test_builder_location_delete,
        location::DeleteRequest::new(test_config("https://open.feishu.cn"))
            .location_id("l_1".to_string())
    );
    smoke_builder!(
        test_builder_location_list,
        location::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );

    smoke_builder!(
        test_builder_company_get,
        company::GetRequest::new(test_config("https://open.feishu.cn"))
            .company_id("co_1".to_string())
    );
    smoke_builder!(
        test_builder_company_list,
        company::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );
    smoke_builder!(
        test_builder_company_batch_get,
        company::BatchGetRequest::new(test_config("https://open.feishu.cn"))
            .company_ids(vec!["co_1".to_string()])
    );
    smoke_builder!(
        test_builder_company_create,
        company::CreateRequest::new(test_config("https://open.feishu.cn"))
            .name("OpenLark".to_string())
    );
    smoke_builder!(
        test_builder_job_family_list,
        job_family::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );
    smoke_builder!(
        test_builder_job_level_list,
        job_level::ListRequest::new(test_config("https://open.feishu.cn")).page_size(20)
    );
    smoke_builder!(
        test_builder_country_region_get,
        country_region::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_builder_country_region_list,
        country_region::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_validation_employee_name_required(#[case] name: &str) {
        let result = employee::CreateRequest::new(test_config("https://127.0.0.1:9"))
            .name(name.to_string())
            .execute_with_options(auth_option())
            .await;
        assert!(result.is_err());
    }

    #[rstest]
    #[case("")]
    #[case("\t")]
    #[tokio::test]
    async fn test_validation_department_name_required(#[case] name: &str) {
        let result = department::CreateRequest::new(test_config("https://127.0.0.1:9"))
            .name(name.to_string())
            .execute_with_options(auth_option())
            .await;
        assert!(result.is_err());
    }

    #[rstest]
    #[case("")]
    #[case("\n")]
    #[tokio::test]
    async fn test_validation_contract_employee_required(#[case] employee_id: &str) {
        let result = contract::CreateRequest::new(test_config("https://127.0.0.1:9"))
            .employee_id(employee_id.to_string())
            .execute_with_options(auth_option())
            .await;
        assert!(result.is_err());
    }

    #[rstest]
    #[case("")]
    #[case("    ")]
    #[tokio::test]
    async fn test_validation_location_name_required(#[case] name: &str) {
        let result = location::CreateRequest::new(test_config("https://127.0.0.1:9"))
            .name(name.to_string())
            .execute_with_options(auth_option())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validation_company_batch_get_requires_ids() {
        let result = company::BatchGetRequest::new(test_config("https://127.0.0.1:9"))
            .execute_with_options(auth_option())
            .await;
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_http_employee_create() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/employees"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"name":"张三"})))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"employee_id":"e_1"}})),
            )
            .mount(&server)
            .await;

        let resp = employee::CreateRequest::new(test_config(&server.uri()))
            .name("张三".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.employee_id, "e_1");
    }

    #[tokio::test]
    async fn test_http_employee_delete() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/employees/e_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = employee::DeleteRequest::new(test_config(&server.uri()))
            .employee_id("e_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_employee_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/employees"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"employee_id":"e_1","name":"张三"}],"has_more":false}
            })))
            .mount(&server)
            .await;

        let resp = employee::ListRequest::new(test_config(&server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items.len(), 1);
    }

    #[tokio::test]
    async fn test_http_employee_search() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/employees/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"employee_id":"e_1","name":"张三"}],"has_more":false}
            })))
            .mount(&server)
            .await;

        let resp = employee::SearchRequest::new(test_config(&server.uri()))
            .query("张".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].employee_id, "e_1");
    }

    #[tokio::test]
    async fn test_http_employee_batch_get() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/employees/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"employee_id":"e_1","name":"张三"}]}
            })))
            .mount(&server)
            .await;

        let resp = employee::BatchGetRequest::new(test_config(&server.uri()))
            .employee_ids(vec!["e_1".to_string()])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items.len(), 1);
    }

    #[tokio::test]
    async fn test_http_department_create() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/departments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"department_id":"d_1"}})),
            )
            .mount(&server)
            .await;

        let resp = department::CreateRequest::new(test_config(&server.uri()))
            .name("研发部".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.department_id, "d_1");
    }

    #[tokio::test]
    async fn test_http_department_get() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/departments/d_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"department":{"department_id":"d_1","name":"研发部"}}
            })))
            .mount(&server)
            .await;

        let resp = department::GetRequest::new(test_config(&server.uri()))
            .department_id("d_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.department.department_id, "d_1");
    }

    #[tokio::test]
    async fn test_http_department_patch() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/departments/d_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = department::PatchRequest::new(test_config(&server.uri()))
            .department_id("d_1".to_string())
            .name("研发一部".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_department_delete() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/departments/d_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = department::DeleteRequest::new(test_config(&server.uri()))
            .department_id("d_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_department_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/departments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"department_id":"d_1","name":"研发部"}],"has_more":false}
            })))
            .mount(&server)
            .await;

        let resp = department::ListRequest::new(test_config(&server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items.len(), 1);
    }

    #[tokio::test]
    async fn test_http_contract_create() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/contracts"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"contract_id":"c_1"}})),
            )
            .mount(&server)
            .await;

        let resp = contract::CreateRequest::new(test_config(&server.uri()))
            .employee_id("e_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.contract_id, "c_1");
    }

    #[tokio::test]
    async fn test_http_contract_get() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/contracts/c_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"contract":{"contract_id":"c_1"}}
            })))
            .mount(&server)
            .await;

        let resp = contract::GetRequest::new(test_config(&server.uri()))
            .contract_id("c_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.contract.contract_id, "c_1");
    }

    #[tokio::test]
    async fn test_http_contract_patch() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/contracts/c_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = contract::PatchRequest::new(test_config(&server.uri()))
            .contract_id("c_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_contract_delete() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/contracts/c_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = contract::DeleteRequest::new(test_config(&server.uri()))
            .contract_id("c_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_contract_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/contracts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"contract_id":"c_1"}],"has_more":false}
            })))
            .mount(&server)
            .await;

        let resp = contract::ListRequest::new(test_config(&server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items.len(), 1);
    }

    #[tokio::test]
    async fn test_http_location_create() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/locations"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"location_id":"l_1"}})),
            )
            .mount(&server)
            .await;

        let resp = location::CreateRequest::new(test_config(&server.uri()))
            .name("上海办公室".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.location_id, "l_1");
    }

    #[tokio::test]
    async fn test_http_location_get() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/locations/l_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"location":{"location_id":"l_1"}}
            })))
            .mount(&server)
            .await;

        let resp = location::GetRequest::new(test_config(&server.uri()))
            .location_id("l_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.location.location_id, "l_1");
    }

    #[tokio::test]
    async fn test_http_location_patch() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v1/locations/l_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = location::PatchRequest::new(test_config(&server.uri()))
            .location_id("l_1".to_string())
            .name("上海总部".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_location_delete() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/locations/l_1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"result":true}})),
            )
            .mount(&server)
            .await;

        let resp = location::DeleteRequest::new(test_config(&server.uri()))
            .location_id("l_1".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_http_location_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/locations"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"location_list":[{"location_id":"l_1"}],"has_more":false}
            })))
            .mount(&server)
            .await;

        let resp = location::ListRequest::new(test_config(&server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.location_list.len(), 1);
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

    fn sample_employee() -> employee::Employee {
        employee::Employee {
            employee_id: "e_1".to_string(),
            name: "张三".to_string(),
            email: None,
            phone: None,
            department_id: None,
            position_id: None,
            employee_no: None,
            status: None,
            hire_date: None,
            termination_date: None,
            created_time: None,
            updated_time: None,
        }
    }

    fn sample_department() -> department::Department {
        department::Department {
            department_id: "d_1".to_string(),
            name: "研发部".to_string(),
            leader_user_ids: None,
            parent_department_id: None,
            status: None,
            code: None,
            description: None,
            created_time: None,
            updated_time: None,
        }
    }

    fn sample_contract() -> contract::Contract {
        contract::Contract {
            contract_id: "c_1".to_string(),
            contract_number: None,
            employee_id: None,
            start_date: None,
            end_date: None,
            contract_type: None,
            status: None,
            signing_date: None,
            probation_start_date: None,
            probation_end_date: None,
            probation_duration: None,
            files: None,
            custom_fields: None,
            created_time: None,
            updated_time: None,
        }
    }

    fn sample_location() -> location::Location {
        location::Location {
            location_id: "l_1".to_string(),
            name: Some("上海办公室".to_string()),
            location_type: None,
            address: None,
            city: None,
            country: None,
            status: None,
            created_time: None,
            updated_time: None,
        }
    }

    fn sample_company() -> company::Company {
        company::Company {
            company_id: "co_1".to_string(),
            name: "OpenLark".to_string(),
            status: None,
            code: None,
            description: None,
            created_time: None,
            updated_time: None,
        }
    }

    roundtrip_eq!(
        test_ser_employee_v1_create,
        employee::CreateResponse,
        employee::CreateResponse {
            employee_id: "e_1".to_string()
        }
    );
    roundtrip_eq!(
        test_ser_employee_v1_delete,
        employee::DeleteResponse,
        employee::DeleteResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_employee_v1_list,
        employee::ListResponse,
        employee::ListResponse {
            items: vec![employee::EmployeeRoster {
                employee_id: "e_1".to_string(),
                name: "张三".to_string(),
                email: None,
                phone: None,
                department_name: None,
                position_name: None,
                employee_no: None,
                status: None,
                hire_date: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_ser_employee_v1_search,
        employee::SearchResponse,
        employee::SearchResponse {
            items: vec![sample_employee()],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_ser_employee_v1_batch_get,
        employee::BatchGetResponse,
        employee::BatchGetResponse {
            items: vec![sample_employee()]
        }
    );
    roundtrip_eq!(
        test_ser_employee_v2_create,
        employee_v2::create::CreateResponse,
        employee_v2::create::CreateResponse {
            data: Some(employee_v2::create::CreateResponseData {
                id: "e_2".to_string(),
                name: Some("李四".to_string()),
                email: None,
            })
        }
    );
    roundtrip_eq!(
        test_ser_employee_v2_search,
        employee_v2::search::SearchResponse,
        employee_v2::search::SearchResponse {
            data: Some(employee_v2::search::SearchResponseData {
                page_token: None,
                has_more: Some(false),
                items: Some(vec![employee_v2::search::EmployeeItem {
                    id: "e_2".to_string(),
                    name: Some("李四".to_string()),
                    email: None,
                    phone: None,
                    department_id: None,
                    position_id: None,
                    employee_no: None,
                    hire_date: None,
                    status: None,
                }]),
            })
        }
    );
    roundtrip_eq!(
        test_ser_employee_v2_batch_get,
        employee_v2::batch_get::BatchGetResponse,
        employee_v2::batch_get::BatchGetResponse {
            data: Some(employee_v2::batch_get::BatchGetResponseData {
                items: Some(vec![employee_v2::batch_get::EmployeeItem {
                    id: "e_2".to_string(),
                    name: Some("李四".to_string()),
                    email: None,
                    phone: None,
                    department_id: None,
                    position_id: None,
                    employee_no: None,
                    hire_date: None,
                    status: None,
                }])
            })
        }
    );

    roundtrip_eq!(
        test_ser_department_create,
        department::CreateResponse,
        department::CreateResponse {
            department_id: "d_1".to_string()
        }
    );
    roundtrip_eq!(
        test_ser_department_get,
        department::GetResponse,
        department::GetResponse {
            department: sample_department()
        }
    );
    roundtrip_eq!(
        test_ser_department_patch,
        department::PatchResponse,
        department::PatchResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_department_delete,
        department::DeleteResponse,
        department::DeleteResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_department_list,
        department::ListResponse,
        department::ListResponse {
            items: vec![sample_department()],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_ser_department_batch_get,
        department::BatchGetResponse,
        department::BatchGetResponse {
            items: vec![sample_department()]
        }
    );
    roundtrip_eq!(
        test_ser_department_search,
        department::SearchResponse,
        department::SearchResponse {
            items: vec![sample_department()],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_ser_department_parents,
        department::ParentsResponse,
        department::ParentsResponse {
            items: vec![sample_department()]
        }
    );

    roundtrip_eq!(
        test_ser_position_create,
        position::create::CreateResponse,
        position::create::CreateResponse {
            data: json!({"id":"p_1"})
        }
    );
    roundtrip_eq!(
        test_ser_position_query,
        position::query::QueryResponse,
        position::query::QueryResponse {
            data: json!({"items":[]})
        }
    );
    roundtrip_eq!(
        test_ser_position_patch,
        position::patch::PatchResponse,
        position::patch::PatchResponse {
            data: json!({"result":true})
        }
    );
    roundtrip_eq!(
        test_ser_position_delete,
        position::del_position::DelPositionResponse,
        position::del_position::DelPositionResponse {
            data: json!({"result":true})
        }
    );
    roundtrip_eq!(
        test_ser_position_active,
        position::active::ActiveResponse,
        position::active::ActiveResponse {
            data: json!({"result":true})
        }
    );
    roundtrip_eq!(
        test_ser_position_recent_change,
        position::query_recent_change::QueryRecentChangeResponse,
        position::query_recent_change::QueryRecentChangeResponse {
            data: json!({"items":[]})
        }
    );

    roundtrip_eq!(
        test_ser_contract_create,
        contract::CreateResponse,
        contract::CreateResponse {
            contract_id: "c_1".to_string()
        }
    );
    roundtrip_eq!(
        test_ser_contract_get,
        contract::GetResponse,
        contract::GetResponse {
            contract: sample_contract()
        }
    );
    roundtrip_eq!(
        test_ser_contract_patch,
        contract::PatchResponse,
        contract::PatchResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_contract_delete,
        contract::DeleteResponse,
        contract::DeleteResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_contract_list,
        contract::ListResponse,
        contract::ListResponse {
            items: vec![sample_contract()],
            has_more: false,
            page_token: None,
        }
    );

    roundtrip_eq!(
        test_ser_location_create,
        location::CreateResponse,
        location::CreateResponse {
            location_id: "l_1".to_string()
        }
    );
    roundtrip_eq!(
        test_ser_location_get,
        location::GetResponse,
        location::GetResponse {
            location: sample_location()
        }
    );
    roundtrip_eq!(
        test_ser_location_patch,
        location::PatchResponse,
        location::PatchResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_location_delete,
        location::DeleteResponse,
        location::DeleteResponse { result: true }
    );
    roundtrip_eq!(
        test_ser_location_list,
        location::ListResponse,
        location::ListResponse {
            location_list: vec![sample_location()],
            page_token: None,
            has_more: Some(false),
        }
    );

    roundtrip_eq!(
        test_ser_company_get,
        company::GetResponse,
        company::GetResponse {
            company: sample_company()
        }
    );
    roundtrip_eq!(
        test_ser_company_list,
        company::ListResponse,
        company::ListResponse {
            items: vec![sample_company()],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_ser_company_batch_get,
        company::BatchGetResponse,
        company::BatchGetResponse {
            items: vec![sample_company()]
        }
    );
    roundtrip_eq!(
        test_ser_company_create,
        company::CreateResponse,
        company::CreateResponse {
            company_id: "co_1".to_string()
        }
    );
    roundtrip_eq!(
        test_ser_job_family_list,
        job_family::ListResponse,
        job_family::ListResponse {
            job_family_list: vec![job_family::JobFamily {
                job_family_id: "jf_1".to_string(),
                name: Some("技术序列".to_string()),
                description: None,
                status: None,
                code: None,
                created_time: None,
                updated_time: None,
            }],
            page_token: None,
            has_more: Some(false),
        }
    );
    roundtrip_eq!(
        test_ser_job_level_list,
        job_level::ListResponse,
        job_level::ListResponse {
            job_level_list: vec![job_level::JobLevel {
                job_level_id: "jl_1".to_string(),
                name: Some("P6".to_string()),
                level: Some(6),
                description: None,
                status: None,
                job_family_id: None,
                created_time: None,
                updated_time: None,
            }],
            page_token: None,
            has_more: Some(false),
        }
    );
    roundtrip_eq!(
        test_ser_country_region_get,
        country_region::get::GetResponse,
        country_region::get::GetResponse {
            country_region: Some(json!({"country_region_id":"cn"}))
        }
    );
    roundtrip_eq!(
        test_ser_country_region_list,
        country_region::list::ListResponse,
        country_region::list::ListResponse {
            items: Some(vec![country_region::list::CountryRegionItem {
                extra: std::collections::HashMap::from([(
                    "country_region_id".to_string(),
                    json!("cn"),
                )]),
            }]),
            page_token: None,
            has_more: None,
        }
    );
}
