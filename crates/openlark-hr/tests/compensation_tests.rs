//! compensation tests 集成测试。
#![cfg(feature = "compensation")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::compensation_management::compensation::v1::{
    archive, change_reason, indicator, item, item_category, lump_sum_payment, plan,
    recurring_payment, social_archive, social_archive_adjust_record, social_insurance, social_plan,
};
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
        test_archive_create_request_builder,
        archive::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_archive_query_request_builder,
        archive::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_archive".to_string())
    );
    smoke_builder!(
        test_change_reason_list_request_builder,
        change_reason::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_indicator_list_request_builder,
        indicator::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_indicator".to_string())
    );
    smoke_builder!(
        test_item_list_request_builder,
        item::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_item".to_string())
    );
    smoke_builder!(
        test_item_category_list_request_builder,
        item_category::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_lump_sum_batch_create_request_builder,
        lump_sum_payment::batch_create::BatchCreateRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_lump_sum_batch_remove_request_builder,
        lump_sum_payment::batch_remove::BatchRemoveRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_lump_sum_batch_update_request_builder,
        lump_sum_payment::batch_update::BatchUpdateRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_lump_sum_query_request_builder,
        lump_sum_payment::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_lump".to_string())
    );
    smoke_builder!(
        test_lump_sum_query_detail_request_builder,
        lump_sum_payment::query_detail::QueryDetailRequest::new(test_config(
            "https://open.feishu.cn"
        ))
        .page_size(20)
        .page_token("next_lump_detail".to_string())
    );
    smoke_builder!(
        test_plan_list_request_builder,
        plan::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_plan".to_string())
    );
    smoke_builder!(
        test_recurring_batch_create_request_builder,
        recurring_payment::batch_create::BatchCreateRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_recurring_batch_remove_request_builder,
        recurring_payment::batch_remove::BatchRemoveRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_recurring_batch_update_request_builder,
        recurring_payment::batch_update::BatchUpdateRequest::new(test_config(
            "https://open.feishu.cn"
        ))
    );
    smoke_builder!(
        test_recurring_query_request_builder,
        recurring_payment::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_recurring".to_string())
    );
    smoke_builder!(
        test_social_archive_query_request_builder,
        social_archive::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["ou_1".to_string()],
            "2024-01-01".to_string(),
        )
        .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_social_archive_adjust_record_query_request_builder,
        social_archive_adjust_record::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["ou_1".to_string()],
            "increase".to_string(),
        )
        .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_social_insurance_list_request_builder,
        social_insurance::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_social_plan_list_request_builder,
        social_plan::list::ListRequest::new(
            test_config("https://open.feishu.cn"),
            "2024-01-01".to_string(),
        )
        .page_size(20)
        .page_token("next_social_plan".to_string())
    );
    smoke_builder!(
        test_social_plan_query_request_builder,
        social_plan::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["plan_1".to_string()],
            "2024-01-01".to_string(),
        )
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_archive_create_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        // CreateResponse 字段均为 Option：缺 data 时 Transport 以 `{}` 兼容解码（#422）
        let data = archive::create::CreateRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .expect("empty success should decode to empty CreateResponse");
        assert!(data.unique_id.is_none());
        assert!(data.archive_tid.is_none());
    }

    #[tokio::test]
    async fn test_archive_query_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = archive::query::QueryRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(
            err.contains("批量查询员工薪资档案响应数据为空")
                || err.contains("成功响应缺少必需的 data")
                || err.contains("api_response_data"),
            "unexpected err: {err}"
        );
    }

    #[tokio::test]
    async fn test_social_plan_list_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_plans"))
            .and(query_param("effective_date", "2024-01-01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = social_plan::list::ListRequest::new(
            test_config(&mock_server.uri()),
            "2024-01-01".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .err()
        .unwrap()
        .to_string();
        assert!(
            err.contains("查询参保方案响应数据为空")
                || err.contains("成功响应缺少必需的 data")
                || err.contains("api_response_data"),
            "unexpected err: {err}"
        );
    }

    #[tokio::test]
    async fn test_social_plan_query_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_plans/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = social_plan::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["plan_1".to_string()],
            "2024-01-01".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .err()
        .unwrap()
        .to_string();
        assert!(
            err.contains("查询参保方案响应数据为空")
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
    async fn test_archive_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"unique_id":"123e4567-e89b-42d3-a456-426614174000","archive_tid":"arc_1"}
            })))
            .mount(&mock_server)
            .await;

        let resp = archive::create::CreateRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(
            resp.unique_id,
            Some("123e4567-e89b-42d3-a456-426614174000".to_string())
        );
        assert_eq!(resp.archive_tid, Some("arc_1".to_string()));
    }

    #[tokio::test]
    async fn test_archive_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives/query"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_archive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"arc_1","user_id":"ou_1","plan_id":"plan_1"}],"has_more":false,"page_token":"next_archive_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = archive::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_archive".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].id, "arc_1");
    }

    #[tokio::test]
    async fn test_change_reason_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/change_reasons"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"reason_1","name":"晋升","note":"晋升调薪","active_status":1,"i18n_names":[{"locale":"zh_cn","value":"晋升"}],"i18n_notes":[{"locale":"zh_cn","value":"晋升调薪"}]}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = change_reason::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].name.as_deref(), Some("晋升"));
    }

    #[tokio::test]
    async fn test_indicator_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/indicators"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_indicator"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"indicator_1","name":"总包","value_type":"money","active_status":1,"i18n_names":[{"locale":"zh_cn","value":"总包"}]}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = indicator::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_indicator".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].id, "indicator_1");
        assert_eq!(resp.items[0].value_type.as_deref(), Some("money"));
        assert_eq!(resp.items[0].active_status, Some(1));
    }

    #[tokio::test]
    async fn test_item_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/items"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_item"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"item_1","name":"基本工资","description":"每月份的薪酬","category_id":"cat_1","value_type":"money","pay_off_frequency_type":"month","decimal_places":2,"active_status":1,"i18n_names":[{"locale":"zh_cn","value":"基本工资"}],"i18n_descriptions":[{"locale":"zh_cn","value":"每月份的薪酬"}]}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = item::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_item".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].value_type.as_deref(), Some("money"));
        assert_eq!(resp.items[0].decimal_places, Some(2));
    }

    #[tokio::test]
    async fn test_item_category_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/item_categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"cat_1","name":"固定","i18n_names":[{"locale":"zh_cn","value":"固定"}]}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = item_category::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(
            resp.items[0].i18n_names.as_ref().unwrap()[0]
                .locale
                .as_deref(),
            Some("zh_cn")
        );
    }

    #[tokio::test]
    async fn test_lump_sum_batch_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/lump_sum_payment/batch_create",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"lump_1","unique_id":"lump_unique_1","code":0,"message":"Success"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_create::BatchCreateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.operate_results.unwrap()[0].code, Some(0));
    }

    #[tokio::test]
    async fn test_lump_sum_batch_remove_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/lump_sum_payment/batch_remove",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"lump_1","unique_id":"lump_unique_1","code":21270202,"message":"uqniue id conflict"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_remove::BatchRemoveRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(
            resp.operate_results.unwrap()[0].id,
            Some("lump_1".to_string())
        );
    }

    #[tokio::test]
    async fn test_lump_sum_batch_update_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/lump_sum_payment/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"lump_1","unique_id":"lump_unique_1","code":0,"message":"Success"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_update::BatchUpdateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(
            resp.operate_results.unwrap()[0].message.as_deref(),
            Some("Success")
        );
    }

    #[tokio::test]
    async fn test_lump_sum_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/lump_sum_payment/query"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_lump"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"records":[{"id":"lump_1","user_id":"ou_1","total_amount":"1000.00","binding_period":2,"currency_id":"cur_1","issuance_frequency":1,"item_id":"item_1","details":[{"id":"detail_1","record_id":"lump_1","user_id":"ou_1","issuance_amount":"1000.00","issuance_status":"to_be_issued","issuance_way":"with_salary","issuance_time":"2024-08-01","currency_id":"cur_1"}],"binding_period_decimal":"2"}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_lump".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.records[0].total_amount.as_deref(), Some("1000.00"));
        assert_eq!(
            resp.records[0].details.as_ref().unwrap()[0]
                .issuance_amount
                .as_deref(),
            Some("1000.00")
        );
    }

    #[tokio::test]
    async fn test_lump_sum_query_detail_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/lump_sum_payment/query_detail"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_lump_detail"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"records":[{"id":"detail_1","record_id":"lump_1","user_id":"ou_1","issuance_amount":"1000.00","issuance_status":"to_be_issued","issuance_way":"with_salary","issuance_time":"2024-08-01","currency_id":"cur_1","belong_time":"2025-01-20","create_time":"2024-08-01 12:34:56","modify_time":"2024-08-01 12:34:56","issuance_country_region_id":"cr_1","issuance_pay_group_id":"pg_1","detail_reference_period_start_date":"2024-08-01","detail_reference_period_end_date":"2024-08-01"}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::query_detail::QueryDetailRequest::new(test_config(
            &mock_server.uri(),
        ))
        .page_size(20)
        .page_token("next_lump_detail".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.records[0].record_id, Some("lump_1".to_string()));
        assert_eq!(
            resp.records[0].issuance_status,
            Some("to_be_issued".to_string())
        );
    }

    #[tokio::test]
    async fn test_plan_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/plans"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_plan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"plan_1","tid":"tid_1","name":"标准方案","description":"标准方案描述","effective_date":"2024-01-01","plan_scope":{"is_all":false,"plan_conditions":[{"left_type":7,"operator":1,"right_value":["dept_1"]}]},"currency_id":"cur_1","probation_salary_status":true,"i18n_names":[{"locale":"zh_cn","value":"标准方案"}],"i18n_descriptions":[{"locale":"zh_cn","value":"标准方案描述"}]}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = plan::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_plan".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].name, "标准方案");
        assert_eq!(resp.items[0].effective_date, "2024-01-01");
    }

    #[tokio::test]
    async fn test_recurring_batch_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_create",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"rec_1","unique_id":"u_1","code":0,"message":"Success"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_create::BatchCreateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        let results = resp.operate_results.unwrap();
        assert_eq!(results[0].id, Some("rec_1".to_string()));
        assert_eq!(results[0].code, Some(0));
    }

    #[tokio::test]
    async fn test_recurring_batch_remove_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_remove",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"rec_1","unique_id":"u_1","code":0,"message":"Success"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_remove::BatchRemoveRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(
            resp.operate_results.unwrap()[0].id,
            Some("rec_1".to_string())
        );
    }

    #[tokio::test]
    async fn test_recurring_batch_update_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/recurring_payment/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"operate_results":[{"id":"rec_1","unique_id":"u_1","code":0,"message":"Success"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_update::BatchUpdateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        let results = resp.operate_results.unwrap();
        assert_eq!(results[0].id, Some("rec_1".to_string()));
        assert_eq!(results[0].code, Some(0));
    }

    #[tokio::test]
    async fn test_recurring_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/recurring_payment/query"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_recurring"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"records":[{"id":"rec_1","unique_id":"u_1","user_id":"ou_1","item_id":"item_1","issuance_type":"with_salary","each_amount":"3000.00","start_date":"2024-01-10","end_date":"2025-01-20","issuance_period":"year","currency_id":"cur_1","remark":"备注","issuance_country_region_id":"cr_1"}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_recurring".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.records[0].id, "rec_1");
        assert_eq!(resp.records[0].each_amount.as_deref(), Some("3000.00"));
    }

    #[tokio::test]
    async fn test_social_archive_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_archive/query"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "user_id_list": ["ou_1"],
                "effective_date": "2024-01-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"archives":[{
                    "user_id":"ou_1",
                    "details":[{
                        "description":{"zh_cn":"2024年社保基数调整","en_us":"Adjustment"},
                        "insurance_type":"social_insurance",
                        "insurance_status":"contribution",
                        "id":"sa_1",
                        "tid":"tid_1",
                        "plan_id":"plan_1",
                        "location_id":"loc_1",
                        "company_id":"cmp_1",
                        "base_salary":"1000.10",
                        "insurance_details":[{
                            "insurance_id":"ins_1",
                            "company_deduction":"2000.20",
                            "company_setting":{
                                "lower_limit":"1000.00","upper_limit":"2000.00",
                                "payment_ratio":"8.00","payment_rounding_rule":"rounding",
                                "payment_decimals":2,"fixed_payment":"200.00"
                            },
                            "personal_deduction":"1000.20",
                            "personal_setting":{
                                "lower_limit":"1000.00","upper_limit":"2000.00",
                                "payment_ratio":"8.00","payment_rounding_rule":"round_down",
                                "payment_decimals":2,"fixed_payment":"200.00"
                            },
                            "payment_frequency":"monthly",
                            "payment_months":[1]
                        }],
                        "effective_date":"2024-01-01"
                    }]
                }]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_archive::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["ou_1".to_string()],
            "2024-01-01".to_string(),
        )
        .user_id_type("open_id".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.archives[0].user_id, "ou_1");
        assert_eq!(resp.archives[0].details[0].id, Some("sa_1".to_string()));
        assert_eq!(
            resp.archives[0].details[0].base_salary,
            Some("1000.10".to_string())
        );
    }

    #[tokio::test]
    async fn test_social_archive_adjust_record_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/compensation/v1/social_archive_adjust_record/query",
            ))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "user_id_list": ["ou_1"],
                "record_type": "increase"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"records":[{
                    "user_id":"ou_1",
                    "record_type":"increase",
                    "details":[{
                        "description":{"zh_cn":"增员","en_us":"Increase"},
                        "insurance_type":"provident_fund",
                        "insurance_status":"contribution",
                        "plan_id":"plan_1",
                        "effective_date":"2024-01-01"
                    }]
                }]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_archive_adjust_record::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["ou_1".to_string()],
            "increase".to_string(),
        )
        .user_id_type("open_id".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.records[0].user_id, "ou_1");
        assert_eq!(resp.records[0].record_type, "increase");
        assert_eq!(resp.records[0].details[0].insurance_type, "provident_fund");
    }

    #[tokio::test]
    async fn test_social_insurance_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_insurances"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"ins_1","name":{"zh_cn":"养老保险","en_us":"endowment insurance"},"insurance_type":"social_insurance","active":true,"is_system":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_insurance::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].name.zh_cn, Some("养老保险".to_string()));
        assert_eq!(resp.items[0].insurance_type, "social_insurance");
        assert_eq!(resp.items[0].active, Some(true));
    }

    #[tokio::test]
    async fn test_social_plan_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_plans"))
            .and(query_param("effective_date", "2024-01-01"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_social_plan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{
                    "plans":[{
                        "plan_id":"sp_1",
                        "plan_tid":"tid_1",
                        "name":{"zh_cn":"标准参保","en_us":"Standard plan"},
                        "effective_date":"2024-01-01",
                        "active":true,
                        "insurance_type":"social_insurance",
                        "scope":{"is_all":true,"rules":[[{"left_type":1,"operator":1,"right_values":["city_1"]}]]},
                        "item_detail":[],
                        "remark":{"zh_cn":"备注","en_us":"remark"}
                    }],
                    "has_more":false,
                    "page_token":"next_2"
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = social_plan::list::ListRequest::new(
            test_config(&mock_server.uri()),
            "2024-01-01".to_string(),
        )
        .page_size(20)
        .page_token("next_social_plan".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.plans[0].plan_id, "sp_1");
        assert_eq!(resp.page_token, Some("next_2".to_string()));
    }

    #[tokio::test]
    async fn test_social_plan_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_plans/query"))
            .and(body_json(
                json!({"plan_ids":["plan_1"],"effective_date":"2024-01-01"}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"plans":[{
                    "plan_id":"sp_1",
                    "plan_tid":"tid_1",
                    "name":{"zh_cn":"标准参保","en_us":"Standard plan"},
                    "effective_date":"2024-01-01",
                    "active":true,
                    "insurance_type":"social_insurance",
                    "item_detail":[{
                        "item_id":"ins_1",
                        "item_name":{"zh_cn":"养老保险","en_us":"Endowment"},
                        "item_setting_of_person":{
                            "lower_limit":"1000.00","upper_limit":"2000.00",
                            "payment_ratio":"8.00","payment_rounding_rule":"rounding",
                            "payment_decimals":2,"fixed_payment":"200.00"
                        },
                        "item_setting_of_company":{
                            "lower_limit":"1000.00","upper_limit":"2000.00",
                            "payment_ratio":"8.00","payment_rounding_rule":"rounding",
                            "payment_decimals":2,"fixed_payment":"200.00"
                        },
                        "payment_frequency":"monthly",
                        "payment_months":[1]
                    }],
                    "remark":{"zh_cn":"备注","en_us":"remark"}
                }]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_plan::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["plan_1".to_string()],
            "2024-01-01".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.plans[0].plan_id, "sp_1");
        assert_eq!(resp.plans[0].name.zh_cn, Some("标准参保".to_string()));
        assert_eq!(resp.plans[0].effective_date, "2024-01-01");
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
        test_archive_create_response_serialization,
        archive::create::CreateResponse,
        archive::create::CreateResponse {
            unique_id: Some("123e4567-e89b-42d3-a456-426614174000".to_string()),
            archive_tid: Some("arc_1".to_string()),
        }
    );
    roundtrip_eq!(
        test_archive_query_response_serialization,
        archive::query::QueryResponse,
        archive::query::QueryResponse {
            items: vec![archive::query::Archive {
                id: "arc_1".to_string(),
                user_id: Some("ou_1".to_string()),
                tid: Some("tid_1".to_string()),
                plan_id: Some("plan_1".to_string()),
                plan_tid: Some("plan_tid_1".to_string()),
                currency_id: Some("cur_1".to_string()),
                change_reason_id: None,
                change_description: None,
                effective_date: Some("2024-01-01".to_string()),
                expiration_date: None,
                salary_level_id: None,
                created_time: None,
                updated_time: None,
                archive_items: Some(vec![archive::query::ArchiveItem {
                    item_id: Some("item_1".to_string()),
                    item_result: Some("15000".to_string()),
                    item_result_regular: Some("18000".to_string()),
                }]),
                archive_indicators: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_change_reason_list_response_serialization,
        change_reason::list::ListResponse,
        change_reason::list::ListResponse {
            items: vec![change_reason::list::ChangeReason {
                id: "reason_1".to_string(),
                name: Some("晋升".to_string()),
                note: Some("晋升调薪".to_string()),
                active_status: Some(1),
                i18n_names: Some(vec![openlark_hr::common::shared_models::I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("晋升".to_string()),
                }]),
                i18n_notes: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_indicator_list_response_serialization,
        indicator::list::ListResponse,
        indicator::list::ListResponse {
            items: vec![indicator::list::Indicator {
                id: "ind_1".to_string(),
                name: "总包".to_string(),
                value_type: Some("money".to_string()),
                active_status: Some(1),
                i18n_names: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_item_list_response_serialization,
        item::list::ListResponse,
        item::list::ListResponse {
            items: vec![item::list::CompensationItem {
                id: "item_1".to_string(),
                name: "基本工资".to_string(),
                description: Some("每月份的薪酬".to_string()),
                category_id: None,
                value_type: Some("money".to_string()),
                pay_off_frequency_type: Some("month".to_string()),
                decimal_places: Some(2),
                active_status: Some(1),
                i18n_names: None,
                i18n_descriptions: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_item_category_list_response_serialization,
        item_category::list::ListResponse,
        item_category::list::ListResponse {
            items: vec![item_category::list::ItemCategory {
                id: "cat_1".to_string(),
                name: "固定".to_string(),
                i18n_names: Some(vec![openlark_hr::common::shared_models::I18nContent {
                    locale: Some("zh_cn".to_string()),
                    value: Some("固定".to_string()),
                }]),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_create_response_serialization,
        lump_sum_payment::batch_create::BatchCreateResponse,
        lump_sum_payment::batch_create::BatchCreateResponse {
            operate_results: Some(vec![lump_sum_payment::batch_create::BatchCreateResult {
                id: Some("lump_1".to_string()),
                unique_id: Some("lump_unique_1".to_string()),
                code: Some(0),
                message: Some("Success".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_remove_response_serialization,
        lump_sum_payment::batch_remove::BatchRemoveResponse,
        lump_sum_payment::batch_remove::BatchRemoveResponse {
            operate_results: Some(vec![lump_sum_payment::batch_remove::BatchRemoveResult {
                id: Some("lump_1".to_string()),
                unique_id: Some("lump_unique_1".to_string()),
                code: Some(21270202),
                message: Some("uqniue id conflict".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_update_response_serialization,
        lump_sum_payment::batch_update::BatchUpdateResponse,
        lump_sum_payment::batch_update::BatchUpdateResponse {
            operate_results: Some(vec![lump_sum_payment::batch_update::BatchUpdateResult {
                id: Some("lump_1".to_string()),
                unique_id: Some("lump_unique_1".to_string()),
                code: Some(0),
                message: Some("Success".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_query_response_serialization,
        lump_sum_payment::query::QueryResponse,
        lump_sum_payment::query::QueryResponse {
            records: vec![lump_sum_payment::query::LumpSumPayment {
                id: "lump_1".to_string(),
                unique_id: Some("lump_unique_1".to_string()),
                user_id: Some("ou_1".to_string()),
                total_amount: Some("1000.00".to_string()),
                binding_period: Some(2),
                currency_id: Some("cur_1".to_string()),
                issuance_frequency: Some(1),
                item_id: Some("item_1".to_string()),
                remark: None,
                issuance_detail_text: None,
                apply_source: Some(1),
                return_amount_before_tax: None,
                return_amount_after_tax: None,
                binding_period_offboarding_type: None,
                create_time: None,
                modify_time: None,
                reference_period_start_date: None,
                reference_period_end_date: None,
                details: None,
                binding_period_decimal: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_lump_sum_query_detail_response_serialization,
        lump_sum_payment::query_detail::QueryDetailResponse,
        lump_sum_payment::query_detail::QueryDetailResponse {
            records: vec![lump_sum_payment::query_detail::LumpSumPaymentDetail {
                id: "detail_1".to_string(),
                record_id: Some("lump_1".to_string()),
                user_id: Some("ou_1".to_string()),
                issuance_amount: Some("1000.00".to_string()),
                issuance_status: Some("to_be_issued".to_string()),
                issuance_way: Some("with_salary".to_string()),
                issuance_time: Some("2024-08-01".to_string()),
                currency_id: Some("cur_1".to_string()),
                belong_time: None,
                create_time: None,
                modify_time: None,
                issuance_country_region_id: None,
                issuance_pay_group_id: None,
                detail_reference_period_start_date: None,
                detail_reference_period_end_date: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_plan_list_response_serialization,
        plan::list::ListResponse,
        plan::list::ListResponse {
            items: vec![plan::list::Plan {
                id: "plan_1".to_string(),
                tid: Some("tid_1".to_string()),
                name: "标准方案".to_string(),
                description: Some("标准方案描述".to_string()),
                effective_date: "2024-01-01".to_string(),
                plan_scope: Some(plan::list::PlanScope {
                    is_all: Some(false),
                    plan_conditions: Some(vec![plan::list::PlanCondition {
                        left_type: Some(7),
                        operator: Some(1),
                        right_value: Some(vec!["dept_1".to_string()]),
                    }]),
                }),
                currency_id: Some("cur_1".to_string()),
                probation_salary_status: Some(true),
                plan_items: None,
                plan_indicators: None,
                i18n_names: None,
                i18n_descriptions: None,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_create_response_serialization,
        recurring_payment::batch_create::BatchCreateResponse,
        recurring_payment::batch_create::BatchCreateResponse {
            operate_results: Some(vec![recurring_payment::batch_create::BatchCreateResult {
                id: Some("rec_1".to_string()),
                unique_id: Some("u_1".to_string()),
                code: Some(0),
                message: Some("Success".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_remove_response_serialization,
        recurring_payment::batch_remove::BatchRemoveResponse,
        recurring_payment::batch_remove::BatchRemoveResponse {
            operate_results: Some(vec![recurring_payment::batch_remove::BatchRemoveResult {
                id: Some("rec_1".to_string()),
                unique_id: Some("u_1".to_string()),
                code: Some(0),
                message: Some("Success".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_update_response_serialization,
        recurring_payment::batch_update::BatchUpdateResponse,
        recurring_payment::batch_update::BatchUpdateResponse {
            operate_results: Some(vec![recurring_payment::batch_update::BatchUpdateResult {
                id: Some("rec_1".to_string()),
                unique_id: Some("u_1".to_string()),
                code: Some(0),
                message: Some("Success".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_query_response_serialization,
        recurring_payment::query::QueryResponse,
        recurring_payment::query::QueryResponse {
            records: vec![recurring_payment::query::RecurringPayment {
                id: "rec_1".to_string(),
                unique_id: Some("u_1".to_string()),
                user_id: Some("ou_1".to_string()),
                item_id: Some("item_1".to_string()),
                issuance_type: Some("with_salary".to_string()),
                each_amount: Some("3000.00".to_string()),
                start_date: Some("2024-01-10".to_string()),
                end_date: Some("2025-01-20".to_string()),
                issuance_period: Some("year".to_string()),
                currency_id: Some("cur_1".to_string()),
                remark: Some("备注".to_string()),
                issuance_country_region_id: Some("cr_1".to_string()),
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_archive_query_response_serialization,
        social_archive::query::QueryResponse,
        social_archive::query::QueryResponse {
            archives: vec![social_archive::query::SocialArchive {
                user_id: "ou_1".to_string(),
                details: vec![social_archive::query::SocialArchiveDetail {
                    description: social_archive::query::I18nDescription {
                        zh_cn: Some("2024年社保基数调整".to_string()),
                        en_us: None,
                    },
                    insurance_type: "social_insurance".to_string(),
                    insurance_status: "contribution".to_string(),
                    id: Some("sa_1".to_string()),
                    tid: None,
                    plan_id: Some("plan_1".to_string()),
                    plan_tid: None,
                    location_id: None,
                    company_id: None,
                    account_type: None,
                    insurance_account: None,
                    base_salary: Some("1000.10".to_string()),
                    insurance_details: None,
                    effective_date: Some("2024-01-01".to_string()),
                }],
            }],
        }
    );
    roundtrip_eq!(
        test_social_archive_adjust_record_query_response_serialization,
        social_archive_adjust_record::query::QueryResponse,
        social_archive_adjust_record::query::QueryResponse {
            records: vec![
                social_archive_adjust_record::query::SocialArchiveAdjustRecord {
                    user_id: "ou_1".to_string(),
                    record_type: "increase".to_string(),
                    details: vec![social_archive_adjust_record::query::SocialArchiveDetail {
                        description: social_archive_adjust_record::query::I18nDescription {
                            zh_cn: Some("增员".to_string()),
                            en_us: Some("Increase".to_string()),
                        },
                        insurance_type: "provident_fund".to_string(),
                        insurance_status: "contribution".to_string(),
                        id: None,
                        tid: None,
                        plan_id: Some("plan_1".to_string()),
                        plan_tid: None,
                        location_id: None,
                        company_id: None,
                        account_type: None,
                        insurance_account: None,
                        base_salary: None,
                        insurance_details: None,
                        effective_date: Some("2024-01-01".to_string()),
                    },],
                },
            ],
        }
    );
    roundtrip_eq!(
        test_social_insurance_list_response_serialization,
        social_insurance::list::ListResponse,
        social_insurance::list::ListResponse {
            items: vec![social_insurance::list::SocialInsurance {
                id: "ins_1".to_string(),
                name: openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("养老保险".to_string()),
                    en_us: Some("endowment insurance".to_string()),
                    extra: std::collections::HashMap::new(),
                },
                insurance_type: "social_insurance".to_string(),
                active: Some(true),
                is_system: Some(true),
            }],
        }
    );
    roundtrip_eq!(
        test_social_plan_list_response_serialization,
        social_plan::list::ListResponse,
        social_plan::list::ListResponse {
            plans: vec![social_plan::query::SocialPlan {
                plan_id: "sp_1".to_string(),
                plan_tid: "tid_1".to_string(),
                name: social_plan::query::I18nName {
                    zh_cn: Some("标准参保".to_string()),
                    en_us: Some("Standard plan".to_string()),
                },
                effective_date: "2024-01-01".to_string(),
                active: true,
                insurance_type: "social_insurance".to_string(),
                scope: None,
                item_detail: vec![],
                remark: social_plan::query::I18nName {
                    zh_cn: Some("备注".to_string()),
                    en_us: None,
                },
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_plan_query_response_serialization,
        social_plan::query::QueryResponse,
        social_plan::query::QueryResponse {
            plans: vec![social_plan::query::SocialPlan {
                plan_id: "sp_1".to_string(),
                plan_tid: "tid_1".to_string(),
                name: social_plan::query::I18nName {
                    zh_cn: Some("标准参保".to_string()),
                    en_us: Some("Standard plan".to_string()),
                },
                effective_date: "2024-01-01".to_string(),
                active: true,
                insurance_type: "social_insurance".to_string(),
                scope: None,
                item_detail: vec![social_plan::query::SocialPlanItemDetail {
                    item_id: "ins_1".to_string(),
                    item_name: social_plan::query::I18nName {
                        zh_cn: Some("养老保险".to_string()),
                        en_us: None,
                    },
                    item_setting_of_person: social_plan::query::SocialPlanItemSetting {
                        lower_limit: "1000.00".to_string(),
                        upper_limit: "2000.00".to_string(),
                        payment_ratio: "8.00".to_string(),
                        payment_rounding_rule: "rounding".to_string(),
                        payment_decimals: 2,
                        fixed_payment: "200.00".to_string(),
                    },
                    item_setting_of_company: social_plan::query::SocialPlanItemSetting {
                        lower_limit: "1000.00".to_string(),
                        upper_limit: "2000.00".to_string(),
                        payment_ratio: "8.00".to_string(),
                        payment_rounding_rule: "rounding".to_string(),
                        payment_decimals: 2,
                        fixed_payment: "200.00".to_string(),
                    },
                    payment_frequency: "monthly".to_string(),
                    payment_months: vec![1],
                }],
                remark: social_plan::query::I18nName {
                    zh_cn: Some("备注".to_string()),
                    en_us: None,
                },
            }],
        }
    );
}
