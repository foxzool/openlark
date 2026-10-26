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
            .semester_ids(vec!["6992035450862224940".to_string()])
            .activity_ids(vec!["7343513161666707459".to_string()])
            .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_additional_information_import_builder,
        v2::additional_information::import::ImportRequest::new(
            test_config("https://open.feishu.cn"),
            "7348736302176534547".to_string(),
            "12454646".to_string(),
        )
        .add_information(v2::additional_information::import::AdditionalInformation {
            item_id: Some("7350194523185610771".to_string()),
            external_id: None,
            reviewee_user_id: "ou_1".to_string(),
            item: "业绩补充说明".to_string(),
            time: "2024-03-12".to_string(),
            detailed_description: "销售额增长目标超额完成".to_string(),
        })
        .import_record_name("人工导入".to_string())
    );
    smoke_builder!(
        test_additional_information_query_builder,
        v2::additional_information::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "7348736302176534547".to_string(),
        )
        .add_reviewee_user_id("ou_1".to_string())
        .page_size(20)
    );
    smoke_builder!(
        test_additional_informations_batch_delete_builder,
        v2::additional_informations::batch::delete::DeleteRequest::new(
            test_config("https://open.feishu.cn"),
            "7348736302176534547".to_string(),
        )
        .add_additional_information("7350195758357807123".to_string())
    );
    smoke_builder!(
        test_indicator_query_builder,
        v2::indicator::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .indicator_ids(vec!["7343513161666707459".to_string()])
            .page_size(10)
    );
    smoke_builder!(
        test_metric_detail_import_builder,
        v2::metric_detail::import::ImportRequest::new(
            test_config("https://open.feishu.cn"),
            "7293040702907514899".to_string(),
            "12454646".to_string(),
        )
        .import_record_name("API录入".to_string())
        .add_metric_detail(v2::metric_detail::import::ImportedMetric {
            reviewee_user_id: "ou_1".to_string(),
            metric_id: "metric_1".to_string(),
            fields: vec![v2::metric_detail::import::ImportedMetricField {
                field_id: "field_1".to_string(),
                field_value: Some("99.5".to_string()),
                field_value_person: None,
            }],
        })
    );
    smoke_builder!(
        test_metric_detail_query_builder,
        v2::metric_detail::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "6992035450862224940".to_string(),
        )
        .add_reviewee_user_id("ou_1".to_string())
        .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_metric_field_query_builder,
        v2::metric_field::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .field_ids(vec!["7293169069640630291".to_string()])
    );
    smoke_builder!(
        test_metric_lib_query_builder,
        v2::metric_lib::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .is_active(true)
            .tag_ids(vec!["7302271694582841364".to_string()])
            .page_size(20)
    );
    smoke_builder!(
        test_metric_tag_list_builder,
        v2::metric_tag::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_metric_template_query_builder,
        v2::metric_template::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .metrics_template_ids(vec!["7360956875099078676".to_string()])
            .status("to_be_configured".to_string())
            .page_size(20)
    );
    smoke_builder!(
        test_question_query_builder,
        v2::question::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .tag_based_question_ids(vec!["7343513161666707459".to_string()])
            .page_size(20)
    );
    smoke_builder!(
        test_review_data_v2_query_builder,
        v2::review_data::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .add_semester_id("6992035450862224940".to_string())
            .add_reviewee_user_id("ou_1".to_string())
            .stage_types(vec!["leader_review".to_string()])
    );
    smoke_builder!(
        test_review_template_query_builder,
        v2::review_template::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .review_template_ids(vec!["7343513161666707459".to_string()])
            .page_size(20)
    );
    smoke_builder!(
        test_reviewee_query_builder,
        v2::reviewee::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .semester_id("6992035450862224940".to_string())
            .user_ids(vec!["ou_3245842393d09e9428ad4655da6e30b3".to_string()])
            .page_size(20)
    );
    smoke_builder!(
        test_semester_list_builder,
        v1::semester::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .year(2024)
            .type_group("Quarter".to_string())
            .type_("Q1".to_string())
            .start_time("1625068800000".to_string())
            .end_time("1640966399999".to_string())
    );
    smoke_builder!(
        test_stage_task_find_by_page_builder,
        v1::stage_task::find_by_page::FindByPageRequest::new(
            test_config("https://open.feishu.cn"),
            "7033710017401751071".to_string(),
        )
        .task_option_lists(vec![1, 2, 3])
        .page_size(30)
    );
    smoke_builder!(
        test_stage_task_find_by_user_list_builder,
        v1::stage_task::find_by_user_list::FindByUserListRequest::new(
            test_config("https://open.feishu.cn"),
            "7033710017401751071".to_string(),
        )
        .add_user_id("ou_1".to_string())
        .task_option_lists(vec![1, 2, 3])
    );
    smoke_builder!(
        test_user_group_user_rel_write_builder,
        v2::user_group_user_rel::write::WriteRequest::new(
            test_config("https://open.feishu.cn"),
            "group_1".to_string(),
            1,
            "123456".to_string(),
        )
        .add_user("ou_1".to_string())
    );
    smoke_builder!(
        test_user_info_query_builder,
        v2::user_info::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .semester_id("6992035450862224940".to_string())
            .user_ids(vec!["ou_3245842393d09e9428ad4655da6e30b3".to_string()])
            .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_review_data_v1_query_builder,
        v1::review_data::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .start_time("1430425599999".to_string())
            .end_time("1630425599999".to_string())
            .add_stage_type("leader_review".to_string())
            .add_reviewee_user_id("ou_1".to_string())
            .updated_later_than("1630425599999".to_string())
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_additional_information_import_semester_id_validation(#[case] semester_id: &str) {
        let result = v2::additional_information::import::ImportRequest::new(
            test_config("https://127.0.0.1:9"),
            semester_id.to_string(),
            "12454646".to_string(),
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("semester_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_metric_detail_import_semester_id_validation(#[case] semester_id: &str) {
        let result = v2::metric_detail::import::ImportRequest::new(
            test_config("https://127.0.0.1:9"),
            semester_id.to_string(),
            "12454646".to_string(),
        )
        .add_metric_detail(v2::metric_detail::import::ImportedMetric {
            reviewee_user_id: "ou_1".to_string(),
            metric_id: "metric_1".to_string(),
            fields: vec![v2::metric_detail::import::ImportedMetricField {
                field_id: "field_1".to_string(),
                field_value: Some("95.0".to_string()),
                field_value_person: None,
            }],
        })
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("semester_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_user_group_user_rel_write_validation(#[case] group_id: &str) {
        let result = v2::user_group_user_rel::write::WriteRequest::new(
            test_config("https://127.0.0.1:9"),
            group_id.to_string(),
            1,
            "123456".to_string(),
        )
        .add_user("ou_1".to_string())
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("group_id"));
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_additional_informations_batch_delete_validation(#[case] semester_id: &str) {
        let result = v2::additional_informations::batch::delete::DeleteRequest::new(
            test_config("https://127.0.0.1:9"),
            semester_id.to_string(),
        )
        .add_additional_information("7350195758357807123".to_string())
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("semester_id"));
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
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "semester_ids": ["6992035450862224940"],
                "activity_ids": ["7343513161666707459"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"activities":[{"id":"7343513161666707459","name":{"zh_cn":"项目一","en_us":"Activity 1"},"description":{"zh_cn":"体验","en_us":"Interactive experience"},"semester_id":"7343513161666707459","mode":"performance_review","progress":"configurable","create_time":"1691951256000","modify_time":"1691951256000","create_user_id":"6924187793321444877","modify_user_id":"6924187793321444877"}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::activity::query::QueryRequest::new(test_config(&mock_server.uri()))
            .semester_ids(vec!["6992035450862224940".to_string()])
            .activity_ids(vec!["7343513161666707459".to_string()])
            .user_id_type("open_id".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();

        let activities = resp.activities.expect("activities 应存在");
        assert_eq!(activities[0].id.as_deref(), Some("7343513161666707459"));
        assert_eq!(
            activities[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("项目一")
        );
    }

    #[tokio::test]
    async fn test_additional_information_import_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/import",
            ))
            .and(header("Authorization", "Bearer test_token"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "semester_id":"7348736302176534547",
                "additional_informations":[{"item_id":"7350194523185610771","reviewee_user_id":"ou_1","item":"业绩补充说明","time":"2024-03-12","detailed_description":"销售额增长目标超额完成"}],
                "import_record_name":"人工导入"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"import_record_id":"7350194523185610771","additional_informations":[]}
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::additional_information::import::ImportRequest::new(
            test_config(&mock_server.uri()),
            "7348736302176534547".to_string(),
            "12454646".to_string(),
        )
        .add_information(v2::additional_information::import::AdditionalInformation {
            item_id: Some("7350194523185610771".to_string()),
            external_id: None,
            reviewee_user_id: "ou_1".to_string(),
            item: "业绩补充说明".to_string(),
            time: "2024-03-12".to_string(),
            detailed_description: "销售额增长目标超额完成".to_string(),
        })
        .import_record_name("人工导入".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.import_record_id, "7350194523185610771");
    }

    #[tokio::test]
    async fn test_metric_tag_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/performance/v2/metric_tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "items":[{
                        "tag_id":"7302271694582841364",
                        "tag_name":{"zh_cn":"体验","en_us":"Interactive experience"},
                        "index":100,
                        "create_time":"1678870702466",
                        "update_time":"1605529625257"
                    }],
                    "page_token":"eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more":true
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = v2::metric_tag::list::ListRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();

        assert_eq!(resp.items[0].tag_id.as_deref(), Some("7302271694582841364"));
        assert_eq!(
            resp.items[0]
                .tag_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("体验")
        );
        assert_eq!(resp.items[0].index, Some(100));
        assert_eq!(resp.has_more, Some(true));
        assert!(resp.page_token.is_some());
    }

    #[tokio::test]
    async fn test_stage_task_find_by_page_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v1/stage_tasks/find_by_page"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "semester_id": "7235911950407352321",
                "task_option_lists": [1, 2, 3],
                "page_size": 30
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "base":{
                        "end_time":"1748707140000",
                        "semester_id":"7235911950407352321",
                        "semester_name":{"en-US":"Demo Semester","zh-CN":"示例周期"},
                        "start_time":"1684684800000"
                    },
                    "items":[{
                        "user_id":"bega29ca",
                        "stage_num_lists":[{"task_option_id":1,"stage_num":1}],
                        "stage_task_info_lists":[{
                            "stage_id":"7270346432875692033",
                            "name":{"en-US":"confirm_invitation","zh-CN":"360° 确认"},
                            "deadline":"1717142244000",
                            "need_todo_count":10,
                            "jump_url":"https://performance.feishu-pre.cn/perf/review",
                            "stage_task_status":"need_todo",
                            "task_option_id":1
                        }]
                    }],
                    "has_more":true,
                    "page_token":"84I9nKzkGJjwv1FrJ1sIveEE2ij7xnes6nhcmxNOXLaLLe52sFVu8S0NPxRFkMhLFCMutL9a6Eod7kzOqWQeu47R7NWiZEmBKJ93Asg5VZv1oJkrG318reCJlrljfaeR"
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::stage_task::find_by_page::FindByPageRequest::new(
            test_config(&mock_server.uri()),
            "7235911950407352321".to_string(),
        )
        .task_option_lists(vec![1, 2, 3])
        .page_size(30)
        .user_id_type("open_id".to_string())
        .execute_with_options(auth_option())
        .await
        .unwrap();

        let base = resp.base.as_ref().expect("base 应存在");
        assert_eq!(base.semester_id.as_deref(), Some("7235911950407352321"));
        let items = resp.items.as_ref().expect("items 应存在");
        assert_eq!(items[0].user_id.as_deref(), Some("bega29ca"));
        let infos = items[0]
            .stage_task_info_lists
            .as_ref()
            .expect("stage_task_info_lists 应存在");
        assert_eq!(infos[0].stage_id.as_deref(), Some("7270346432875692033"));
        assert_eq!(infos[0].stage_task_status.as_deref(), Some("need_todo"));
        assert_eq!(resp.has_more, Some(true));
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
            activities: Some(vec![v2::activity::query::Activity {
                id: Some("7343513161666707459".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("项目一".to_string()),
                    en_us: Some("Activity 1".to_string()),
                    extra: Default::default(),
                }),
                description: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                semester_id: Some("7343513161666707459".to_string()),
                mode: Some("performance_review".to_string()),
                progress: Some("configurable".to_string()),
                create_time: Some("1691951256000".to_string()),
                modify_time: Some("1691951256000".to_string()),
                create_user_id: Some("6924187793321444877".to_string()),
                modify_user_id: Some("6924187793321444877".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_additional_information_import_response_serialization,
        v2::additional_information::import::ImportResponse,
        v2::additional_information::import::ImportResponse {
            import_record_id: "7350194523185610771".to_string(),
            additional_informations: vec![
                v2::additional_information::import::AdditionalInformation {
                    item_id: Some("7350195758357807123".to_string()),
                    external_id: Some("6789523104723558912".to_string()),
                    reviewee_user_id: "ou_1".to_string(),
                    item: "业绩补充说明".to_string(),
                    time: "2024-03-12".to_string(),
                    detailed_description: "补充说明".to_string(),
                }
            ],
        }
    );
    roundtrip_eq!(
        test_additional_information_query_response_serialization,
        v2::additional_information::query::QueryResponse,
        v2::additional_information::query::QueryResponse {
            additional_informations: vec![
                v2::additional_information::query::AdditionalInformation {
                    item_id: Some("7350195758357807123".to_string()),
                    external_id: Some("6789523104723558912".to_string()),
                    reviewee_user_id: Some("ou_1".to_string()),
                    item: Some("业绩补充说明".to_string()),
                    time: Some("2024-03-12".to_string()),
                    detailed_description: Some("补充说明".to_string()),
                }
            ],
            has_more: Some(false),
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_additional_informations_batch_delete_response_serialization,
        v2::additional_informations::batch::delete::DeleteResponse,
        v2::additional_informations::batch::delete::DeleteResponse {
            additional_informations: vec!["7350195758357807123".to_string()],
        }
    );
    roundtrip_eq!(
        test_indicator_query_response_serialization,
        v2::indicator::query::QueryResponse,
        v2::indicator::query::QueryResponse {
            indicators: Some(vec![v2::indicator::query::Indicator {
                id: Some("7343513161666707459".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("完成度".to_string()),
                    en_us: Some("Completeness".to_string()),
                    extra: Default::default(),
                }),
                r#type: Some("plus".to_string()),
                options: Some(vec![v2::indicator::query::IndicatorOption {
                    id: Some("7343513161666707459".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("等级 1".to_string()),
                        en_us: Some("Rank 1".to_string()),
                        extra: Default::default(),
                    }),
                    lable: Some("qwerty".to_string()),
                }]),
            }]),
            has_more: Some(true),
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
        }
    );
    roundtrip_eq!(
        test_metric_detail_import_response_serialization,
        v2::metric_detail::import::ImportResponse,
        v2::metric_detail::import::ImportResponse {
            import_record_id: "7241404194141224979".to_string(),
        }
    );
    roundtrip_eq!(
        test_metric_detail_query_response_serialization,
        v2::metric_detail::query::QueryResponse,
        v2::metric_detail::query::QueryResponse {
            semester_id: Some("7291278856547794964".to_string()),
            reviewee_metrics: Some(vec![v2::metric_detail::query::RevieweeMetric {
                reviewee_user_id: Some(openlark_hr::performance::performance::v2::models::UserId {
                    open_id: Some("od-asd2dasdasd".to_string()),
                    user_id: Some("ou-ux987dsf6x".to_string()),
                }),
                metric_template_id: Some("7294570803306168339".to_string()),
                metric_details: Some(v2::metric_detail::query::MetricDetail {
                    metric_id: Some("7272581996315099155".to_string()),
                    name: Some("示例指标".to_string()),
                    fields: Some(vec![v2::metric_detail::query::MetricFieldInDetail {
                        field_id: Some("7296082067266207764".to_string()),
                        field_value: Some("90".to_string()),
                        field_value_person: Some(
                            openlark_hr::performance::performance::v2::models::UserId {
                                open_id: Some("od-asd2dasdasd".to_string()),
                                user_id: Some("ou-ux987dsf6x".to_string()),
                            },
                        ),
                    }]),
                    dimension_id: Some("7303895818346430484".to_string()),
                    dimension_name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("维度一".to_string()),
                        en_us: Some("Dimension 1".to_string()),
                        extra: Default::default(),
                    }),
                    dimension_weight: Some("90%".to_string()),
                    add_from: Some("admin".to_string()),
                    is_from_library: Some(true),
                }),
                reviewee_stage_statuses: Some(vec![
                    v2::metric_detail::query::RevieweeStageStatus {
                        stage_id: Some("1".to_string()),
                        stage_type: Some("kpi_metric_setting".to_string()),
                        stage_status: Some(0),
                    },
                ]),
            }]),
        }
    );
    roundtrip_eq!(
        test_metric_field_query_response_serialization,
        v2::metric_field::query::QueryResponse,
        v2::metric_field::query::QueryResponse {
            items: Some(vec![v2::metric_field::query::MetricField {
                field_id: Some("7272581996315099155".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("销量".to_string()),
                    en_us: Some("Sales".to_string()),
                    extra: Default::default(),
                }),
                r#type: Some("text".to_string()),
            }]),
        }
    );
    roundtrip_eq!(
        test_metric_lib_query_response_serialization,
        v2::metric_lib::query::QueryResponse,
        v2::metric_lib::query::QueryResponse {
            items: Some(vec![v2::metric_lib::query::MetricLib {
                metric_id: Some("7272581996315099155".to_string()),
                name: Some("销售额".to_string()),
                type_id: Some("7272578300650717203".to_string()),
                tags: Some(vec![v2::metric_lib::query::MetricTagInfo {
                    tag_id: Some("7302271694582841364".to_string()),
                    tag_name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("体验".to_string()),
                        en_us: Some("Interactive experience".to_string()),
                        extra: Default::default(),
                    }),
                }]),
                fields: Some(vec![v2::metric_lib::query::MetricFieldInLibrary {
                    field_id: Some("7296701873237786643".to_string()),
                    input_setting: Some("admin".to_string()),
                    field_value: Some("90".to_string()),
                    field_value_person: Some(
                        openlark_hr::performance::performance::v2::models::UserId {
                            open_id: Some("od-asd2dasdasd".to_string()),
                            user_id: Some("ou-ux987dsf6x".to_string()),
                        },
                    ),
                }]),
                scoring_setting_type: Some("score_by_formula".to_string()),
                scoring_formula: Some(v2::metric_lib::query::ScoringFormula {
                    formula_id: Some("7302271694582841364".to_string()),
                    formula_name: Some("示例公式".to_string()),
                    formula_details: Some("1 + 1".to_string()),
                }),
                data_source_inputters: Some(vec![
                    openlark_hr::performance::performance::v2::models::UserId {
                        open_id: Some("od-asd2dasdasd".to_string()),
                        user_id: Some("ou-ux987dsf6x".to_string()),
                    },
                ]),
                range_of_availability: Some("admins_and_reviewees".to_string()),
                is_active: Some(true),
            }]),
            has_more: Some(true),
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
        }
    );
    roundtrip_eq!(
        test_metric_tag_list_response_serialization,
        v2::metric_tag::list::ListResponse,
        v2::metric_tag::list::ListResponse {
            items: vec![v2::metric_tag::list::MetricTag {
                tag_id: Some("7302271694582841364".to_string()),
                tag_name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                index: Some(100),
                create_time: Some("1678870702466".to_string()),
                update_time: Some("1605529625257".to_string()),
            }],
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
            has_more: Some(true),
        }
    );
    roundtrip_eq!(
        test_metric_template_query_response_serialization,
        v2::metric_template::query::QueryResponse,
        v2::metric_template::query::QueryResponse {
            items: Some(vec![v2::metric_template::query::MetricTemplate {
                id: Some("7296488199415660563".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                description: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                status: Some("to_be_configured".to_string()),
                is_set_by_group: Some(true),
                total_metric_score_method: Some("weight".to_string()),
                metric_weight_method: Some(
                    "sum_of_metric_weights_for_each_dimension_equals_100".to_string(),
                ),
                metric_dimensions: Some(vec![v2::metric_template::query::MetricDimension {
                    group_id: Some("7296753366268215316".to_string()),
                    metric_dimension_id: Some("7296753366268215316".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("体验".to_string()),
                        en_us: Some("Interactive experience".to_string()),
                        extra: Default::default(),
                    }),
                    evaluation_rule_id_for_each_metric: Some("7296701873237786643".to_string(),),
                    dimension_weight: Some("90".to_string()),
                    description: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("维度描述".to_string()),
                        en_us: Some("Dimension description".to_string()),
                        extra: Default::default(),
                    }),
                    review_rule_option: Some(0),
                    custom_metric_config: Some(v2::metric_template::query::CustomMetricConfig {
                        default_formula_id: Some("7296701873237786643".to_string()),
                        least_metrics_size: Some(1),
                        add_metric_options: Some(vec![1, 2]),
                    },),
                },]),
                metrics: Some(vec![v2::metric_template::query::MetricInTemplate {
                    group_id: Some("7272581996315099155".to_string()),
                    metric_id: Some("7272581996315099155".to_string()),
                    name: Some("销售额".to_string()),
                    type_id: Some("7272581996315099155".to_string()),
                    fields: Some(vec![v2::metric_template::query::MetricFieldInTemplate {
                        id: Some("7272581996315099155".to_string()),
                        input_setting: Some("admin".to_string()),
                        filed_value: Some("100".to_string()),
                        field_value_person: Some(
                            openlark_hr::performance::performance::v2::models::UserId {
                                open_id: Some("od-asd2dasdasd".to_string()),
                                user_id: Some("ou-ux987dsf6x".to_string()),
                            },
                        ),
                    }]),
                    is_from_library: Some(true),
                    scoring_setting_type: Some("score_by_formula".to_string()),
                    data_source_inputters: Some(vec![
                        openlark_hr::performance::performance::v2::models::UserId {
                            open_id: Some("od-asd2dasdasd".to_string()),
                            user_id: Some("ou-ux987dsf6x".to_string()),
                        },
                    ]),
                    metric_dimension_id: Some("7272581996315099155".to_string()),
                    review_rule_config: Some(v2::metric_template::query::MetricReviewRuleConfig {
                        max: Some("10".to_string()),
                        min: Some("0".to_string()),
                    },),
                }]),
                groups: Some(vec![v2::metric_template::query::MetricGroup {
                    group_id: Some("7296466589195436051".to_string()),
                    name: Some("分组 1".to_string()),
                }]),
            }]),
            has_more: Some(true),
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
        }
    );
    roundtrip_eq!(
        test_question_query_response_serialization,
        v2::question::query::QueryResponse,
        v2::question::query::QueryResponse {
            tag_based_questions: Some(vec![v2::question::query::TagBasedQuestion {
                question_id: Some("7343513161666707459".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                tag_items: Some(vec![v2::question::query::TagItem {
                    id: Some("7343513161666707459".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("体验".to_string()),
                        en_us: Some("Interactive experience".to_string()),
                        extra: Default::default(),
                    }),
                }]),
            }]),
            has_more: Some(true),
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
        }
    );
    roundtrip_eq!(
        test_review_data_v2_query_response_serialization,
        v2::review_data::query::QueryResponse,
        v2::review_data::query::QueryResponse {
            datas: Some(vec![v2::review_data::query::ReviewProfile {
                user_id: Some(openlark_hr::performance::performance::v2::models::UserId {
                    open_id: Some("od-asd2dasdasd".to_string()),
                    user_id: Some("ou-ux987dsf6x".to_string()),
                }),
                semester_id: Some("7343513161666707459".to_string()),
                activity_id: Some("7343513161666707459".to_string()),
                review_template_id: Some("7343513161666707459".to_string()),
                stages: Some(vec![v2::review_data::query::ReviewStage {
                    stage_id: Some("7343513161666707459".to_string()),
                    stage_type: Some("review".to_string()),
                    template_id: Some("7343513161666707459".to_string()),
                    records: Some(vec![v2::review_data::query::ReviewRecord {
                        progress: Some(1),
                        units: Some(vec![v2::review_data::query::ReviewUnit {
                            unit_id: Some("7343513161666707459".to_string()),
                            is_unknown: Some(false),
                            data: Some(vec![v2::review_data::query::ReviewDetail {
                                field_id: Some("7343513161666707459".to_string()),
                                reviewer_user_id: Some(
                                    openlark_hr::performance::performance::v2::models::UserId {
                                        open_id: Some("od-asd2dasdasd".to_string()),
                                        user_id: Some("ou-ux987dsf6x".to_string()),
                                    },
                                ),
                                submit_time: Some("7343513161666707459".to_string()),
                                indicator_id: Some("7343513161666707459".to_string()),
                                option_id: Some("7343513161666707459".to_string()),
                                score: Some("1.1".to_string()),
                                text: Some("qwert".to_string()),
                                tag_based_question_id: Some("7343513161666707459".to_string()),
                                tag_text_item_data: Some(vec![
                                    v2::review_data::query::TagText {
                                        tag_text_id: Some("7343513161666707459".to_string()),
                                        tag_text: Some("qwertyu".to_string()),
                                        tag_richtext: None,
                                    },
                                ]),
                                perf_coefficient_value: Some("1.1".to_string()),
                                sub_indicator_data: Some(vec![
                                    v2::review_data::query::SubIndicator {
                                        field_id: Some("7343513161666707459".to_string()),
                                        indicator_id: Some("7343513161666707459".to_string()),
                                        option_id: Some("7343513161666707459".to_string()),
                                        score: Some("1.1".to_string()),
                                        text: Some("qwertyuiop".to_string()),
                                        richtext: None,
                                    },
                                ]),
                                objective_data: Some(vec![
                                    v2::review_data::query::ObjectiveData {
                                        objective_id: Some("7343513161666707459".to_string()),
                                        score: Some("1.1".to_string()),
                                        text: Some("qwertyu".to_string()),
                                        keyresult_data: Some(vec![
                                            v2::review_data::query::KeyresultData {
                                                keyresult_id: Some(
                                                    "7343513161666707459".to_string(),
                                                ),
                                                score: Some("1.1".to_string()),
                                                text: Some("qwerty".to_string()),
                                                richtext: None,
                                            },
                                        ]),
                                        richtext: None,
                                    },
                                ]),
                                metric_data: Some(vec![v2::review_data::query::MetricData {
                                    id: Some("7343513161666707459".to_string()),
                                    score: Some("1.1".to_string()),
                                }]),
                                leader_review_data_source: Some("review".to_string()),
                                multi_texts: Some(vec!["qwerty".to_string()]),
                                richtext: None,
                                multi_richtexts: None,
                                is_principal_review_item: Some(true),
                                invited_review_record_info: Some(
                                    v2::review_data::query::InvitedReviewRecordInfo {
                                        reviewer_id: Some(
                                            openlark_hr::performance::performance::v2::models::UserId {
                                                open_id: Some("od-asd2dasdasd".to_string()),
                                                user_id: Some("ou-ux987dsf6x".to_string()),
                                            },
                                        ),
                                        is_rejected: Some(false),
                                        rejected_reason: Some("test".to_string()),
                                        distribute_type: Some(1),
                                        avg_diff: Some("1.23".to_string()),
                                        relationship_with_reviewee: Some(
                                            "direct_report".to_string(),
                                        ),
                                        invitedby: Some("system_default".to_string()),
                                    },
                                ),
                                direct_project_leader_record_info: Some(
                                    v2::review_data::query::DirectProjectLeaderRecordInfo {
                                        reviewer_id: Some(
                                            openlark_hr::performance::performance::v2::models::UserId {
                                                open_id: Some("od-asd2dasdasd".to_string()),
                                                user_id: Some("ou-ux987dsf6x".to_string()),
                                            },
                                        ),
                                        cooperation_projects: Some(vec![
                                            v2::review_data::query::CooperationProject {
                                                id: Some("7309457114076807188".to_string()),
                                                name: Some(
                                                    openlark_hr::common::shared_models::I18nText {
                                                        zh_cn: Some("体验".to_string()),
                                                        en_us: Some(
                                                            "Interactive experience".to_string(),
                                                        ),
                                                        extra: Default::default(),
                                                    },
                                                ),
                                                roles: Some(vec![
                                                    v2::review_data::query::CooperationRole {
                                                        reviewer_role: Some(
                                                            v2::review_data::query::CooperationUserRole {
                                                                role_id: Some(
                                                                    "7213434603057807379".to_string(),
                                                                ),
                                                                name: Some(
                                                                    openlark_hr::common::shared_models::I18nText {
                                                                        zh_cn: Some("体验".to_string()),
                                                                        en_us: Some("Interactive experience".to_string()),
                                                                        extra: Default::default(),
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                        reviewee_role: Some(
                                                            v2::review_data::query::CooperationUserRole {
                                                                role_id: Some(
                                                                    "7213434603057807379".to_string(),
                                                                ),
                                                                name: Some(
                                                                    openlark_hr::common::shared_models::I18nText {
                                                                        zh_cn: Some("体验".to_string()),
                                                                        en_us: Some("Interactive experience".to_string()),
                                                                        extra: Default::default(),
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                ]),
                                                user_roles: Some(vec![
                                                    v2::review_data::query::CooperationUserRole {
                                                        role_id: Some(
                                                            "7213434603057807379".to_string(),
                                                        ),
                                                        name: Some(
                                                            openlark_hr::common::shared_models::I18nText {
                                                                zh_cn: Some("体验".to_string()),
                                                                en_us: Some("Interactive experience".to_string()),
                                                                extra: Default::default(),
                                                            },
                                                        ),
                                                    },
                                                ]),
                                                underling_roles: Some(vec![
                                                    v2::review_data::query::CooperationUserRole {
                                                        role_id: Some(
                                                            "7213434603057807379".to_string(),
                                                        ),
                                                        name: Some(
                                                            openlark_hr::common::shared_models::I18nText {
                                                                zh_cn: Some("体验".to_string()),
                                                                en_us: Some("Interactive experience".to_string()),
                                                                extra: Default::default(),
                                                            },
                                                        ),
                                                    },
                                                ]),
                                            },
                                        ]),
                                        review_depend_projects: None,
                                        participated_projects: None,
                                    },
                                ),
                            }]),
                        }]),
                        invited_review_record_info: None,
                        direct_project_leader_record_info: None,
                        record_id: Some("7385000219907457024-7385000219907457025".to_string()),
                    }]),
                    review_stage_role: Some("reviewee".to_string()),
                }]),
            }]),
        }
    );
    roundtrip_eq!(
        test_review_template_query_response_serialization,
        v2::review_template::query::QueryResponse,
        v2::review_template::query::QueryResponse {
            review_templates: Some(vec![v2::review_template::query::ReviewTemplate {
                review_template_id: Some("7343513161666723843".to_string()),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                description: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("体验".to_string()),
                    en_us: Some("Interactive experience".to_string()),
                    extra: Default::default(),
                }),
                status: Some("enabled".to_string()),
                templates: Some(vec![v2::review_template::query::TemplateItem {
                    template_id: Some("7343513161666707459".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("体验".to_string()),
                        en_us: Some("Interactive experience".to_string()),
                        extra: Default::default(),
                    }),
                    stage_type: Some("leader_review".to_string()),
                    review_stage_role: Some("reviewee".to_string()),
                    review_stage_data_write_mode: Some("cooperate,independent".to_string()),
                }]),
                units: Some(vec![v2::review_template::query::ReviewUnit {
                    unit_id: Some("7343513161666723843".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("体验".to_string()),
                        en_us: Some("Interactive experience".to_string()),
                        extra: Default::default(),
                    }),
                    fields: Some(vec![v2::review_template::query::UnitField {
                        field_id: Some("7343513161666707459".to_string()),
                        name: Some(openlark_hr::common::shared_models::I18nText {
                            zh_cn: Some("体验".to_string()),
                            en_us: Some("Interactive experience".to_string()),
                            extra: Default::default(),
                        }),
                        indicator_id: Some("7343513161666707459".to_string()),
                        tag_based_question_id: Some("7343513161666707459".to_string()),
                        objective_text_qustion_title: Some(
                            openlark_hr::common::shared_models::I18nText {
                                zh_cn: Some("体验".to_string()),
                                en_us: Some("Interactive experience".to_string()),
                                extra: Default::default(),
                            },
                        ),
                        keyresult_text_qustion_title: Some(
                            openlark_hr::common::shared_models::I18nText {
                                zh_cn: Some("体验".to_string()),
                                en_us: Some("Interactive experience".to_string()),
                                extra: Default::default(),
                            },
                        ),
                        parent_field_id: Some("7343513161666707459".to_string()),
                        kpi_template_id: Some("7494252079230222371".to_string()),
                    }]),
                }]),
            }]),
            has_more: Some(true),
            page_token: Some(
                "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=".to_string()
            ),
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
            items: Some(vec![v1::semester::list::Semester {
                id: Some("6992035450862224940".to_string()),
                year: Some(2024),
                type_group: Some("Quarter".to_string()),
                r#type: Some("Q1".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("2024 全年绩效周期".to_string()),
                        en_us: Some("2024 full year semester".to_string()),
                    }
                ),
                progress: Some("enabled".to_string()),
                start_time: Some("1625068800000".to_string()),
                end_time: Some("1640966399999".to_string()),
                create_time: Some("1625068800000".to_string()),
                modify_time: Some("1625068800000".to_string()),
                create_user_id: Some("ou_ce613028fe74745421f5dc320bb9c709".to_string()),
                modify_user_id: Some("ou_ce613028fe74745421f5dc320bb9c709".to_string()),
            }]),
            has_more: None,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_stage_task_find_by_page_response_serialization,
        v1::stage_task::find_by_page::FindByPageResponse,
        v1::stage_task::find_by_page::FindByPageResponse {
            base: Some(v1::stage_task::models::SemesterBaseInfo {
                semester_id: Some("7235911950407352321".to_string()),
                semester_name: Some(openlark_hr::performance::performance::v1::models::I18nName {
                    zh_cn: Some("示例周期".to_string()),
                    en_us: Some("Demo Semester".to_string()),
                }),
                start_time: Some("1684684800000".to_string()),
                end_time: Some("1748707140000".to_string()),
            }),
            items: Some(vec![v1::stage_task::models::StageTask {
                user_id: Some("bega29ca".to_string()),
                stage_num_lists: Some(vec![v1::stage_task::models::StageTaskStatusNum {
                    task_option_id: Some(1),
                    stage_num: Some(1),
                }]),
                stage_task_info_lists: Some(vec![v1::stage_task::models::StageTaskInfo {
                    stage_id: Some("7270346432875692033".to_string()),
                    name: Some(openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("360° 确认".to_string()),
                        en_us: Some("confirm_invitation".to_string()),
                    }),
                    deadline: Some("1717142244000".to_string()),
                    need_todo_count: Some(10),
                    jump_url: Some("https://performance.feishu-pre.cn/perf/review".to_string()),
                    stage_task_status: Some("need_todo".to_string()),
                    task_option_id: Some(1),
                }]),
            }]),
            has_more: Some(true),
            page_token: Some("84I9nKzkGJjwv1FrJ1sIveEE2ij7xnes6nhcmxNOXLaLLe52sFVu8S0NPxRFkMhLFCMutL9a6Eod7kzOqWQeu47R7NWiZEmBKJ93Asg5VZv1oJkrG318reCJlrljfaeR".to_string()),
        }
    );
    roundtrip_eq!(
        test_stage_task_find_by_user_list_response_serialization,
        v1::stage_task::find_by_user_list::FindByUserListResponse,
        v1::stage_task::find_by_user_list::FindByUserListResponse {
            base: Some(v1::stage_task::models::SemesterBaseInfo {
                semester_id: Some("7235911950407352321".to_string()),
                semester_name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("示例周期".to_string()),
                        en_us: Some("Demo Semester".to_string()),
                    }
                ),
                start_time: Some("1684684800000".to_string()),
                end_time: Some("1748707140000".to_string()),
            }),
            items: Some(vec![v1::stage_task::models::StageTask {
                user_id: Some("bega29ca".to_string()),
                stage_num_lists: Some(vec![v1::stage_task::models::StageTaskStatusNum {
                    task_option_id: Some(1),
                    stage_num: Some(1),
                }]),
                stage_task_info_lists: Some(vec![v1::stage_task::models::StageTaskInfo {
                    stage_id: Some("7270346432875692033".to_string()),
                    name: Some(
                        openlark_hr::performance::performance::v1::models::I18nName {
                            zh_cn: Some("360° 确认".to_string()),
                            en_us: Some("confirm_invitation".to_string()),
                        }
                    ),
                    deadline: Some("1717142244000".to_string()),
                    need_todo_count: Some(10),
                    jump_url: Some("https://performance.feishu-pre.cn/perf/review".to_string()),
                    stage_task_status: Some("need_todo".to_string()),
                    task_option_id: Some(1),
                }]),
            }]),
        }
    );
    roundtrip_eq!(
        test_user_group_user_rel_write_response_serialization,
        v2::user_group_user_rel::write::WriteResponse,
        v2::user_group_user_rel::write::WriteResponse {
            data: v2::user_group_user_rel::write::WriteUserGroupScopeData {
                success_user_ids: vec!["ou_1".to_string()],
                fail_user_datas: vec![
                    v2::user_group_user_rel::write::WriteUserGroupScopeFailUserData {
                        user_id: "ou_2".to_string(),
                        fail_code: 1,
                    }
                ],
            },
        }
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
            semesters: Some(vec![v1::review_data::query::Semester {
                id: Some("6992035450862224940".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("sr".to_string()),
                        en_us: None,
                    }
                ),
                start_time: Some("1625068800000".to_string()),
                end_time: Some("1640966399999".to_string()),
                ..Default::default()
            }]),
            activities: Some(vec![v1::review_data::query::Activity {
                id: Some("6992035450862323244".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("rino".to_string()),
                        en_us: None,
                    }
                ),
                semester_id: Some("6992035450862224940".to_string()),
            }]),
            indicators: Some(vec![v1::review_data::query::Indicator {
                id: Some("6966127279593784876".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("业绩".to_string()),
                        en_us: None,
                    }
                ),
                options: Some(vec![v1::review_data::query::IndicatorOption {
                    id: Some("6966127279593653804".to_string()),
                    name: Some(
                        openlark_hr::performance::performance::v1::models::I18nName {
                            zh_cn: Some("符合预期".to_string()),
                            en_us: None,
                        }
                    ),
                    label: Some("B".to_string()),
                }]),
            }]),
            templates: Some(vec![v1::review_data::query::Template {
                id: Some("6982759008789366313".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("上级评估".to_string()),
                        en_us: Some("Manager Review".to_string()),
                    }
                ),
                stage: Some("leader_review".to_string()),
            }]),
            units: Some(vec![v1::review_data::query::Unit {
                id: Some("6982759009396508196".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("业绩".to_string()),
                        en_us: None,
                    }
                ),
            }]),
            fields: Some(vec![v1::review_data::query::Field {
                id: Some("6982759006887888417".to_string()),
                name: Some(
                    openlark_hr::performance::performance::v1::models::I18nName {
                        zh_cn: Some("业绩".to_string()),
                        en_us: None,
                    }
                ),
                indicator_id: None,
                parent_field_id: None,
            }]),
            datas: Some(vec![v1::review_data::query::ReviewProfile {
                user_id: Some(openlark_hr::performance::performance::v1::models::UserId {
                    open_id: Some("ou_3245842393d09e9428ad4655da6e30b3".to_string()),
                    user_id: Some("6932009399175086099".to_string()),
                }),
                semester_id: Some("6992035450862224940".to_string()),
                activity_id: Some("6992035450862323244".to_string()),
                stages: Some(vec![v1::review_data::query::ReviewStage {
                    stage_type: Some("leader_review".to_string()),
                    progress: Some(3),
                    data: Some(vec![v1::review_data::query::ReviewDetail {
                        template_id: Some("6982759008789366313".to_string()),
                        unit_id: Some("6982759009396508196".to_string()),
                        field_id: Some("6982759006887888417".to_string()),
                        reviewer_user_id: Some(
                            openlark_hr::performance::performance::v1::models::UserId {
                                open_id: Some("ou_f0ff33850fc47236e34360d5eed99405".to_string()),
                                user_id: Some("6924863940618878996".to_string()),
                            },
                        ),
                        submit_time: Some("1627977100000".to_string()),
                        indicator_id: Some("6966127279593784876".to_string()),
                        option_id: Some("6966127279593653804".to_string()),
                        score: None,
                        text: None,
                        perf_coefficient_result: None,
                        richtext: None,
                    }]),
                }]),
            }]),
        }
    );
}
