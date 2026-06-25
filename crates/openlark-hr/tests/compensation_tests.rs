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
        social_archive::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next_social_archive".to_string())
    );
    smoke_builder!(
        test_social_archive_adjust_record_query_request_builder,
        social_archive_adjust_record::query::QueryRequest::new(test_config(
            "https://open.feishu.cn"
        ))
        .page_size(20)
        .page_token("next_adjust_record".to_string())
    );
    smoke_builder!(
        test_social_insurance_list_request_builder,
        social_insurance::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_social_plan_list_request_builder,
        social_plan::list::ListRequest::new(test_config("https://open.feishu.cn"), 1735689600)
            .page_size(20)
            .page_token("next_social_plan".to_string())
    );
    smoke_builder!(
        test_social_plan_query_request_builder,
        social_plan::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["plan_1".to_string()],
            1735689600,
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

        let err = archive::create::CreateRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(err.contains("创建薪资档案响应数据为空"));
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
        assert!(err.contains("批量查询员工薪资档案响应数据为空"));
    }

    #[tokio::test]
    async fn test_social_plan_list_data_missing_validation() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_plans"))
            .and(query_param("effective_date", "1735689600"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"code":0,"msg":"ok"})))
            .mount(&mock_server)
            .await;

        let err = social_plan::list::ListRequest::new(test_config(&mock_server.uri()), 1735689600)
            .execute_with_options(auth_option())
            .await
            .err()
            .unwrap()
            .to_string();
        assert!(err.contains("查询参保方案响应数据为空"));
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
            1735689600,
        )
        .execute_with_options(auth_option())
        .await
        .err()
        .unwrap()
        .to_string();
        assert!(err.contains("查询参保方案响应数据为空"));
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
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"ok","data":{"archive_id":"arc_1"}})),
            )
            .mount(&mock_server)
            .await;

        let resp = archive::create::CreateRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.archive_id, Some("arc_1".to_string()));
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
                "data":{"items":[{"id":"reason_1","name":"晋升","reason_type":1}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = change_reason::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].name, "晋升");
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
                "data":{"items":[{"id":"indicator_1","name":"总包","indicator_type":1,"formula":"base+bonus"}],"has_more":false,"page_token":"next_2"}
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
                "data":{"items":[{"id":"item_1","name":"基本工资","item_type":1,"include_in_social_insurance":true}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = item::list::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_item".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.items[0].include_in_social_insurance);
    }

    #[tokio::test]
    async fn test_item_category_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/item_categories"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"cat_1","name":"固定","category_type":1}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = item_category::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].category_type, 1);
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
                "data":{"results":[{"id":"lump_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_create::BatchCreateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert!(resp.results.unwrap()[0].success);
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
                "data":{"results":[{"id":"lump_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_remove::BatchRemoveRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.results.unwrap()[0].id, Some("lump_1".to_string()));
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
                "data":{"results":[{"id":"lump_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::batch_update::BatchUpdateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert!(resp.results.unwrap()[0].success);
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
                "data":{"items":[{"id":"lump_1","user_id":"ou_1","amount":1000.0}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = lump_sum_payment::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_lump".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].amount, Some(1000.0));
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
                "data":{"items":[{"id":"detail_1","payment_id":"lump_1","user_id":"ou_1"}],"has_more":false,"page_token":"next_2"}
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
        assert_eq!(resp.items[0].payment_id, Some("lump_1".to_string()));
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
                "data":{"items":[{"id":"plan_1","name":"标准方案","status":1,"effective_date":1735689600}],"has_more":false,"page_token":"next_2"}
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
                "data":{"results":[{"id":"rec_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_create::BatchCreateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert!(resp.results.unwrap()[0].success);
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
                "data":{"results":[{"id":"rec_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_remove::BatchRemoveRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.results.unwrap()[0].id, Some("rec_1".to_string()));
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
                "data":{"results":[{"id":"rec_1","success":true}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::batch_update::BatchUpdateRequest::new(test_config(
            &mock_server.uri(),
        ))
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert!(resp.results.unwrap()[0].success);
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
                "data":{"items":[{"id":"rec_1","user_id":"ou_1","amount":3000.0}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = recurring_payment::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_recurring".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].id, "rec_1");
    }

    #[tokio::test]
    async fn test_social_archive_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_archive/query"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_social_archive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"sa_1","user_id":"ou_1","city":"上海"}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_archive::query::QueryRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .page_token("next_social_archive".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].city, Some("上海".to_string()));
    }

    #[tokio::test]
    async fn test_social_archive_adjust_record_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_archive_adjust_record/query"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_adjust_record"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"adj_1","user_id":"ou_1","adjust_type":2}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp =
            social_archive_adjust_record::query::QueryRequest::new(test_config(&mock_server.uri()))
                .page_size(20)
                .page_token("next_adjust_record".to_string())
                .execute_with_options(auth_option())
                .await
                .unwrap();
        assert_eq!(resp.items[0].adjust_type, Some(2));
    }

    #[tokio::test]
    async fn test_social_insurance_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_insurances"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"ins_1","name":"养老保险","insurance_type":1}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_insurance::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].name, "养老保险");
    }

    #[tokio::test]
    async fn test_social_plan_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_plans"))
            .and(query_param("effective_date", "1735689600"))
            .and(query_param("page_size", "20"))
            .and(query_param("page_token", "next_social_plan"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"sp_1","name":"标准参保","effective_date":1735689600}],"has_more":false,"page_token":"next_2"}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_plan::list::ListRequest::new(test_config(&mock_server.uri()), 1735689600)
            .page_size(20)
            .page_token("next_social_plan".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.items[0].id, "sp_1");
    }

    #[tokio::test]
    async fn test_social_plan_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/social_plans/query"))
            .and(body_json(
                json!({"plan_ids":["plan_1"],"effective_date":1735689600}),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"ok",
                "data":{"items":[{"id":"sp_1","name":"标准参保","effective_date":1735689600}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = social_plan::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["plan_1".to_string()],
            1735689600,
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.items[0].name, "标准参保");
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
            archive_id: Some("arc_1".to_string())
        }
    );
    roundtrip_eq!(
        test_archive_query_response_serialization,
        archive::query::QueryResponse,
        archive::query::QueryResponse {
            items: vec![archive::query::Archive {
                id: "arc_1".to_string(),
                user_id: Some("ou_1".to_string()),
                plan_id: Some("plan_1".to_string()),
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
                name: "晋升".to_string(),
                reason_type: 1,
            }],
        }
    );
    roundtrip_eq!(
        test_indicator_list_response_serialization,
        indicator::list::ListResponse,
        indicator::list::ListResponse {
            items: vec![indicator::list::Indicator {
                id: "ind_1".to_string(),
                name: "总包".to_string(),
                indicator_type: 1,
                formula: Some("base+bonus".to_string()),
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
                item_type: 1,
                include_in_social_insurance: true,
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
                category_type: 1,
            }],
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_create_response_serialization,
        lump_sum_payment::batch_create::BatchCreateResponse,
        lump_sum_payment::batch_create::BatchCreateResponse {
            results: Some(vec![lump_sum_payment::batch_create::BatchCreateResult {
                id: Some("lump_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_remove_response_serialization,
        lump_sum_payment::batch_remove::BatchRemoveResponse,
        lump_sum_payment::batch_remove::BatchRemoveResponse {
            results: Some(vec![lump_sum_payment::batch_remove::BatchRemoveResult {
                id: Some("lump_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_batch_update_response_serialization,
        lump_sum_payment::batch_update::BatchUpdateResponse,
        lump_sum_payment::batch_update::BatchUpdateResponse {
            results: Some(vec![lump_sum_payment::batch_update::BatchUpdateResult {
                id: Some("lump_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_lump_sum_query_response_serialization,
        lump_sum_payment::query::QueryResponse,
        lump_sum_payment::query::QueryResponse {
            items: vec![lump_sum_payment::query::LumpSumPayment {
                id: "lump_1".to_string(),
                user_id: Some("ou_1".to_string()),
                amount: Some(1000.0),
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_lump_sum_query_detail_response_serialization,
        lump_sum_payment::query_detail::QueryDetailResponse,
        lump_sum_payment::query_detail::QueryDetailResponse {
            items: vec![lump_sum_payment::query_detail::LumpSumPaymentDetail {
                id: "detail_1".to_string(),
                payment_id: Some("lump_1".to_string()),
                user_id: Some("ou_1".to_string()),
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
                name: "标准方案".to_string(),
                status: 1,
                effective_date: 1735689600,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_create_response_serialization,
        recurring_payment::batch_create::BatchCreateResponse,
        recurring_payment::batch_create::BatchCreateResponse {
            results: Some(vec![recurring_payment::batch_create::BatchCreateResult {
                id: Some("rec_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_remove_response_serialization,
        recurring_payment::batch_remove::BatchRemoveResponse,
        recurring_payment::batch_remove::BatchRemoveResponse {
            results: Some(vec![recurring_payment::batch_remove::BatchRemoveResult {
                id: Some("rec_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_batch_update_response_serialization,
        recurring_payment::batch_update::BatchUpdateResponse,
        recurring_payment::batch_update::BatchUpdateResponse {
            results: Some(vec![recurring_payment::batch_update::BatchUpdateResult {
                id: Some("rec_1".to_string()),
                success: true,
            }]),
        }
    );
    roundtrip_eq!(
        test_recurring_query_response_serialization,
        recurring_payment::query::QueryResponse,
        recurring_payment::query::QueryResponse {
            items: vec![recurring_payment::query::RecurringPayment {
                id: "rec_1".to_string(),
                user_id: Some("ou_1".to_string()),
                amount: Some(3000.0),
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_archive_query_response_serialization,
        social_archive::query::QueryResponse,
        social_archive::query::QueryResponse {
            items: vec![social_archive::query::SocialArchive {
                id: "sa_1".to_string(),
                user_id: Some("ou_1".to_string()),
                city: Some("上海".to_string()),
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_archive_adjust_record_query_response_serialization,
        social_archive_adjust_record::query::QueryResponse,
        social_archive_adjust_record::query::QueryResponse {
            items: vec![
                social_archive_adjust_record::query::SocialArchiveAdjustRecord {
                    id: "adj_1".to_string(),
                    user_id: Some("ou_1".to_string()),
                    adjust_type: Some(2),
                }
            ],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_insurance_list_response_serialization,
        social_insurance::list::ListResponse,
        social_insurance::list::ListResponse {
            items: vec![social_insurance::list::SocialInsurance {
                id: "ins_1".to_string(),
                name: "养老保险".to_string(),
                insurance_type: 1,
            }],
        }
    );
    roundtrip_eq!(
        test_social_plan_list_response_serialization,
        social_plan::list::ListResponse,
        social_plan::list::ListResponse {
            items: vec![social_plan::list::SocialPlan {
                id: "sp_1".to_string(),
                name: "标准参保".to_string(),
                effective_date: 1735689600,
            }],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_social_plan_query_response_serialization,
        social_plan::query::QueryResponse,
        social_plan::query::QueryResponse {
            items: vec![social_plan::query::SocialPlan {
                id: "sp_1".to_string(),
                name: "标准参保".to_string(),
                effective_date: 1735689600,
            }],
        }
    );
}
