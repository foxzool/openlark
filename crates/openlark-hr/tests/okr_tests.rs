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
            1,
            "progress.png".to_string(),
            "YmFzZTY0".to_string(),
        )
    );
    smoke_builder!(
        test_okr_batch_get_builder,
        v1::okr::batch_get::BatchGetRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["okr_1".to_string(), "okr_2".to_string()],
        )
    );
    smoke_builder!(
        test_period_create_builder,
        v1::period::create::CreateRequest::new(
            test_config("https://open.feishu.cn"),
            "2026Q1".to_string(),
            1735689600,
            1743465600,
        )
        .description("季度 OKR".to_string())
        .target_setting_deadline(1736294400)
        .review_time(1743379200)
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
            "okr_1".to_string(),
            "进展更新".to_string(),
            60,
        )
        .description("本周推进顺利".to_string())
        .attachments(vec![v1::progress_record::create::ProgressAttachment {
            file_type: "image".to_string(),
            file_url: "https://example.com/1.png".to_string(),
            file_name: "1.png".to_string(),
        }])
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
            "新进展".to_string(),
            70,
        )
        .description("更新说明".to_string())
    );
    smoke_builder!(
        test_review_query_builder,
        v1::review::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            "period_1".to_string(),
        )
        .page_size(20)
    );
    smoke_builder!(
        test_user_okr_list_builder,
        v1::user::okr::list::ListRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
        )
        .period_id("period_1".to_string())
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
    async fn test_period_create_name_validation(#[case] name: &str) {
        let result = v1::period::create::CreateRequest::new(
            test_config("https://127.0.0.1:9"),
            name.to_string(),
            1735689600,
            1743465600,
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("周期名称不能为空")
        );
    }

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_progress_record_create_required_validation(#[case] okr_id: &str) {
        let result = v1::progress_record::create::CreateRequest::new(
            test_config("https://127.0.0.1:9"),
            okr_id.to_string(),
            "进展".to_string(),
            20,
        )
        .execute_with_options(auth_option())
        .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("OKR ID 不能为空")
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
                .contains("OKR ID 列表不能为空且不能超过 50 个")
        );
    }

    #[tokio::test]
    async fn test_okr_batch_get_too_many_validation() {
        let ids = (0..51).map(|idx| format!("okr_{idx}")).collect::<Vec<_>>();
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
                .contains("OKR ID 列表不能为空且不能超过 50 个")
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
                "name":"2026Q1",
                "start_time":1735689600,
                "end_time":1743465600
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"period_id":"period_1","name":"2026Q1","start_time":1735689600,"end_time":1743465600,"status":1,"created_at":1735689600}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::period::create::CreateRequest::new(
            test_config(&mock_server.uri()),
            "2026Q1".to_string(),
            1735689600,
            1743465600,
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.period_id, "period_1");
    }

    #[tokio::test]
    async fn test_progress_record_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/progress_records"))
            .and(body_json(json!({
                "okr_id":"okr_1",
                "content":"进展更新",
                "progress_rate":60
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"progress_id":"p_1","okr_id":"okr_1","content":"进展更新","progress_rate":60,"created_at":1735689600}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::progress_record::create::CreateRequest::new(
            test_config(&mock_server.uri()),
            "okr_1".to_string(),
            "进展更新".to_string(),
            60,
        )
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
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"items":[{"okr_id":"okr_1","title":"目标1","content":"内容","period_id":"period_1","progress_rate":30,"status":1,"creator_id":"ou_1","created_at":1735689600,"updated_at":1735776000}]}
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

        assert_eq!(resp.items[0].okr_id, "okr_1");
    }

    #[tokio::test]
    async fn test_review_query_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/reviews/query"))
            .and(query_param("period_id", "period_1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{"items":[{"review_id":"r_1","period_id":"period_1","user_id":"ou_1","content":"复盘","score":4,"self_assessment":"较好","submitted_at":1735689600}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = v1::review::query::QueryRequest::new(
            test_config(&mock_server.uri()),
            "period_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();

        assert_eq!(resp.items[0].review_id, "r_1");
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
            image_id: "img_1".to_string(),
            image_url: "https://example.com/img_1.png".to_string(),
        }
    );
    roundtrip_eq!(
        test_okr_batch_get_response_serialization,
        v1::okr::batch_get::BatchGetResponse,
        v1::okr::batch_get::BatchGetResponse {
            items: vec![v1::okr::batch_get::OkrInfo {
                okr_id: "okr_1".to_string(),
                title: "目标1".to_string(),
                content: "季度增长".to_string(),
                period_id: "period_1".to_string(),
                progress_rate: 50,
                status: 1,
                creator_id: "ou_1".to_string(),
                created_at: 1735689600,
                updated_at: 1735776000,
            }],
        }
    );
    roundtrip_eq!(
        test_period_create_response_serialization,
        v1::period::create::CreateResponse,
        v1::period::create::CreateResponse {
            period_id: "period_1".to_string(),
            name: "2026Q1".to_string(),
            start_time: 1735689600,
            end_time: 1743465600,
            status: 1,
            created_at: 1735689600,
        }
    );
    roundtrip_eq!(
        test_period_list_response_serialization,
        v1::period::list::ListResponse,
        v1::period::list::ListResponse {
            items: vec![v1::period::list::OkrPeriod {
                period_id: "period_1".to_string(),
                name: "2026Q1".to_string(),
                start_time: 1735689600,
                end_time: 1743465600,
                status: 2,
                description: Some("季度目标".to_string()),
                target_setting_deadline: Some(1736294400),
                review_time: Some(1743379200),
                created_at: 1735689600,
                updated_at: 1735776000,
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
            items: vec![v1::period_rule::list::PeriodRule {
                rule_id: "rule_1".to_string(),
                period_id: "period_1".to_string(),
                name: "评分规则".to_string(),
                rule_type: 1,
                config: json!({"score_weight": 0.6}),
                created_at: 1735689600,
                updated_at: 1735776000,
            }],
        }
    );
    roundtrip_eq!(
        test_progress_record_create_response_serialization,
        v1::progress_record::create::CreateResponse,
        v1::progress_record::create::CreateResponse {
            progress_id: "p_1".to_string(),
            okr_id: "okr_1".to_string(),
            content: "进展更新".to_string(),
            progress_rate: 60,
            created_at: 1735689600,
        }
    );
    roundtrip_eq!(
        test_progress_record_get_response_serialization,
        v1::progress_record::get::GetResponse,
        v1::progress_record::get::GetResponse {
            progress_id: "p_1".to_string(),
            okr_id: "okr_1".to_string(),
            content: "进展更新".to_string(),
            progress_rate: 60,
            description: Some("说明".to_string()),
            attachments: Some(vec![v1::progress_record::create::ProgressAttachment {
                file_type: "image".to_string(),
                file_url: "https://example.com/1.png".to_string(),
                file_name: "1.png".to_string(),
            }]),
            creator_id: "ou_1".to_string(),
            created_at: 1735689600,
            updated_at: 1735776000,
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
            okr_id: "okr_1".to_string(),
            content: "新进展".to_string(),
            progress_rate: 70,
            updated_at: 1735776000,
        }
    );
    roundtrip_eq!(
        test_review_query_response_serialization,
        v1::review::query::QueryResponse,
        v1::review::query::QueryResponse {
            items: vec![v1::review::query::ReviewInfo {
                review_id: "review_1".to_string(),
                period_id: "period_1".to_string(),
                user_id: "ou_1".to_string(),
                content: "复盘内容".to_string(),
                score: 4,
                self_assessment: "表现良好".to_string(),
                peer_assessment: Some("协作优秀".to_string()),
                improvement_plan: Some("加强复盘节奏".to_string()),
                submitted_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_user_okr_list_response_serialization,
        v1::user::okr::list::ListResponse,
        v1::user::okr::list::ListResponse {
            items: vec![v1::user::okr::list::UserOkr {
                okr_id: "okr_1".to_string(),
                title: "增长目标".to_string(),
                content: "季度增长 20%".to_string(),
                period_id: "period_1".to_string(),
                progress_rate: 45,
                status: 1,
                created_at: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );
}
