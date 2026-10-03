//! performance tests 集成测试。
#![cfg(feature = "performance")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::performance::performance::{v1, v2};
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
        test_activity_query_builder,
        v2::activity::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .name("项目A".to_string())
            .page_size(20)
            .page_token("next".to_string())
    );
    smoke_builder!(
        test_additional_information_import_builder,
        v2::additional_information::import::ImportRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .add_information("ou_1".to_string(), "补充说明".to_string())
    );
    smoke_builder!(
        test_additional_information_query_builder,
        v2::additional_information::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .add_user_id("ou_1".to_string())
    );
    smoke_builder!(
        test_additional_informations_batch_delete_builder,
        v2::additional_informations::batch::delete::DeleteRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .add_user_id("ou_1".to_string())
    );
    smoke_builder!(
        test_indicator_query_builder,
        v2::indicator::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .name("能力".to_string())
            .page_size(10)
    );
    smoke_builder!(
        test_metric_detail_import_builder,
        v2::metric_detail::import::ImportRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .add_metric_detail("ou_1".to_string(), "metric_1".to_string(), 99.5)
    );
    smoke_builder!(
        test_metric_detail_query_builder,
        v2::metric_detail::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .user_id("ou_1".to_string())
        .page_size(20)
    );
    smoke_builder!(
        test_metric_field_query_builder,
        v2::metric_field::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "lib_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_metric_lib_query_builder,
        v2::metric_lib::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .name("销售".to_string())
            .page_size(20)
    );
    smoke_builder!(
        test_metric_tag_list_builder,
        v2::metric_tag::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_metric_template_query_builder,
        v2::metric_template::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "lib_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_question_query_builder,
        v2::question::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "tpl_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_review_data_v2_query_builder,
        v2::review_data::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .user_id("ou_1".to_string())
        .page_size(20)
    );
    smoke_builder!(
        test_review_template_query_builder,
        v2::review_template::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .name("季度模板".to_string())
            .page_size(20)
    );
    smoke_builder!(
        test_reviewee_query_builder,
        v2::reviewee::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_semester_list_builder,
        v1::semester::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .name("2026Q1".to_string())
            .page_size(20)
    );
    smoke_builder!(
        test_stage_task_find_by_page_builder,
        v1::stage_task::find_by_page::FindByPageRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_stage_task_find_by_user_list_builder,
        v1::stage_task::find_by_user_list::FindByUserListRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .add_user_id("ou_1".to_string())
    );
    smoke_builder!(
        test_user_group_user_rel_write_builder,
        v2::user_group_user_rel::write::WriteRequest::new(
            test_config("https://open.feishu.cn"),
            "group_1".to_string(),
        )
        .add_user("ou_1".to_string())
        .remove_user("ou_2".to_string())
    );
    smoke_builder!(
        test_user_info_query_builder,
        v2::user_info::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_review_data_v1_query_builder,
        v1::review_data::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "cycle_1".to_string(),
        )
        .page_size(20)
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_additional_information_import_cycle_id_validation(#[case] cycle_id: &str) {
        let result = v2::additional_information::import::ImportRequest::new(
            test_config("https://127.0.0.1:9"),
            cycle_id.to_string(),
        )
        .add_information("ou_1".to_string(), "补充说明".to_string())
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("cycle_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_metric_detail_import_cycle_id_validation(#[case] cycle_id: &str) {
        let result = v2::metric_detail::import::ImportRequest::new(
            test_config("https://127.0.0.1:9"),
            cycle_id.to_string(),
        )
        .add_metric_detail("ou_1".to_string(), "metric_1".to_string(), 95.0)
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("cycle_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_user_group_user_rel_write_validation(#[case] group_id: &str) {
        let result = v2::user_group_user_rel::write::WriteRequest::new(
            test_config("https://127.0.0.1:9"),
            group_id.to_string(),
        )
        .add_user("ou_1".to_string())
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("user_group_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_additional_informations_batch_delete_validation(#[case] cycle_id: &str) {
        let result = v2::additional_informations::batch::delete::DeleteRequest::new(
            test_config("https://127.0.0.1:9"),
            cycle_id.to_string(),
        )
        .add_user_id("ou_1".to_string())
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("cycle_id"));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_activity_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/activity/query"))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("name", "项目A"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"items":[{"id":"act_1","name":"项目A","status":1,"created_at":1735689600}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::activity::query::QueryRequest::new(test_config(&mock_server.uri()))
            .name("项目A".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();

        assert_eq!(resp.items[0].id, "act_1");
    }

    #[tokio::test]
    async fn test_additional_information_import_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/import",
            ))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "cycle_id":"cycle_1",
                "additional_informations":[{"user_id":"ou_1","content":"补充说明"}]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"success_count":1,"failed_count":0}
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::additional_information::import::ImportRequest::new(
            test_config(&mock_server.uri()),
            "cycle_1".to_string(),
        )
        .add_information("ou_1".to_string(), "补充说明".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.success_count, 1);
    }

    #[tokio::test]
    async fn test_metric_tag_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/performance/v2/metric_tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"items":[{"id":"tag_1","name":"增长","tag_type":1}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::metric_tag::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();

        assert_eq!(resp.items[0].name, "增长");
    }

    #[tokio::test]
    async fn test_stage_task_find_by_page_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v1/stage_tasks/find_by_page"))
            .and(query_param("cycle_id", "cycle_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"items":[{"user_id":"ou_1","stage_id":"stage_1","status":2}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::stage_task::find_by_page::FindByPageRequest::new(
            test_config(&mock_server.uri()),
            "cycle_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.items[0].stage_id, "stage_1");
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
        test_activity_query_response_serialization,
        v2::activity::query::QueryResponse,
        v2::activity::query::QueryResponse {
            items: vec![v2::activity::query::Activity {
                id: "act_1".to_string(),
                name: "项目A".to_string(),
                status: 1,
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_additional_information_import_response_serialization,
        v2::additional_information::import::ImportResponse,
        v2::additional_information::import::ImportResponse {
            success_count: 1,
            failed_count: 0,
        }
    );
    roundtrip_eq!(
        test_additional_information_query_response_serialization,
        v2::additional_information::query::QueryResponse,
        v2::additional_information::query::QueryResponse {
            items: vec![v2::additional_information::query::AdditionalInformation {
                user_id: "ou_1".to_string(),
                content: "补充说明".to_string(),
            }],
        }
    );
    roundtrip_eq!(
        test_additional_informations_batch_delete_response_serialization,
        v2::additional_informations::batch::delete::DeleteResponse,
        v2::additional_informations::batch::delete::DeleteResponse { success_count: 1 }
    );
    roundtrip_eq!(
        test_indicator_query_response_serialization,
        v2::indicator::query::QueryResponse,
        v2::indicator::query::QueryResponse {
            items: vec![v2::indicator::query::Indicator {
                id: "ind_1".to_string(),
                name: "执行力".to_string(),
                indicator_type: 1,
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_metric_detail_import_response_serialization,
        v2::metric_detail::import::ImportResponse,
        v2::metric_detail::import::ImportResponse {
            success_count: 1,
            failed_count: 0,
        }
    );
    roundtrip_eq!(
        test_metric_detail_query_response_serialization,
        v2::metric_detail::query::QueryResponse,
        v2::metric_detail::query::QueryResponse {
            items: vec![v2::metric_detail::query::MetricDetailResult {
                user_id: "ou_1".to_string(),
                metric_id: "metric_1".to_string(),
                value: 95.0,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_metric_field_query_response_serialization,
        v2::metric_field::query::QueryResponse,
        v2::metric_field::query::QueryResponse {
            items: vec![v2::metric_field::query::MetricField {
                id: "field_1".to_string(),
                name: "达成率".to_string(),
                field_type: 1,
                required: true,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_metric_lib_query_response_serialization,
        v2::metric_lib::query::QueryResponse,
        v2::metric_lib::query::QueryResponse {
            items: vec![v2::metric_lib::query::MetricLib {
                id: "lib_1".to_string(),
                name: "销售指标库".to_string(),
                description: Some("季度销售".to_string()),
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_metric_tag_list_response_serialization,
        v2::metric_tag::list::ListResponse,
        v2::metric_tag::list::ListResponse {
            items: vec![v2::metric_tag::list::MetricTag {
                id: "tag_1".to_string(),
                name: "增长".to_string(),
                tag_type: 1,
            }],
        }
    );
    roundtrip_eq!(
        test_metric_template_query_response_serialization,
        v2::metric_template::query::QueryResponse,
        v2::metric_template::query::QueryResponse {
            items: vec![v2::metric_template::query::MetricTemplate {
                id: "tpl_1".to_string(),
                name: "模板A".to_string(),
                metric_lib_id: "lib_1".to_string(),
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_question_query_response_serialization,
        v2::question::query::QueryResponse,
        v2::question::query::QueryResponse {
            items: vec![v2::question::query::Question {
                id: "q_1".to_string(),
                content: "请描述挑战".to_string(),
                question_type: 1,
                required: true,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_review_data_v2_query_response_serialization,
        v2::review_data::query::QueryResponse,
        v2::review_data::query::QueryResponse {
            items: vec![v2::review_data::query::ReviewData {
                user_id: "ou_1".to_string(),
                cycle_id: "cycle_1".to_string(),
                status: 2,
                score: Some(98.0),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_review_template_query_response_serialization,
        v2::review_template::query::QueryResponse,
        v2::review_template::query::QueryResponse {
            items: vec![v2::review_template::query::ReviewTemplate {
                id: "tpl_1".to_string(),
                name: "季度模板".to_string(),
                status: 1,
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_reviewee_query_response_serialization,
        v2::reviewee::query::QueryResponse,
        v2::reviewee::query::QueryResponse {
            items: vec![v2::reviewee::query::Reviewee {
                user_id: "ou_1".to_string(),
                name: "张三".to_string(),
                department_id: Some("od_1".to_string()),
                status: 1,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_semester_list_response_serialization,
        v1::semester::list::ListResponse,
        v1::semester::list::ListResponse {
            items: vec![v1::semester::list::Semester {
                id: "sem_1".to_string(),
                name: "2026Q1".to_string(),
                status: 2,
                start_time: 1735689600,
                end_time: 1743465600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_stage_task_find_by_page_response_serialization,
        v1::stage_task::find_by_page::FindByPageResponse,
        v1::stage_task::find_by_page::FindByPageResponse {
            items: vec![v1::stage_task::find_by_page::StageTask {
                user_id: "ou_1".to_string(),
                stage_id: "stage_1".to_string(),
                status: 2,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_stage_task_find_by_user_list_response_serialization,
        v1::stage_task::find_by_user_list::FindByUserListResponse,
        v1::stage_task::find_by_user_list::FindByUserListResponse {
            items: vec![v1::stage_task::find_by_user_list::StageTask {
                user_id: "ou_1".to_string(),
                stage_id: "stage_1".to_string(),
                status: 1,
            }],
        }
    );
    roundtrip_eq!(
        test_user_group_user_rel_write_response_serialization,
        v2::user_group_user_rel::write::WriteResponse,
        v2::user_group_user_rel::write::WriteResponse { success: true }
    );
    roundtrip_eq!(
        test_user_info_query_response_serialization,
        v2::user_info::query::QueryResponse,
        v2::user_info::query::QueryResponse {
            items: vec![v2::user_info::query::UserInfo {
                user_id: "ou_1".to_string(),
                name: "张三".to_string(),
                department_id: Some("od_1".to_string()),
                role: 1,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_review_data_v1_query_response_serialization,
        v1::review_data::query::QueryResponse,
        v1::review_data::query::QueryResponse {
            items: vec![v1::review_data::query::ReviewData {
                user_id: "ou_1".to_string(),
                cycle_id: "cycle_1".to_string(),
                status: 2,
                score: Some(95.0),
            }],
            has_more: false,
            page_token: None,
        }
    );
}
