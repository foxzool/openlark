//! payroll tests 集成测试。
#![cfg(feature = "payroll")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::payroll::payroll::v1::*;
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
            .pay_period_start_date("2024-01-01".to_string())
            .pay_period_end_date("2024-01-31".to_string())
            .page_size(20)
            .page_token("next_activity".to_string())
            .statuses(vec![100, 200])
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
            .page_index(1)
            .page_size(20)
            .activity_id("pa_1".to_string())
            .include_segment_data(true)
    );
    smoke_builder!(
        test_query_payment_detail_request_builder,
        payment_detail::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .page_index(1)
            .page_size(30)
            .employee_ids(vec!["ou_1".to_string()])
            .activity_ids(vec!["pa_1".to_string()])
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
            .source_code("test__c".to_string())
            .selected_fields(vec!["base_salary__c".to_string()])
            .field_filters(vec![
                datasource_record::query::DatasourceRecordFieldFilter {
                    field_code: "employment_id".to_string(),
                    field_values: Some(vec!["ou_1".to_string()]),
                    operator: Some(1),
                },
            ])
            .page_size(15)
            .page_token("next_record".to_string())
    );
    smoke_builder!(
        test_save_datasource_record_request_builder,
        datasource_record::save::SaveRequest::new(
            test_config("https://open.feishu.cn"),
            "test__c".to_string(),
            vec![datasource_record::save::DatasourceRecord {
                active_status: 1,
                field_values: vec![datasource_record::save::DatasourceRecordFieldValue {
                    field_code: "base_salary__c".to_string(),
                    value: "20000".to_string(),
                    field_type: None,
                }],
            }],
        )
    );
    smoke_builder!(
        test_list_cost_allocation_plan_request_builder,
        cost_allocation_plan::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .pay_period("2023-11".to_string())
            .page_size(20)
            .page_token("next_plan".to_string())
    );
    smoke_builder!(
        test_list_cost_allocation_detail_request_builder,
        cost_allocation_detail::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .cost_allocation_plan_id("72131231231231231".to_string())
            .pay_period("2024-06".to_string())
            .report_type(0)
            .page_size(20)
            .page_token("next_cost_detail".to_string())
    );
    smoke_builder!(
        test_list_cost_allocation_report_request_builder,
        cost_allocation_report::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .cost_allocation_plan_id("6823630319749580304".to_string())
            .pay_period("2023-11".to_string())
            .report_type(1)
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

    #[tokio::test]
    async fn test_payment_detail_requires_employee_ids() {
        // 未设置 employee_ids（官方必填）
        let result = payment_detail::query::QueryRequest::new(test_config("https://127.0.0.1:9"))
            .page_index(1)
            .page_size(20)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("employee_ids"));
    }

    #[tokio::test]
    async fn test_payment_detail_requires_non_empty_employee_ids() {
        let result = payment_detail::query::QueryRequest::new(test_config("https://127.0.0.1:9"))
            .page_index(1)
            .page_size(20)
            .employee_ids(vec![])
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.expect_err("应返回错误").to_string();
        assert!(err.contains("employee_ids"));
    }

    #[tokio::test]
    async fn test_save_datasource_record_source_code_validation() {
        for source_code in ["", "   "] {
            let result = datasource_record::save::SaveRequest::new(
                test_config("https://127.0.0.1:9"),
                source_code.to_string(),
                vec![datasource_record::save::DatasourceRecord {
                    active_status: 1,
                    field_values: vec![datasource_record::save::DatasourceRecordFieldValue {
                        field_code: "base_salary__c".to_string(),
                        value: "10000".to_string(),
                        field_type: None,
                    }],
                }],
            )
            .execute_with_options(auth_option())
            .await;

            assert!(result.is_err());
            let err = result.expect_err("应返回错误").to_string();
            assert!(err.contains("source_code"));
        }
    }

    #[tokio::test]
    async fn test_save_datasource_record_requires_records() {
        let result = datasource_record::save::SaveRequest::new(
            test_config("https://127.0.0.1:9"),
            "test__c".to_string(),
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
                        "pay_group_id": "pg_1",
                        "name": {"zh_cn": "中国区薪资组", "en_us": "China pay group"},
                        "code": "PAY_GROUP-3-8",
                        "status": 1
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

        assert_eq!(resp.items[0].pay_group_id, "pg_1");
        assert_eq!(resp.items[0].name.zh_cn, Some("中国区薪资组".to_string()));
        assert_eq!(resp.items[0].code, "PAY_GROUP-3-8");
        assert_eq!(resp.items[0].status, 1);
    }

    #[tokio::test]
    async fn test_list_payment_activity_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activitys"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("pay_period_start_date", "2024-01-01"))
            .and(query_param("pay_period_end_date", "2024-01-31"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_activity"))
            .and(query_param("statuses", "100,200"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "payment_activitys": [{
                        "activity_id": "pa_1",
                        "activity_names": [{"locale": "zh_cn", "value": "2026-01 发薪"}],
                        "pay_date": "2026-01-31",
                        "total_number_of_payroll": 100,
                        "number_of_calculation_activities": 1,
                        "calculation_activities": [{
                            "calculation_activity_id": "ca_1",
                            "calculation_activity_names": [{"locale": "zh_cn", "value": "算薪活动"}]
                        }],
                        "activity_status": 100
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_activity::list::ListRequest::new(test_config(&mock_server.uri()))
            .pay_period_start_date("2024-01-01".to_string())
            .pay_period_end_date("2024-01-31".to_string())
            .page_size(20)
            .page_token("next_activity".to_string())
            .statuses(vec![100, 200])
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.payment_activitys[0].activity_id, "pa_1");
        assert_eq!(resp.payment_activitys[0].activity_status, 100);
    }

    #[tokio::test]
    async fn test_list_payment_activity_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/payment_activity_details"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_index", "1"))
            .and(query_param("page_size", "20"))
            .and(query_param("activity_id", "pa_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "payment_activity_details": [{
                        "employee_id": "ou_1",
                        "payment_details": [{
                            "id": "acct_1",
                            "accounting_item_names": [{"locale": "zh_cn", "value": "基本工资"}],
                            "accounting_item_value": {
                                "original_value": "20000",
                                "reference_values": []
                            },
                            "segment_values": [],
                            "accounting_item_type": 2
                        }]
                    }],
                    "total": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_activity_detail::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_index(1)
            .page_size(20)
            .activity_id("pa_1".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.total, 1);
        assert_eq!(resp.payment_activity_details[0].employee_id, "ou_1");
        assert_eq!(
            resp.payment_activity_details[0].payment_details[0].accounting_item_names[0].value,
            "基本工资"
        );
    }

    #[tokio::test]
    async fn test_archive_payment_activity_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_activitys/archive"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"activity_id": "pa_1"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&mock_server)
            .await;

        payment_activity::archive::ArchiveRequest::new(
            test_config(&mock_server.uri()),
            "pa_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .expect("请求应成功");
    }

    #[tokio::test]
    async fn test_query_payment_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/payment_detail/query"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "page_index": 1,
                "page_size": 20,
                "employee_ids": ["ou_1"],
                "activity_ids": ["pa_1"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "payment_details": [{
                        "employee_id": "ou_1",
                        "activity_id": "pa_1",
                        "payment_accounting_items": [{
                            "id": "acct_1",
                            "accounting_item_names": [{"locale": "zh_cn", "value": "基本工资"}],
                            "accounting_item_value": {
                                "original_value": "20000",
                                "reference_values": []
                            },
                            "segment_values": [],
                            "accounting_item_type": 2
                        }]
                    }],
                    "total": 1
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = payment_detail::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_index(1)
            .page_size(20)
            .employee_ids(vec!["ou_1".to_string()])
            .activity_ids(vec!["pa_1".to_string()])
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.total, 1);
        assert_eq!(resp.payment_details[0].employee_id, "ou_1");
        assert_eq!(resp.payment_details[0].activity_id, "pa_1");
    }

    #[tokio::test]
    async fn test_save_datasource_record_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/save"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "source_code": "test__c",
                "records": [{
                    "active_status": 1,
                    "field_values": [{"field_code": "base_salary__c", "value": "20000"}]
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {"affect_counts": "1"}
            })))
            .mount(&mock_server)
            .await;

        let resp = datasource_record::save::SaveRequest::new(
            test_config(&mock_server.uri()),
            "test__c".to_string(),
            vec![datasource_record::save::DatasourceRecord {
                active_status: 1,
                field_values: vec![datasource_record::save::DatasourceRecordFieldValue {
                    field_code: "base_salary__c".to_string(),
                    value: "20000".to_string(),
                    field_type: None,
                }],
            }],
        )
        .execute_with_options(auth_option())
        .await
        .expect("请求应成功");

        assert_eq!(resp.affect_counts, "1");
    }

    #[tokio::test]
    async fn test_query_datasource_record_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/payroll/v1/datasource_records/query"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_record"))
            .and(body_json(json!({
                "source_code": "test__c",
                "field_filters": [{
                    "field_code": "employment_id",
                    "field_values": ["ou_1"],
                    "operator": 1
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "page_token": "next_record_2",
                    "has_more": false,
                    "records": [{
                        "active_status": 1,
                        "field_values": [{"field_code": "base_salary__c", "value": "20000"}]
                    }]
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = datasource_record::query::QueryRequest::new(test_config(&mock_server.uri()))
            .source_code("test__c".to_string())
            .field_filters(vec![
                datasource_record::query::DatasourceRecordFieldFilter {
                    field_code: "employment_id".to_string(),
                    field_values: Some(vec!["ou_1".to_string()]),
                    operator: Some(1),
                },
            ])
            .page_size(20)
            .page_token("next_record".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert!(!resp.has_more);
        assert_eq!(resp.records[0].active_status, 1);
        assert_eq!(resp.records[0].field_values[0].field_code, "base_salary__c");
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
                    // 官方响应示例形状：datasources[]（无 items 键）
                    "page_token": "next_ds2",
                    "has_more": false,
                    "datasources": [{
                        "code": "test_datasource__c",
                        "i18n_names": [{"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}],
                        "active_status": 1,
                        "fields": [{
                            "code": "test__c",
                            "i18n_names": [{"locale": "zh_cn", "value": "名称"}],
                            "field_type": 1,
                            "active_status": 1,
                            "i18n_description": [{"locale": "zh_cn", "value": "描述"}],
                            "decimal_places": 1
                        }],
                        "i18n_description": [{"locale": "zh_cn", "value": "外部薪资系统"}],
                        "data_period_type": 1
                    }]
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

        assert!(!resp.has_more);
        let ds = &resp.datasources.as_ref().unwrap()[0];
        assert_eq!(ds.code.as_deref(), Some("test_datasource__c"));
        assert_eq!(ds.active_status, Some(1));
        assert_eq!(ds.fields.as_ref().unwrap()[0].field_type, Some(1));
    }

    #[tokio::test]
    async fn test_list_cost_allocation_plan_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_plans"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("pay_period", "2023-11"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_plan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    // 官方响应示例形状：items[]{id, names, applicable_country_region, dimensions, cost_items}
                    "items": [{
                        "id": "6823630319749593096",
                        "names": [{"locale": "zh_cn", "value": "研发成本分摊"}],
                        "applicable_country_region": "6823630319749593389",
                        "dimensions": [{
                            "i18n_names": [{"locale": "zh_cn", "value": "公司主体", "id": "723123123123123213"}],
                            "api_name": "company",
                            "obj_api_name": "jobData"
                        }],
                        "cost_items": [{
                            "id": "7433424967234601004",
                            "name": [{"locale": "zh_cn", "value": "基本工资"}],
                            "enable_correct": true
                        }]
                    }],
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_plan::list::ListRequest::new(test_config(&mock_server.uri()))
            .pay_period("2023-11".to_string())
            .page_size(20)
            .page_token("next_plan".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.items[0].id.as_deref(), Some("6823630319749593096"));
        assert_eq!(
            resp.items[0].names.as_ref().unwrap()[0].value.as_deref(),
            Some("研发成本分摊")
        );
        assert_eq!(
            resp.items[0].dimensions.as_ref().unwrap()[0]
                .api_name
                .as_deref(),
            Some("company")
        );
        assert_eq!(
            resp.items[0].cost_items.as_ref().unwrap()[0].enable_correct,
            Some(true)
        );
    }

    #[tokio::test]
    async fn test_list_cost_allocation_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_details"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_cost_detail"))
            .and(query_param("cost_allocation_plan_id", "72131231231231231"))
            .and(query_param("pay_period", "2024-06"))
            .and(query_param("report_type", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    // 官方响应示例形状：cost_allocation_report_datas[]（无 items 键）
                    "cost_allocation_report_datas": [{
                        "data_summary_dimensions": [{
                            "dimension_level": 1,
                            "dimension_type": 1,
                            "dimension_value_id": "6823630319749580306",
                            "dimension_names": [{"locale": "zh_cn", "value": "名称"}],
                            "dimension_titles": [{"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}]
                        }],
                        "compensation_cost_item": {
                            "compensation_costs": [{
                                "compensation_cost_value": "123456.78",
                                "i18n_names": [{"locale": "zh_cn", "value": "名称"}]
                            }]
                        },
                        "employment_id": "72321836123692186"
                    }],
                    "cost_allocation_report_names": [{"locale": "zh_cn", "value": "名称"}],
                    "pay_period": "2024-06",
                    "has_more": false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_detail::list::ListRequest::new(test_config(&mock_server.uri()))
            .cost_allocation_plan_id("72131231231231231".to_string())
            .pay_period("2024-06".to_string())
            .report_type(0)
            .page_size(20)
            .page_token("next_cost_detail".to_string())
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.pay_period, "2024-06");
        let detail = &resp.cost_allocation_report_datas.as_ref().unwrap()[0];
        assert_eq!(detail.employment_id.as_deref(), Some("72321836123692186"));
        assert_eq!(
            detail.data_summary_dimensions.as_ref().unwrap()[0]
                .dimension_value_id
                .as_deref(),
            Some("6823630319749580306")
        );
        assert_eq!(
            detail
                .compensation_cost_item
                .as_ref()
                .unwrap()
                .compensation_costs
                .as_ref()
                .unwrap()[0]
                .compensation_cost_value
                .as_deref(),
            Some("123456.78")
        );
    }

    #[tokio::test]
    async fn test_list_cost_allocation_report_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/cost_allocation_reports"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("cost_allocation_plan_id", "6823630319749580304"))
            .and(query_param("pay_period", "2023-11"))
            .and(query_param("report_type", "1"))
            .and(query_param("page_size", "50"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    // 官方响应示例形状：pay_period + cost_allocation_report_names + cost_allocation_report_datas[]
                    "pay_period": "2023-11",
                    "has_more": false,
                    "cost_allocation_report_names": [{"locale": "zh_cn", "value": "名称"}],
                    "cost_allocation_report_datas": [{
                        "data_summary_dimensions": [{
                            "dimension_level": 1,
                            "dimension_type": 1,
                            "dimension_value_id": "6823630319749580306",
                            "enum_dimension": {
                                "enum_value_id": "7188920315914207276",
                                "enum_key": "company"
                            },
                            "dimension_value_lookup_info": {
                                "type": "work_calendar",
                                "id": "6961286846093788621",
                                "code": "D1230011115"
                            },
                            "dimension_names": [{"locale": "zh_cn", "value": "名称"}],
                            "dimension_titles": [{"locale": "zh_cn", "value": "名称", "id": "723123123123123213"}]
                        }],
                        "compensation_cost_item": {
                            "number_of_individuals_for_payment": 100,
                            "compensation_costs": [{
                                "compensation_cost_value": "123456.78",
                                "i18n_names": [{"locale": "zh_cn", "value": "名称"}]
                            }]
                        }
                    }]
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = cost_allocation_report::list::ListRequest::new(test_config(&mock_server.uri()))
            .cost_allocation_plan_id("6823630319749580304".to_string())
            .pay_period("2023-11".to_string())
            .report_type(1)
            .page_size(50)
            .execute_with_options(auth_option())
            .await
            .expect("请求应成功");

        assert_eq!(resp.pay_period, "2023-11");
        assert!(!resp.has_more);
        let report_data = &resp.cost_allocation_report_datas.as_ref().unwrap()[0];
        let dimension = &report_data.data_summary_dimensions.as_ref().unwrap()[0];
        assert_eq!(dimension.dimension_level, Some(1));
        assert_eq!(
            dimension
                .enum_dimension
                .as_ref()
                .and_then(|e| e.enum_key.as_deref()),
            Some("company")
        );
        assert_eq!(
            dimension
                .dimension_value_lookup_info
                .as_ref()
                .and_then(|l| l.r#type.as_deref()),
            Some("work_calendar")
        );
        assert_eq!(
            report_data
                .compensation_cost_item
                .as_ref()
                .unwrap()
                .number_of_individuals_for_payment,
            Some(100)
        );
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
                        "id": "acct_1",
                        "i18n_names": [{"locale": "zh_cn", "value": "基本工资"}],
                        "category_id": "cat_1",
                        "data_type": 2,
                        "decimal_places": 2,
                        "active_status": 1
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

        assert_eq!(resp.items[0].id, "acct_1");
        assert_eq!(resp.items[0].i18n_names[0].value, "基本工资");
        assert_eq!(resp.items[0].data_type, 2);
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
                pay_group_id: "pg_1".to_string(),
                name: paygroup::list::PaygroupName {
                    zh_cn: Some("中国区薪资组".to_string()),
                    en_us: Some("China pay group".to_string()),
                },
                code: "PAY_GROUP-3-8".to_string(),
                status: 1,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_payment_activity_response_serialization,
        payment_activity::list::ListResponse,
        payment_activity::list::ListResponse {
            payment_activitys: vec![payment_activity::list::PaymentActivity {
                activity_id: "pa_1".to_string(),
                activity_names: vec![payment_activity::list::I18nContent {
                    locale: "zh_cn".to_string(),
                    value: "2026-01 发薪".to_string(),
                }],
                pay_date: "2026-01-31".to_string(),
                total_number_of_payroll: 100,
                number_of_calculation_activities: 1,
                calculation_activities: vec![payment_activity::list::CalculationActivity {
                    calculation_activity_id: "ca_1".to_string(),
                    calculation_activity_names: vec![payment_activity::list::I18nContent {
                        locale: "zh_cn".to_string(),
                        value: "算薪活动".to_string(),
                    }],
                }],
                activity_status: 100,
            }],
            has_more: false,
            page_token: Some("next_activity".to_string()),
        }
    );
    roundtrip_eq!(
        test_archive_payment_activity_response_serialization,
        payment_activity::archive::ArchiveResponse,
        payment_activity::archive::ArchiveResponse {}
    );
    roundtrip_eq!(
        test_list_payment_activity_detail_response_serialization,
        payment_activity_detail::list::ListResponse,
        payment_activity_detail::list::ListResponse {
            payment_activity_details: vec![payment_activity_detail::list::PaymentActivityDetail {
                employee_id: "ou_1".to_string(),
                payment_details: vec![payment_activity_detail::list::PaymentAccountingItem {
                    id: "acct_1".to_string(),
                    accounting_item_names: vec![payment_activity_detail::list::I18nContent {
                        locale: "zh_cn".to_string(),
                        value: "基本工资".to_string(),
                    }],
                    accounting_item_value: payment_activity_detail::list::AccountingItemValue {
                        original_value: Some("20000".to_string()),
                        reference_values: vec![],
                    },
                    segment_values: vec![],
                    accounting_item_type: 2,
                }],
            }],
            total: 1,
        }
    );
    roundtrip_eq!(
        test_query_payment_detail_response_serialization,
        payment_detail::query::QueryResponse,
        payment_detail::query::QueryResponse {
            payment_details: vec![payment_detail::query::PaymentDetail {
                employee_id: "ou_1".to_string(),
                activity_id: "pa_1".to_string(),
                payment_accounting_items: vec![payment_detail::query::PaymentAccountingItem {
                    id: "acct_1".to_string(),
                    accounting_item_names: vec![payment_detail::query::I18nContent {
                        locale: "zh_cn".to_string(),
                        value: "基本工资".to_string(),
                    }],
                    accounting_item_value: payment_detail::query::AccountingItemValue {
                        original_value: Some("20000".to_string()),
                        reference_values: vec![],
                    },
                    segment_values: vec![],
                    accounting_item_type: 2,
                }],
            }],
            total: 1,
        }
    );
    roundtrip_eq!(
        test_list_datasource_response_serialization,
        datasource::list::ListResponse,
        datasource::list::ListResponse {
            page_token: Some("next_ds".to_string()),
            has_more: false,
            datasources: Some(vec![datasource::list::DatasourceItem {
                code: Some("test_datasource__c".to_string()),
                i18n_names: Some(vec![datasource::list::I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("外部薪资系统".to_string()),
                    id: Some("723123123123123213".to_string()),
                }]),
                active_status: Some(1),
                fields: Some(vec![datasource::list::DatasourceField {
                    code: Some("test__c".to_string()),
                    i18n_names: Some(vec![datasource::list::I18nContent {
                        locale: Some("zh_cn".to_string()),
                        value: Some("基本工资".to_string()),
                        id: None,
                    }]),
                    field_type: Some(1),
                    active_status: Some(1),
                    i18n_description: Some(vec![datasource::list::I18nContent {
                        locale: Some("zh_cn".to_string()),
                        value: Some("描述".to_string()),
                        id: None,
                    }]),
                    decimal_places: Some(1),
                }]),
                i18n_description: Some(vec![datasource::list::I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("外部薪资系统描述".to_string()),
                    id: None,
                }]),
                data_period_type: Some(1),
            }]),
        }
    );
    roundtrip_eq!(
        test_query_datasource_record_response_serialization,
        datasource_record::query::QueryResponse,
        datasource_record::query::QueryResponse {
            page_token: Some("next".to_string()),
            has_more: false,
            records: vec![datasource_record::query::DatasourceRecord {
                active_status: 1,
                field_values: vec![datasource_record::query::DatasourceRecordFieldValue {
                    field_code: "base_salary__c".to_string(),
                    value: "20000".to_string(),
                    field_type: None,
                }],
            }],
        }
    );
    roundtrip_eq!(
        test_save_datasource_record_response_serialization,
        datasource_record::save::SaveResponse,
        datasource_record::save::SaveResponse {
            affect_counts: "10".to_string(),
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_plan_response_serialization,
        cost_allocation_plan::list::ListResponse,
        cost_allocation_plan::list::ListResponse {
            items: vec![cost_allocation_plan::list::CostAllocationPlan {
                id: Some("6823630319749593096".to_string()),
                names: Some(vec![cost_allocation_plan::list::I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("研发成本分摊".to_string()),
                    id: None,
                }]),
                applicable_country_region: Some("6823630319749593389".to_string()),
                dimensions: Some(vec![cost_allocation_plan::list::Dimension {
                    i18n_names: Some(vec![cost_allocation_plan::list::I18nContent {
                        locale: Some("zh_cn".to_string()),
                        value: Some("公司主体".to_string()),
                        id: Some("723123123123123213".to_string()),
                    }]),
                    api_name: Some("company".to_string()),
                    obj_api_name: Some("jobData".to_string()),
                }]),
                cost_items: Some(vec![cost_allocation_plan::list::CostItem {
                    id: Some("7433424967234601004".to_string()),
                    name: Some(vec![cost_allocation_plan::list::I18nContent {
                        locale: Some("zh_cn".to_string()),
                        value: Some("基本工资".to_string()),
                        id: None,
                    }]),
                    enable_correct: Some(true),
                }]),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_detail_response_serialization,
        cost_allocation_detail::list::ListResponse,
        cost_allocation_detail::list::ListResponse {
            cost_allocation_report_datas: Some(vec![
                cost_allocation_detail::list::CostAllocationReportData {
                    data_summary_dimensions: Some(vec![
                        cost_allocation_detail::list::DataSummaryDimension {
                            dimension_level: Some(1),
                            dimension_type: Some(1),
                            dimension_value_id: Some("6823630319749580306".to_string()),
                            enum_dimension: Some(cost_allocation_detail::list::EnumDimension {
                                enum_value_id: Some("7188920315914207276".to_string()),
                                enum_key: Some("company".to_string()),
                            }),
                            dimension_value_lookup_info: Some(
                                cost_allocation_detail::list::DimensionValueLookupInfo {
                                    r#type: Some("work_calendar".to_string()),
                                    id: Some("6961286846093788621".to_string()),
                                    code: Some("D1230011115".to_string()),
                                },
                            ),
                            dimension_names: Some(vec![
                                cost_allocation_detail::list::I18nContent {
                                    locale: Some("zh_cn".to_string()),
                                    value: Some("名称".to_string()),
                                    id: None,
                                }
                            ]),
                            dimension_titles: Some(vec![
                                cost_allocation_detail::list::I18nContent {
                                    locale: Some("zh_cn".to_string()),
                                    value: Some("名称".to_string()),
                                    id: Some("723123123123123213".to_string()),
                                }
                            ]),
                        }
                    ]),
                    compensation_cost_item: Some(
                        cost_allocation_detail::list::CompensationCostItem {
                            compensation_costs: Some(vec![
                                cost_allocation_detail::list::CompensationCost {
                                    compensation_cost_value: Some("123456.78".to_string()),
                                    i18n_names: Some(vec![
                                        cost_allocation_detail::list::I18nContent {
                                            locale: Some("zh_cn".to_string()),
                                            value: Some("名称".to_string()),
                                            id: None,
                                        }
                                    ]),
                                }
                            ]),
                        }
                    ),
                    employment_id: Some("72321836123692186".to_string()),
                }
            ]),
            cost_allocation_report_names: Some(vec![cost_allocation_detail::list::I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("名称".to_string()),
                id: None,
            }]),
            pay_period: "2024-06".to_string(),
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_list_cost_allocation_report_response_serialization,
        cost_allocation_report::list::ListResponse,
        cost_allocation_report::list::ListResponse {
            pay_period: "2023-11".to_string(),
            cost_allocation_report_names: Some(vec![cost_allocation_report::list::I18nContent {
                locale: Some("zh_cn".to_string()),
                value: Some("名称".to_string()),
                id: None,
            }]),
            cost_allocation_report_datas: Some(vec![
                cost_allocation_report::list::CostAllocationReportData {
                    data_summary_dimensions: Some(vec![
                        cost_allocation_report::list::DataSummaryDimension {
                            dimension_level: Some(1),
                            dimension_type: Some(1),
                            dimension_value_id: Some("6823630319749580306".to_string()),
                            enum_dimension: Some(cost_allocation_report::list::EnumDimension {
                                enum_value_id: Some("7188920315914207276".to_string()),
                                enum_key: Some("company".to_string()),
                            }),
                            dimension_value_lookup_info: Some(
                                cost_allocation_report::list::DimensionValueLookupInfo {
                                    r#type: Some("work_calendar".to_string()),
                                    id: Some("6961286846093788621".to_string()),
                                    code: Some("D1230011115".to_string()),
                                },
                            ),
                            dimension_names: Some(vec![
                                cost_allocation_report::list::I18nContent {
                                    locale: Some("zh_cn".to_string()),
                                    value: Some("名称".to_string()),
                                    id: None,
                                }
                            ]),
                            dimension_titles: Some(vec![
                                cost_allocation_report::list::I18nContent {
                                    locale: Some("zh_cn".to_string()),
                                    value: Some("名称".to_string()),
                                    id: Some("723123123123123213".to_string()),
                                }
                            ]),
                        },
                    ]),
                    compensation_cost_item: Some(
                        cost_allocation_report::list::CompensationCostItem {
                            number_of_individuals_for_payment: Some(100),
                            compensation_costs: Some(vec![
                                cost_allocation_report::list::CompensationCost {
                                    compensation_cost_value: Some("123456.78".to_string()),
                                    i18n_names: Some(vec![
                                        cost_allocation_report::list::I18nContent {
                                            locale: Some("zh_cn".to_string()),
                                            value: Some("名称".to_string()),
                                            id: None,
                                        }
                                    ]),
                                }
                            ]),
                        }
                    ),
                },
            ]),
            has_more: false,
            page_token: Some("next_report".to_string()),
        }
    );
    roundtrip_eq!(
        test_list_acct_item_response_serialization,
        acct_item::list::ListResponse,
        acct_item::list::ListResponse {
            items: vec![acct_item::list::AcctItem {
                id: "acct_1".to_string(),
                i18n_names: vec![acct_item::list::I18nContent {
                    locale: "zh_cn".to_string(),
                    value: "基本工资".to_string(),
                }],
                category_id: "cat_1".to_string(),
                data_type: 2,
                decimal_places: 2,
                active_status: 1,
            }],
            has_more: false,
            page_token: None,
        }
    );
}
