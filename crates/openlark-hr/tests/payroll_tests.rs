//! payroll tests 集成测试。
#![cfg(feature = "payroll")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::payroll::payroll::v1::*;
use rstest::rstest;
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path, query_param},
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
        test_query_paygroup_request_builder,
        paygroup::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(50)
            .page_token("next_paygroup".to_string())
    );
    smoke_builder!(
        test_list_payment_activity_request_builder,
        payment_activity::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_activity".to_string())
            .paygroup_id("pg_1".to_string())
            .status(1)
    );
    smoke_builder!(
        test_archive_payment_activity_request_builder,
        payment_activity::archive::ArchiveRequest::new(
            test_config("https://open.feishu.cn"),
            "pa_1".to_string(),
        )
    );
    smoke_builder!(
        test_list_payment_activity_detail_request_builder,
        payment_activity_detail::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .activity_id("pa_1".to_string())
            .page_size(20)
            .page_token("next_detail".to_string())
    );
    smoke_builder!(
        test_query_payment_detail_request_builder,
        payment_detail::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "pa_1".to_string(),
        )
        .employee_ids(vec!["ou_1".to_string()])
        .page_size(30)
        .page_token("next_payment_detail".to_string())
    );
    smoke_builder!(
        test_list_datasource_request_builder,
        datasource::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(10)
            .page_token("next_ds".to_string())
    );
    smoke_builder!(
        test_query_datasource_record_request_builder,
        datasource_record::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .datasource_id("ds_1".to_string())
            .employee_ids(vec!["ou_1".to_string()])
            .page_size(15)
            .page_token("next_record".to_string())
    );
    smoke_builder!(
        test_save_datasource_record_request_builder,
        datasource_record::save::SaveRequest::new(
            test_config("https://open.feishu.cn"),
            "ds_1".to_string(),
            vec!["ou_1".to_string()],
            vec![datasource_record::save::DatasourceRecord {
                employee_id: "ou_1".to_string(),
                items: vec![datasource_record::save::DatasourceRecordItem {
                    field_name: "base_salary".to_string(),
                    value: json!(20000),
                }],
            }],
        )
    );
    smoke_builder!(
        test_list_cost_allocation_plan_request_builder,
        cost_allocation_plan::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_plan".to_string())
    );
    smoke_builder!(
        test_list_cost_allocation_detail_request_builder,
        cost_allocation_detail::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_cost_detail".to_string())
            .activity_id("pa_1".to_string())
            .plan_id("plan_1".to_string())
    );
    smoke_builder!(
        test_list_cost_allocation_report_request_builder,
        cost_allocation_report::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .activity_id("pa_1".to_string())
            .plan_id("plan_1".to_string())
            .page_size(20)
            .page_token("next_cost_report".to_string())
    );
    smoke_builder!(
        test_list_acct_item_request_builder,
        acct_item::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_acct_item".to_string())
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_payment_detail_activity_id_validation(#[case] activity_id: &str) {
        let result = payment_detail::query::QueryRequest::new(
            test_config("https://127.0.0.1:9"),
            activity_id.to_string(),
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("activity_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_save_datasource_record_datasource_id_validation(#[case] datasource_id: &str) {
        let result = datasource_record::save::SaveRequest::new(
            test_config("https://127.0.0.1:9"),
            datasource_id.to_string(),
            vec!["ou_1".to_string()],
            vec![datasource_record::save::DatasourceRecord {
                employee_id: "ou_1".to_string(),
                items: vec![datasource_record::save::DatasourceRecordItem {
                    field_name: "base_salary".to_string(),
                    value: json!(10000),
                }],
            }],
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("datasource_id"));
    }

    #[tokio::test]
    async fn test_save_datasource_record_requires_employee_ids() {
        let result = datasource_record::save::SaveRequest::new(
            test_config("https://127.0.0.1:9"),
            "ds_1".to_string(),
            vec![],
            vec![datasource_record::save::DatasourceRecord {
                employee_id: "ou_1".to_string(),
                items: vec![datasource_record::save::DatasourceRecordItem {
                    field_name: "base_salary".to_string(),
                    value: json!(10000),
                }],
            }],
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("employee_ids"));
    }

    #[tokio::test]
    async fn test_save_datasource_record_requires_records() {
        let result = datasource_record::save::SaveRequest::new(
            test_config("https://127.0.0.1:9"),
            "ds_1".to_string(),
            vec!["ou_1".to_string()],
            vec![],
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("records"));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_query_paygroup_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/paygroups"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "paygroup_id": "pg_1",
                        "name": "中国区薪资组",
                        "created_at": 1735689600,
                        "updated_at": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = paygroup::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].paygroup_id, "pg_1");
    }

    #[tokio::test]
    async fn test_list_payment_activity_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activitys"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_activity"))
            .and(query_param("paygroup_id", "pg_1"))
            .and(query_param("status", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "activity_id": "pa_1",
                        "paygroup_id": "pg_1",
                        "name": "2026-01 发薪",
                        "period_start_time": 1735689600,
                        "period_end_time": 1738368000,
                        "status": 1,
                        "created_at": 1735689600,
                        "updated_at": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_activity::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_activity".to_string())
            .paygroup_id("pg_1".to_string())
            .status(1)
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].activity_id, "pa_1");
    }

    #[tokio::test]
    async fn test_list_payment_activity_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activity_details"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("activity_id", "pa_1"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_detail"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "detail_id": "detail_1",
                        "activity_id": "pa_1",
                        "employee_id": "ou_1",
                        "gross_pay": 20000.0,
                        "net_pay": 18500.0,
                        "total_deduction": 1500.0,
                        "tax_amount": 1200.0
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_activity_detail::list::ListRequest::new(test_config(&mock_server.uri()))
            .activity_id("pa_1".to_string())
            .page_size(20)
            .page_token("next_detail".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].detail_id, "detail_1");
    }

    #[tokio::test]
    async fn test_archive_payment_activity_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_activitys/archive"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "success": true,
                    "activity_id": "pa_1",
                    "archived_at": 1735689600
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_activity::archive::ArchiveRequest::new(
            test_config(&mock_server.uri()),
            "pa_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .expect("请求应成功");

        assert!(resp.success);
        assert_eq!(resp.activity_id, "pa_1");
    }

    #[tokio::test]
    async fn test_query_payment_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_detail/query"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "activity_id": "pa_1",
                "employee_ids": ["ou_1"],
                "page_size": 20
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "detail_id": "detail_1",
                        "activity_id": "pa_1",
                        "employee_id": "ou_1",
                        "salary_items": [{"acct_item_id": "acct_1", "name": "基本工资", "amount": 20000.0}],
                        "deduction_items": [{"acct_item_id": "acct_2", "name": "个税", "amount": 1500.0}],
                        "net_pay": 18500.0,
                        "gross_pay": 20000.0,
                        "currency": "CNY"
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_detail::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            "pa_1".to_string(),
        )
        .employee_ids(vec!["ou_1".to_string()])
        .page_size(20)
        .execute_with_options(auth_option())
        .await
        .expect("请求应成功");

        assert_eq!(resp.items[0].employee_id, "ou_1");
        assert_eq!(resp.items[0].net_pay, 18500.0);
    }

    #[tokio::test]
    async fn test_save_datasource_record_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/save"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "datasource_id": "ds_1",
                "employee_ids": ["ou_1"],
                "records": [{
                    "employee_id": "ou_1",
                    "items": [{"field_name": "base_salary", "value": 20000}]
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "success": true,
                    "processed_count": 1,
                    "failed_count": 0
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = datasource_record::save::SaveRequest::new(
            test_config(&mock_server.uri()),
            "ds_1".to_string(),
            vec!["ou_1".to_string()],
            vec![datasource_record::save::DatasourceRecord {
                employee_id: "ou_1".to_string(),
                items: vec![datasource_record::save::DatasourceRecordItem {
                    field_name: "base_salary".to_string(),
                    value: json!(20000),
                }],
            }],
        )
        .execute_with_options(auth_option())
        .await
        .expect("请求应成功");

        assert!(resp.success);
        assert_eq!(resp.processed_count, 1);
    }

    #[tokio::test]
    async fn test_query_datasource_record_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/query"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "datasource_id": "ds_1",
                "employee_ids": ["ou_1"],
                "page_size": 20,
                "page_token": "next_record"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "record_id": "record_1",
                        "datasource_id": "ds_1",
                        "employee_id": "ou_1",
                        "items": [{"field_name": "base_salary", "value": 20000}],
                        "created_at": 1735689600,
                        "updated_at": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = datasource_record::query::QueryRequest::new(test_config(&mock_server.uri()))
            .datasource_id("ds_1".to_string())
            .employee_ids(vec!["ou_1".to_string()])
            .page_size(20)
            .page_token("next_record".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].record_id, "record_1");
    }

    #[tokio::test]
    async fn test_list_datasource_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/datasources"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_ds"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "datasource_id": "ds_1",
                        "name": "薪资系统",
                        "type_field": "api",
                        "created_at": 1735689600,
                        "updated_at": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = datasource::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_ds".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].datasource_id, "ds_1");
    }

    #[tokio::test]
    async fn test_list_cost_allocation_plan_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_plans"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_plan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "plan_id": "plan_1",
                        "name": "研发成本分摊",
                        "created_at": 1735689600,
                        "updated_at": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_plan::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_plan".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].plan_id, "plan_1");
    }

    #[tokio::test]
    async fn test_list_cost_allocation_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_details"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_cost_detail"))
            .and(query_param("activity_id", "pa_1"))
            .and(query_param("plan_id", "plan_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "detail_id": "cad_1",
                        "activity_id": "pa_1",
                        "plan_id": "plan_1",
                        "employee_id": "ou_1",
                        "allocation_ratio": 0.5,
                        "amount": 10000.0
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_detail::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_cost_detail".to_string())
            .activity_id("pa_1".to_string())
            .plan_id("plan_1".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].detail_id, "cad_1");
    }

    #[tokio::test]
    async fn test_list_cost_allocation_report_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_reports"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("plan_id", "plan_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "report_id": "report_1",
                        "activity_id": "pa_1",
                        "plan_id": "plan_1",
                        "employee_count": 120,
                        "total_amount": 500000.0,
                        "stats_time": 1735689600
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_report::list::ListRequest::new(test_config(&mock_server.uri()))
            .plan_id("plan_1".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].report_id, "report_1");
    }

    #[tokio::test]
    async fn test_list_acct_item_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/acct_items"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_acct_item"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [{
                        "acct_item_id": "acct_1",
                        "name": "基本工资",
                        "type_field": "earning"
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = acct_item::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_acct_item".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].acct_item_id, "acct_1");
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
                let raw = serde_json::to_string(&origin).expect("序列化应成功");
                let decoded: $ty = serde_json::from_str(&raw).expect("反序列化应成功");
                assert_eq!(origin, decoded);
            }
        };
    }

    roundtrip_eq!(
        test_query_paygroup_response_serialization,
        paygroup::list::ListResponse,
        paygroup::list::ListResponse {
            items: vec![paygroup::list::Paygroup {
                paygroup_id: "pg_1".to_string(),
                name: "中国区薪资组".to_string(),
                description: Some("总部薪资".to_string()),
                company_id: Some("cmp_1".to_string()),
                effective_time: Some(1735689600),
                expiration_time: None,
                created_at: 1735689600,
                updated_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_payment_activity_response_serialization,
        payment_activity::list::ListResponse,
        payment_activity::list::ListResponse {
            items: vec![payment_activity::list::PaymentActivity {
                activity_id: "pa_1".to_string(),
                paygroup_id: "pg_1".to_string(),
                name: "2026-01 发薪".to_string(),
                period_start_time: 1735689600,
                period_end_time: 1738368000,
                status: 1,
                total_amount: Some(500000.0),
                currency: Some("CNY".to_string()),
                created_at: 1735689600,
                updated_at: 1735689600,
            }],
            has_more: false,
            page_token: Some("next_activity".to_string()),
        }
    );
    roundtrip_eq!(
        test_archive_payment_activity_response_serialization,
        payment_activity::archive::ArchiveResponse,
        payment_activity::archive::ArchiveResponse {
            success: true,
            activity_id: "pa_1".to_string(),
            archived_at: 1735689600,
        }
    );
    roundtrip_eq!(
        test_list_payment_activity_detail_response_serialization,
        payment_activity_detail::list::ListResponse,
        payment_activity_detail::list::ListResponse {
            items: vec![payment_activity_detail::list::PaymentActivityDetail {
                detail_id: "detail_1".to_string(),
                activity_id: "pa_1".to_string(),
                employee_id: "ou_1".to_string(),
                employee_name: Some("张三".to_string()),
                department_id: Some("od_1".to_string()),
                gross_pay: 20000.0,
                net_pay: 18500.0,
                total_deduction: 1500.0,
                tax_amount: 1200.0,
                currency: Some("CNY".to_string()),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_query_payment_detail_response_serialization,
        payment_detail::query::QueryResponse,
        payment_detail::query::QueryResponse {
            items: vec![payment_detail::query::PaymentDetail {
                detail_id: "detail_1".to_string(),
                activity_id: "pa_1".to_string(),
                employee_id: "ou_1".to_string(),
                salary_items: vec![payment_detail::query::SalaryItem {
                    acct_item_id: "acct_1".to_string(),
                    name: "基本工资".to_string(),
                    amount: 20000.0,
                }],
                deduction_items: vec![payment_detail::query::DeductionItem {
                    acct_item_id: "acct_2".to_string(),
                    name: "个税".to_string(),
                    amount: 1500.0,
                }],
                net_pay: 18500.0,
                gross_pay: 20000.0,
                currency: "CNY".to_string(),
            }],
            has_more: false,
            page_token: Some("next_payment_detail".to_string()),
        }
    );
    roundtrip_eq!(
        test_list_datasource_response_serialization,
        datasource::list::ListResponse,
        datasource::list::ListResponse {
            items: vec![datasource::list::Datasource {
                datasource_id: "ds_1".to_string(),
                name: "薪资系统".to_string(),
                type_field: "api".to_string(),
                description: Some("外部薪资系统".to_string()),
                status: Some(1),
                created_at: 1735689600,
                updated_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_query_datasource_record_response_serialization,
        datasource_record::query::QueryResponse,
        datasource_record::query::QueryResponse {
            items: vec![datasource_record::query::DatasourceRecord {
                record_id: "record_1".to_string(),
                datasource_id: "ds_1".to_string(),
                employee_id: "ou_1".to_string(),
                items: vec![datasource_record::query::DatasourceRecordItem {
                    field_name: "base_salary".to_string(),
                    value: json!(20000),
                }],
                created_at: 1735689600,
                updated_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_save_datasource_record_response_serialization,
        datasource_record::save::SaveResponse,
        datasource_record::save::SaveResponse {
            success: true,
            processed_count: 10,
            failed_count: 1,
            failed_employee_ids: Some(vec!["ou_2".to_string()]),
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_plan_response_serialization,
        cost_allocation_plan::list::ListResponse,
        cost_allocation_plan::list::ListResponse {
            items: vec![cost_allocation_plan::list::CostAllocationPlan {
                plan_id: "plan_1".to_string(),
                name: "研发成本分摊".to_string(),
                description: Some("按部门分摊".to_string()),
                effective_time: Some(1735689600),
                expiration_time: None,
                status: Some(1),
                created_at: 1735689600,
                updated_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_detail_response_serialization,
        cost_allocation_detail::list::ListResponse,
        cost_allocation_detail::list::ListResponse {
            items: vec![cost_allocation_detail::list::CostAllocationDetail {
                detail_id: "cad_1".to_string(),
                activity_id: "pa_1".to_string(),
                plan_id: "plan_1".to_string(),
                employee_id: "ou_1".to_string(),
                cost_center_id: Some("cc_1".to_string()),
                allocation_ratio: 0.5,
                amount: 10000.0,
                currency: Some("CNY".to_string()),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_report_response_serialization,
        cost_allocation_report::list::ListResponse,
        cost_allocation_report::list::ListResponse {
            items: vec![cost_allocation_report::list::CostAllocationReport {
                report_id: "car_1".to_string(),
                activity_id: "pa_1".to_string(),
                plan_id: "plan_1".to_string(),
                cost_center_id: Some("cc_1".to_string()),
                cost_center_name: Some("研发中心".to_string()),
                employee_count: 120,
                total_amount: 500000.0,
                currency: Some("CNY".to_string()),
                stats_time: 1735689600,
            }],
            has_more: false,
            page_token: Some("next_report".to_string()),
        }
    );
    roundtrip_eq!(
        test_list_acct_item_response_serialization,
        acct_item::list::ListResponse,
        acct_item::list::ListResponse {
            items: vec![acct_item::list::AcctItem {
                acct_item_id: "acct_1".to_string(),
                name: "基本工资".to_string(),
                type_field: "earning".to_string(),
                include_in_salary: Some(true),
                include_in_social_insurance: Some(true),
                include_in_tax: Some(true),
                order: Some(1),
            }],
            has_more: false,
            page_token: None,
        }
    );
}
