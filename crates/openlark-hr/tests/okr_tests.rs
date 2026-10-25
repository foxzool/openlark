//! okr tests 集成测试。
#![cfg(feature = "okr")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::okr::okr::v1;
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
        test_image_upload_builder,
        v1::image::upload::UploadRequest::new(
            test_config("https://open.feishu.cn"),
            "6974586812998174252".to_string(),
            2,
            "progress.png".to_string(),
            vec![1u8, 2, 3],
        )
    );
    smoke_builder!(
        test_okr_batch_get_builder,
        v1::okr::batch_get::BatchGetRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["okr_1".to_string(), "okr_2".to_string()],
        )
        .lang("zh_cn".to_string())
    );
    smoke_builder!(
        test_period_create_builder,
        v1::period::create::CreateRequest::new(
            test_config("https://open.feishu.cn"),
            "6969864184272078374".to_string(),
            "2026-01".to_string(),
        )
    );
    smoke_builder!(
        test_period_list_builder,
        v1::period::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .page_token("next".to_string())
    );
    smoke_builder!(
        test_period_patch_builder,
        v1::period::patch::PatchRequest::new(
            test_config("https://open.feishu.cn"),
            "period_1".to_string(),
            2,
        )
    );
    smoke_builder!(
        test_period_rule_list_builder,
        v1::period_rule::list::ListRequest::new(test_config("https://open.feishu.cn"))
            .period_id("period_1".to_string())
    );
    smoke_builder!(
        test_progress_record_create_builder,
        v1::progress_record::create::CreateRequest::new(
            test_config("https://open.feishu.cn"),
            "周报系统".to_string(),
            "https://www.zhoubao.com".to_string(),
            "7041430377642082323".to_string(),
            2,
            v1::progress_record::create::ContentBlock {
                blocks: vec![v1::progress_record::create::ContentBlockElement {
                    block_element_type: Some("paragraph".to_string()),
                    paragraph: Some(v1::progress_record::create::ContentParagraph {
                        style: None,
                        elements: vec![v1::progress_record::create::ContentParagraphElement {
                            paragraph_element_type: Some("textRun".to_string()),
                            text_run: Some(v1::progress_record::create::ContentTextRun {
                                text: Some("进展更新".to_string()),
                                style: None,
                            }),
                            docs_link: None,
                            person: None,
                        }],
                    }),
                    gallery: None,
                }],
            },
        )
        .progress_rate(v1::progress_record::create::ProgressRateNew {
            percent: Some(60.0),
            status: Some(0),
        })
    );
    smoke_builder!(
        test_progress_record_get_builder,
        v1::progress_record::get::GetRequest::new(
            test_config("https://open.feishu.cn"),
            "progress_1".to_string(),
        )
    );
    smoke_builder!(
        test_progress_record_delete_builder,
        v1::progress_record::delete::DeleteRequest::new(
            test_config("https://open.feishu.cn"),
            "progress_1".to_string(),
        )
    );
    smoke_builder!(
        test_progress_record_update_builder,
        v1::progress_record::update::UpdateRequest::new(
            test_config("https://open.feishu.cn"),
            "progress_1".to_string(),
            v1::progress_record::create::ContentBlock { blocks: Vec::new() },
        )
        .progress_rate(v1::progress_record::create::ProgressRateNew {
            percent: Some(70.0),
            status: None,
        })
    );
    smoke_builder!(
        test_review_query_builder,
        v1::review::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["ou_1".to_string()],
            vec!["period_1".to_string()],
        )
        .user_id_type("open_id".to_string())
    );
    smoke_builder!(
        test_user_okr_list_builder,
        v1::user::okr::list::ListRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
            0,
            10,
        )
        .user_id_type("open_id".to_string())
        .lang("zh_cn".to_string())
        .period_ids(vec!["period_1".to_string()])
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_period_create_name_validation(#[case] period_rule_id: &str) {
        let result = v1::period::create::CreateRequest::new(
            test_config("https://127.0.0.1:9"),
            period_rule_id.to_string(),
            "2026-01".to_string(),
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("周期规则 ID 不能为空")
        );
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_progress_record_create_required_validation(#[case] source_title: &str) {
        let result = v1::progress_record::create::CreateRequest::new(
            test_config("https://127.0.0.1:9"),
            source_title.to_string(),
            "https://www.zhoubao.com".to_string(),
            "7041430377642082323".to_string(),
            2,
            v1::progress_record::create::ContentBlock { blocks: Vec::new() },
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("进展来源不能为空")
        );
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_progress_record_get_id_validation(#[case] progress_id: &str) {
        let result = v1::progress_record::get::GetRequest::new(
            test_config("https://127.0.0.1:9"),
            progress_id.to_string(),
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("进展记录 ID 不能为空")
        );
    }

    #[tokio::test]
    async fn test_okr_batch_get_empty_list_validation() {
        let result =
            v1::okr::batch_get::BatchGetRequest::new(test_config("https://127.0.0.1:9"), vec![])
                .execute_with_options(auth_option())
                .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("OKR ID 列表不能为空且不能超过 10 个")
        );
    }

    #[tokio::test]
    async fn test_okr_batch_get_too_many_validation() {
        let ids = (0..11).map(|idx| format!("okr_{idx}")).collect::<Vec<_>>();
        let result =
            v1::okr::batch_get::BatchGetRequest::new(test_config("https://127.0.0.1:9"), ids)
                .execute_with_options(auth_option())
                .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("OKR ID 列表不能为空且不能超过 10 个")
        );
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_period_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/periods"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({
                "period_rule_id":"6969864184272078374",
                "start_month":"2026-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"period_id":"period_1","start_month":"2026-01","end_month":"2026-01"}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::period::create::CreateRequest::new(
            test_config(&mock_server.uri()),
            "6969864184272078374".to_string(),
            "2026-01".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.period_id, "period_1");
        assert_eq!(resp.end_month, "2026-01");
    }

    #[tokio::test]
    async fn test_progress_record_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/progress_records"))
            .and(body_json(json!({
                "source_title":"周报系统",
                "source_url":"https://www.zhoubao.com",
                "target_id":"7041430377642082323",
                "target_type":2,
                "content":{"blocks":[{"type":"paragraph","paragraph":{"elements":[{"type":"textRun","textRun":{"text":"进展更新"}}]}}]},
                "progress_rate":{"percent":60.0,"status":0}
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"progress_id":"p_1","modify_time":"1735689600000","content":{"blocks":[]}}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::progress_record::create::CreateRequest::new(
            test_config(&mock_server.uri()),
            "周报系统".to_string(),
            "https://www.zhoubao.com".to_string(),
            "7041430377642082323".to_string(),
            2,
            v1::progress_record::create::ContentBlock {
                blocks: vec![v1::progress_record::create::ContentBlockElement {
                    block_element_type: Some("paragraph".to_string()),
                    paragraph: Some(v1::progress_record::create::ContentParagraph {
                        style: None,
                        elements: vec![v1::progress_record::create::ContentParagraphElement {
                            paragraph_element_type: Some("textRun".to_string()),
                            text_run: Some(v1::progress_record::create::ContentTextRun {
                                text: Some("进展更新".to_string()),
                                style: None,
                            }),
                            docs_link: None,
                            person: None,
                        }],
                    }),
                    gallery: None,
                }],
            },
        )
        .progress_rate(v1::progress_record::create::ProgressRateNew {
            percent: Some(60.0),
            status: Some(0),
        })
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.progress_id, "p_1");
    }

    #[tokio::test]
    async fn test_okr_batch_get_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/okrs/batch_get"))
            .and(query_param("okr_ids", "okr_1,okr_2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"okr_list":[{"id":"okr_1","name":"目标1","period_id":"period_1","objective_list":[]}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::okr::batch_get::BatchGetRequest::new(
            test_config(&mock_server.uri()),
            vec!["okr_1".to_string(), "okr_2".to_string()],
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.okr_list[0].id, "okr_1");
    }

    #[tokio::test]
    async fn test_review_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/reviews/query"))
            .and(query_param("user_ids", "ou_1"))
            .and(query_param("period_ids", "period_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"review_list":[{"review_period_list":[{"cycle_review_list":[{"create_time":"1646902033525","url":"https://example.feishu.cn/docs/doc1"}],"period_id":"period_1","progress_report_list":[]}],"user_id":{"open_id":"ou_1","user_id":"6969855501744834092"}}]}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::review::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            vec!["ou_1".to_string()],
            vec!["period_1".to_string()],
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.review_list.len(), 1);
        assert_eq!(
            resp.review_list[0]
                .user_id
                .as_ref()
                .and_then(|u| u.open_id.as_deref()),
            Some("ou_1")
        );
        assert_eq!(resp.review_list[0].review_period_list.len(), 1);
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
        test_image_upload_response_serialization,
        v1::image::upload::UploadResponse,
        v1::image::upload::UploadResponse {
            file_token: "boxcnOj88GDkmWGm2zsTyCBqoLb".to_string(),
            url: "https://example.com/img_1.png".to_string(),
        }
    );
    roundtrip_eq!(
        test_okr_batch_get_response_serialization,
        v1::okr::batch_get::BatchGetResponse,
        v1::okr::batch_get::BatchGetResponse {
            okr_list: vec![v1::okr::batch_get::OkrBatch {
                id: "okr_1".to_string(),
                permission: Some(1),
                period_id: Some("period_1".to_string()),
                name: Some("目标1".to_string()),
                objective_list: Vec::new(),
                confirm_status: Some(1),
            }],
        }
    );
    roundtrip_eq!(
        test_period_create_response_serialization,
        v1::period::create::CreateResponse,
        v1::period::create::CreateResponse {
            period_id: "period_1".to_string(),
            start_month: "2026-01".to_string(),
            end_month: "2026-01".to_string(),
        }
    );
    roundtrip_eq!(
        test_period_list_response_serialization,
        v1::period::list::ListResponse,
        v1::period::list::ListResponse {
            items: vec![v1::period::list::OkrPeriod {
                id: "7071200999834255380".to_string(),
                zh_name: Some("2022 年 1 月 - 3 月".to_string()),
                en_name: Some("Jan - Mar 2022".to_string()),
                status: Some(0),
                period_start_time: Some("1546272000000".to_string()),
                period_end_time: Some("1577721600000".to_string()),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_period_patch_response_serialization,
        v1::period::patch::PatchResponse,
        v1::period::patch::PatchResponse {
            period_id: "period_1".to_string(),
            status: 2,
            updated_at: 1735776000,
        }
    );
    roundtrip_eq!(
        test_period_rule_list_response_serialization,
        v1::period_rule::list::ListResponse,
        v1::period_rule::list::ListResponse {
            period_rules: vec![v1::period_rule::list::PeriodRule {
                period_rule_id: "134".to_string(),
                period_type: Some("year".to_string()),
                length: Some(12),
                first_month: Some(12),
            }],
        }
    );
    roundtrip_eq!(
        test_progress_record_create_response_serialization,
        v1::progress_record::create::CreateResponse,
        v1::progress_record::create::CreateResponse {
            progress_id: "p_1".to_string(),
            modify_time: Some("1735689600000".to_string()),
            content: Some(v1::progress_record::create::ContentBlock { blocks: Vec::new() }),
            progress_rate: Some(v1::progress_record::create::ProgressRateNew {
                percent: Some(60.0),
                status: Some(0),
            }),
        }
    );
    roundtrip_eq!(
        test_progress_record_get_response_serialization,
        v1::progress_record::get::GetResponse,
        v1::progress_record::get::GetResponse {
            progress_id: "p_1".to_string(),
            modify_time: Some("1735689600000".to_string()),
            content: Some(v1::progress_record::create::ContentBlock { blocks: Vec::new() }),
            progress_rate: Some(v1::progress_record::create::ProgressRateNew {
                percent: Some(60.0),
                status: Some(0),
            }),
        }
    );
    roundtrip_eq!(
        test_progress_record_delete_response_serialization,
        v1::progress_record::delete::DeleteResponse,
        v1::progress_record::delete::DeleteResponse { result: true }
    );
    roundtrip_eq!(
        test_progress_record_update_response_serialization,
        v1::progress_record::update::UpdateResponse,
        v1::progress_record::update::UpdateResponse {
            progress_id: "p_1".to_string(),
            modify_time: Some("1735776000000".to_string()),
            content: Some(v1::progress_record::create::ContentBlock { blocks: Vec::new() }),
            progress_rate: Some(v1::progress_record::create::ProgressRateNew {
                percent: Some(70.0),
                status: Some(1),
            }),
        }
    );
    roundtrip_eq!(
        test_review_query_response_serialization,
        v1::review::query::QueryResponse,
        v1::review::query::QueryResponse {
            review_list: vec![v1::review::query::OkrReview {
                user_id: Some(v1::okr::batch_get::OkrUser {
                    open_id: Some("ou_e6139117c300506837def50545420c6a".to_string()),
                    user_id: Some("6969855501744834092".to_string()),
                }),
                review_period_list: vec![v1::review::query::OkrReviewPeriod {
                    period_id: Some("period_1".to_string()),
                    cycle_review_list: vec![v1::review::query::OkrReviewPeriodUrl {
                        url: Some("https://example.feishu.cn/docs/doc1".to_string()),
                        create_time: Some("1646902033525".to_string()),
                    }],
                    progress_report_list: Vec::new(),
                }],
            }],
        }
    );
    roundtrip_eq!(
        test_user_okr_list_response_serialization,
        v1::user::okr::list::ListResponse,
        v1::user::okr::list::ListResponse {
            okr_list: vec![v1::okr::batch_get::OkrBatch {
                id: "7072252816005349396".to_string(),
                permission: Some(1),
                period_id: Some("period_1".to_string()),
                name: Some("2022 年 3 月".to_string()),
                objective_list: Vec::new(),
                confirm_status: Some(4),
            }],
            total: Some(14),
        }
    );
}
