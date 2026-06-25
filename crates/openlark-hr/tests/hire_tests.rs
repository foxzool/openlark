//! hire tests 集成测试。
#![cfg(feature = "hire")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::hire::hire::v1::*;
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
        test_talent_combined_create_builder,
        talent::CombinedCreateRequest::new(test_config("https://open.feishu.cn"))
            .name("张三".to_string())
            .email("zhangsan@example.com".to_string())
    );
    smoke_builder!(
        test_talent_get_builder,
        talent::GetRequest::new(test_config("https://open.feishu.cn"))
            .talent_id("talent_001".to_string())
            .need_detail(true)
    );
    smoke_builder!(
        test_talent_combined_update_builder,
        talent::CombinedUpdateRequest::new(test_config("https://open.feishu.cn"))
            .talent_id("talent_001".to_string())
            .name("李四".to_string())
    );
    smoke_builder!(
        test_talent_list_builder,
        talent::ListRequest::new(test_config("https://open.feishu.cn"))
            .page_size(20)
            .query("后端".to_string())
    );
    smoke_builder!(
        test_talent_batch_get_id_builder,
        talent::BatchGetIdRequest::new(test_config("https://open.feishu.cn"))
            .add_talent_id("talent_001".to_string())
            .add_talent_id("talent_002".to_string())
    );

    smoke_builder!(
        test_interview_list_builder,
        interview::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_get_by_talent_builder,
        interview::get_by_talent::GetByTalentRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_interview_list_builder,
        application::interview::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_record_get_builder,
        interview_record::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_record_list_builder,
        interview_record::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_offer_create_builder,
        offer::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_get_builder,
        offer::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_update_builder,
        offer::update::UpdateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_list_builder,
        offer::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_offer_status_builder,
        offer::offer_status::OfferStatusRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_application_create_builder,
        application::create::CreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_get_builder,
        application::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_get_detail_builder,
        application::get_detail::GetDetailRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_application_list_builder,
        application::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_job_combined_create_builder,
        job::combined_create::CombinedCreateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_get_builder,
        job::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_combined_update_builder,
        job::combined_update::CombinedUpdateRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_list_builder,
        job::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_open_builder,
        job::open::OpenRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_evaluation_list_builder,
        evaluation::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_evaluation_task_list_builder,
        evaluation_task::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_interview_feedback_form_list_builder,
        interview_feedback_form::list::ListRequest::new(test_config("https://open.feishu.cn"))
    );

    smoke_builder!(
        test_advertisement_publish_builder,
        advertisement::publish::PublishRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_attachment_get_builder,
        attachment::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_referral_search_builder,
        referral::search::SearchRequest::new(test_config("https://open.feishu.cn"))
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_talent_combined_create_name_validation(#[case] name: &str) {
        let result = talent::CombinedCreateRequest::new(test_config("https://127.0.0.1:9"))
            .name(name.to_string())
            .email("test@example.com".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("候选人姓名不能为空")
        );
    }

    #[tokio::test]
    async fn test_talent_combined_create_contact_validation() {
        let result = talent::CombinedCreateRequest::new(test_config("https://127.0.0.1:9"))
            .name("张三".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("至少需要提供邮箱或手机号")
        );
    }

    #[rstest]
    #[case(0)]
    #[case(101)]
    #[tokio::test]
    async fn test_talent_list_page_size_boundary(#[case] page_size: i32) {
        let result = talent::ListRequest::new(test_config("https://127.0.0.1:9"))
            .page_size(page_size)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("page_size 必须在 1-100 之间")
        );
    }

    #[tokio::test]
    async fn test_talent_batch_get_id_empty_validation() {
        let result = talent::BatchGetIdRequest::new(test_config("https://127.0.0.1:9"))
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(result.err().unwrap().to_string().contains("不能为空"));
    }

    #[tokio::test]
    async fn test_talent_combined_update_requires_fields() {
        let result = talent::CombinedUpdateRequest::new(test_config("https://127.0.0.1:9"))
            .talent_id("talent_001".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("至少需要提供一个更新字段")
        );
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_talent_combined_create_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_create"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(
                json!({"name":"张三","email":"zhangsan@example.com"}),
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(
                    json!({"code":0,"msg":"success","data":{"talent_id":"talent_001"}}),
                ),
            )
            .mount(&mock_server)
            .await;

        let resp = talent::CombinedCreateRequest::new(test_config(&mock_server.uri()))
            .name("张三".to_string())
            .email("zhangsan@example.com".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_id, "talent_001");
    }

    #[tokio::test]
    async fn test_talent_get_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/talents/talent_001"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent":{"talent_id":"talent_001","name":"张三"}
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::GetRequest::new(test_config(&mock_server.uri()))
            .talent_id("talent_001".to_string())
            .need_detail(true)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent.talent_id, "talent_001");
    }

    #[tokio::test]
    async fn test_talent_combined_update_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/combined_update"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"talent_id":"talent_001","name":"李四"})))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"result":true}})),
            )
            .mount(&mock_server)
            .await;

        let resp = talent::CombinedUpdateRequest::new(test_config(&mock_server.uri()))
            .talent_id("talent_001".to_string())
            .name("李四".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_talent_list_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/talents"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent_list":[{"talent_id":"talent_001","name":"张三"}],
                    "has_more":false
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::ListRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_list.len(), 1);
    }

    #[tokio::test]
    async fn test_talent_batch_get_id_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/talents/batch_get_id"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(json!({"talent_ids":["talent_001","talent_002"]})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,
                "msg":"success",
                "data":{
                    "talent_id_list":[
                        {"talent_id":"talent_001","is_valid":true},
                        {"talent_id":"talent_002","is_valid":true}
                    ]
                }
            })))
            .mount(&mock_server)
            .await;

        let resp = talent::BatchGetIdRequest::new(test_config(&mock_server.uri()))
            .talent_ids(vec!["talent_001".to_string(), "talent_002".to_string()])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.talent_id_list.len(), 2);
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
        test_talent_combined_create_response_serialization,
        talent::CombinedCreateResponse,
        talent::CombinedCreateResponse {
            talent_id: "talent_001".to_string()
        }
    );
    roundtrip_eq!(
        test_talent_get_response_serialization,
        talent::GetResponse,
        talent::GetResponse {
            talent: talent::Talent {
                talent_id: "talent_001".to_string(),
                name: Some("张三".to_string()),
                resume: None,
                email: Some("zhangsan@example.com".to_string()),
                phone: None,
                status: Some(1),
                create_time: None,
                update_time: None,
            },
            education_list: None,
            work_experience_list: None,
        }
    );
    roundtrip_eq!(
        test_talent_combined_update_response_serialization,
        talent::CombinedUpdateResponse,
        talent::CombinedUpdateResponse { result: true }
    );
    roundtrip_eq!(
        test_talent_list_response_serialization,
        talent::ListResponse,
        talent::ListResponse {
            talent_list: vec![talent::Talent {
                talent_id: "talent_001".to_string(),
                name: Some("张三".to_string()),
                resume: None,
                email: None,
                phone: None,
                status: Some(1),
                create_time: None,
                update_time: None,
            }],
            page_token: Some("next_1".to_string()),
            has_more: Some(false),
        }
    );
    roundtrip_eq!(
        test_talent_batch_get_id_response_serialization,
        talent::BatchGetIdResponse,
        talent::BatchGetIdResponse {
            talent_id_list: vec![talent::TalentIdInfo {
                talent_id: "talent_001".to_string(),
                is_valid: true,
            }],
        }
    );

    roundtrip_eq!(
        test_application_interview_list_response_serialization,
        application::interview::list::ListResponse,
        application::interview::list::ListResponse {
            records: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_record_get_response_serialization,
        interview_record::get::GetResponse,
        interview_record::get::GetResponse {
            id: Some("ir_001".to_string()),
            conclusion: Some(1),
            interviewer: Some(openlark_hr::hire::hire::common_models::IdNameObject {
                id: Some("ou_interviewer".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    );
    roundtrip_eq!(
        test_interview_record_list_response_serialization,
        interview_record::list::ListResponse,
        interview_record::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_offer_create_response_serialization,
        offer::create::CreateResponse,
        offer::create::CreateResponse {
            offer_id: Some("of_001".to_string()),
            application_id: Some("ap_001".to_string()),
            schema_id: Some("schema_1".to_string()),
            offer_type: Some(1),
            basic_info: None,
            salary_info: None,
            customized_info_list: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_get_response_serialization,
        offer::get::GetResponse,
        offer::get::GetResponse {
            offer: Some(offer::get::OfferDetail {
                id: Some("of_001".to_string()),
                application_id: Some("ap_001".to_string()),
                basic_info: None,
                salary_plan: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_update_response_serialization,
        offer::update::UpdateResponse,
        offer::update::UpdateResponse {
            offer_id: Some("of_001".to_string()),
            application_id: None,
            schema_id: Some("schema_1".to_string()),
            offer_type: Some(1),
            basic_info: None,
            salary_info: None,
            customized_info_list: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_list_response_serialization,
        offer::list::ListResponse,
        offer::list::ListResponse {
            has_more: Some(false),
            page_token: None,
            items: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_intern_status_response_serialization,
        offer::intern_offer_status::InternOfferStatusResponse,
        offer::intern_offer_status::InternOfferStatusResponse {
            offer_id: Some("of_001".to_string()),
            operation: Some("confirm_onboarding".to_string()),
            onboarding_info: Some(offer::intern_offer_status::InternOnboardingInfo {
                actual_onboarding_date: Some("2022-01-01".to_string()),
                extra: Default::default(),
            }),
            offboarding_info: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_status_response_serialization,
        offer::offer_status::OfferStatusResponse,
        offer::offer_status::OfferStatusResponse {}
    );

    roundtrip_eq!(
        test_application_create_response_serialization,
        application::create::CreateResponse,
        application::create::CreateResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(1),
            stage_id: Some("stage_1".to_string()),
            stage_name: Some("初筛".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_get_response_serialization,
        application::get::GetResponse,
        application::get::GetResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(1),
            stage_id: Some("stage_1".to_string()),
            stage_name: Some("初筛".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_get_detail_response_serialization,
        application::get_detail::GetDetailResponse,
        application::get_detail::GetDetailResponse {
            application_id: Some("ap_001".to_string()),
            talent_id: Some("talent_001".to_string()),
            job_id: Some("job_001".to_string()),
            application_status: Some(2),
            stage_id: Some("stage_offer".to_string()),
            stage_name: Some("Offer".to_string()),
            job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                extra: Default::default(),
            }),
            talent_info: Some(
                openlark_hr::hire::hire::common_models::ApplicationTalentInfo {
                    talent_id: Some("talent_001".to_string()),
                    talent_name: Some("张三".to_string()),
                    mobile: None,
                    email: Some("zhangsan@example.com".to_string()),
                    extra: Default::default(),
                }
            ),
            offer_info: Some(
                openlark_hr::hire::hire::common_models::ApplicationOfferInfo {
                    offer_id: Some("offer_001".to_string()),
                    offer_status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_list_response_serialization,
        application::list::ListResponse,
        application::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::ApplicationSummary {
                application_id: Some("ap_001".to_string()),
                talent_id: Some("talent_001".to_string()),
                job_id: Some("job_001".to_string()),
                application_status: Some(1),
                stage_id: Some("stage_1".to_string()),
                stage_name: Some("初筛".to_string()),
                job_info: Some(openlark_hr::hire::hire::common_models::ApplicationJobInfo {
                    job_id: Some("job_001".to_string()),
                    job_name: Some("后端工程师".to_string()),
                    extra: Default::default(),
                }),
                extra: Default::default(),
            }],
            page_token: Some("cursor_1".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_cancel_onboard_response_serialization,
        application::cancel_onboard::CancelOnboardResponse,
        application::cancel_onboard::CancelOnboardResponse {
            operation: openlark_hr::hire::hire::common_models::ApplicationOperationResult {
                application_id: Some("app_1".to_string()),
                stage_id: None,
                stage_name: None,
                offer_id: None,
                employee_id: None,
                onboard_status: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_application_offer_response_serialization,
        application::offer::OfferResponse,
        application::offer::OfferResponse {
            offer: Some(openlark_hr::hire::hire::v1::offer::get::OfferDetail {
                id: Some("offer_1".to_string()),
                application_id: Some("app_1".to_string()),
                basic_info: None,
                salary_plan: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_application_recover_response_serialization,
        application::recover::RecoverResponse,
        application::recover::RecoverResponse {
            operation: openlark_hr::hire::hire::common_models::ApplicationOperationResult {
                application_id: Some("app_1".to_string()),
                stage_id: Some("stage_resume".to_string()),
                stage_name: Some("简历筛选".to_string()),
                offer_id: None,
                employee_id: None,
                onboard_status: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_application_terminate_response_serialization,
        application::terminate::TerminateResponse,
        application::terminate::TerminateResponse {
            operation: openlark_hr::hire::hire::common_models::ApplicationOperationResult {
                application_id: Some("app_1".to_string()),
                stage_id: None,
                stage_name: None,
                offer_id: None,
                employee_id: None,
                onboard_status: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_application_transfer_onboard_response_serialization,
        application::transfer_onboard::TransferOnboardResponse,
        application::transfer_onboard::TransferOnboardResponse {
            operation: openlark_hr::hire::hire::common_models::ApplicationOperationResult {
                application_id: Some("app_1".to_string()),
                stage_id: None,
                stage_name: None,
                offer_id: None,
                employee_id: Some("emp_1".to_string()),
                onboard_status: Some(1),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_application_transfer_stage_response_serialization,
        application::transfer_stage::TransferStageResponse,
        application::transfer_stage::TransferStageResponse {
            operation: openlark_hr::hire::hire::common_models::ApplicationOperationResult {
                application_id: Some("app_1".to_string()),
                stage_id: Some("stage_interview".to_string()),
                stage_name: Some("面试".to_string()),
                offer_id: None,
                employee_id: None,
                onboard_status: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_external_background_check_batch_query_response_serialization,
        external_background_check::batch_query::BatchQueryResponse,
        external_background_check::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalBackgroundCheckSummary {
                    external_background_check_id: Some("bg_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(1),
                    vendor_name: Some("iBackground".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_bg".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_background_check_create_response_serialization,
        external_background_check::create::CreateResponse,
        external_background_check::create::CreateResponse {
            external_background_check_id: Some("bg_1".to_string()),
            application_id: Some("app_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_background_check_delete_response_serialization,
        external_background_check::delete::DeleteResponse,
        external_background_check::delete::DeleteResponse {
            external_background_check_id: Some("bg_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_background_check_update_response_serialization,
        external_background_check::update::UpdateResponse,
        external_background_check::update::UpdateResponse {
            external_background_check_id: Some("bg_1".to_string()),
            application_id: Some("app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_interview_batch_query_response_serialization,
        external_interview::batch_query::BatchQueryResponse,
        external_interview::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalInterviewSummary {
                    external_interview_id: Some("iv_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(2),
                    interview_round_name: Some("一面".to_string()),
                    start_time: Some("2026-05-01T10:00:00Z".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_iv".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_interview_create_response_serialization,
        external_interview::create::CreateResponse,
        external_interview::create::CreateResponse {
            external_interview_id: Some("iv_1".to_string()),
            application_id: Some("app_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_interview_delete_response_serialization,
        external_interview::delete::DeleteResponse,
        external_interview::delete::DeleteResponse {
            external_interview_id: Some("iv_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_interview_update_response_serialization,
        external_interview::update::UpdateResponse,
        external_interview::update::UpdateResponse {
            external_interview_id: Some("iv_1".to_string()),
            application_id: Some("app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_tripartite_agreement_create_response_serialization,
        tripartite_agreement::create::CreateResponse,
        tripartite_agreement::create::CreateResponse {
            agreement_id: Some("agree_1".to_string()),
            application_id: Some("app_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_tripartite_agreement_delete_response_serialization,
        tripartite_agreement::delete::DeleteResponse,
        tripartite_agreement::delete::DeleteResponse {
            agreement_id: Some("agree_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_tripartite_agreement_list_response_serialization,
        tripartite_agreement::list::ListResponse,
        tripartite_agreement::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::TripartiteAgreementSummary {
                    agreement_id: Some("agree_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(1),
                    sign_status: Some(2),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_agree".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_tripartite_agreement_update_response_serialization,
        tripartite_agreement::update::UpdateResponse,
        tripartite_agreement::update::UpdateResponse {
            agreement_id: Some("agree_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_job_combined_create_response_serialization,
        job::combined_create::CombinedCreateResponse,
        job::combined_create::CombinedCreateResponse {
            job_id: Some("job_001".to_string()),
            active_status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_get_response_serialization,
        job::get::GetResponse,
        job::get::GetResponse {
            job_id: Some("job_001".to_string()),
            job_name: Some("后端工程师".to_string()),
            active_status: Some(1),
            process_id: Some("process_1".to_string()),
            process_name: Some("标准流程".to_string()),
            department_id: Some("dept_1".to_string()),
            recruiters: Some(vec![
                openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                    recruiter_id: Some("rec_1".to_string()),
                    manager_id: None,
                    user_id: Some("ou_1".to_string()),
                    name: Some("招聘负责人".to_string()),
                    role: Some("owner".to_string()),
                    role_type: Some(1),
                    extra: Default::default(),
                }
            ]),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_get_detail_response_serialization,
        job::get_detail::GetDetailResponse,
        job::get_detail::GetDetailResponse {
            job_id: Some("job_001".to_string()),
            job_name: Some("后端工程师".to_string()),
            active_status: Some(1),
            process_id: Some("process_1".to_string()),
            process_name: Some("标准流程".to_string()),
            department_id: Some("dept_1".to_string()),
            job_description: Some("负责核心服务开发".to_string()),
            recruiters: Some(vec![
                openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                    recruiter_id: Some("rec_1".to_string()),
                    manager_id: None,
                    user_id: Some("ou_1".to_string()),
                    name: Some("招聘负责人".to_string()),
                    role: Some("owner".to_string()),
                    role_type: Some(1),
                    extra: Default::default(),
                }
            ]),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_combined_update_response_serialization,
        job::combined_update::CombinedUpdateResponse,
        job::combined_update::CombinedUpdateResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_list_response_serialization,
        job::list::ListResponse,
        job::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::JobSummary {
                job_id: Some("job_001".to_string()),
                job_name: Some("后端工程师".to_string()),
                active_status: Some(1),
                process_id: Some("process_1".to_string()),
                process_name: Some("标准流程".to_string()),
                department_id: Some("dept_1".to_string()),
                job_description: None,
                recruiters: None,
                extra: Default::default(),
            }],
            page_token: Some("cursor_job".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_open_response_serialization,
        job::open::OpenResponse,
        job::open::OpenResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_close_response_serialization,
        job::close::CloseResponse,
        job::close::CloseResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_config_response_serialization,
        job::config::ConfigResponse,
        job::config::ConfigResponse {
            config: openlark_hr::hire::hire::common_models::JobConfigInfo {
                job_id: Some("job_001".to_string()),
                process_id: Some("process_1".to_string()),
                process_name: Some("标准流程".to_string()),
                job_requirement_schema_id: Some("schema_job".to_string()),
                interview_registration_schema_id: Some("schema_interview".to_string()),
                offer_application_form_id: Some("form_offer".to_string()),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_job_recruiter_response_serialization,
        job::recruiter::RecruiterResponse,
        job::recruiter::RecruiterResponse {
            recruiters: vec![openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                recruiter_id: Some("rec_1".to_string()),
                manager_id: None,
                user_id: Some("ou_1".to_string()),
                name: Some("招聘负责人".to_string()),
                role: Some("owner".to_string()),
                role_type: Some(1),
                extra: Default::default(),
            }],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_update_config_response_serialization,
        job::update_config::UpdateConfigResponse,
        job::update_config::UpdateConfigResponse {
            job_id: Some("job_001".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_create_response_serialization,
        external_application::create::CreateResponse,
        external_application::create::CreateResponse {
            external_application_id: Some("ext_app_1".to_string()),
            application_id: Some("app_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_delete_response_serialization,
        external_application::delete::DeleteResponse,
        external_application::delete::DeleteResponse {
            external_application_id: Some("ext_app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_list_response_serialization,
        external_application::list::ListResponse,
        external_application::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalApplicationSummary {
                    external_application_id: Some("ext_app_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    status: Some(1),
                    source_name: Some("Boss".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_ext_app".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_update_response_serialization,
        external_application::update::UpdateResponse,
        external_application::update::UpdateResponse {
            external_application_id: Some("ext_app_1".to_string()),
            application_id: Some("app_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_create_response_serialization,
        external_offer::create::CreateResponse,
        external_offer::create::CreateResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            offer_id: Some("offer_1".to_string()),
            status: Some(1),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_delete_response_serialization,
        external_offer::delete::DeleteResponse,
        external_offer::delete::DeleteResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_batch_query_response_serialization,
        external_offer::batch_query::BatchQueryResponse,
        external_offer::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalOfferSummary {
                    external_offer_id: Some("ext_offer_1".to_string()),
                    offer_id: Some("offer_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(2),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_ext_offer".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_update_response_serialization,
        external_offer::update::UpdateResponse,
        external_offer::update::UpdateResponse {
            external_offer_id: Some("ext_offer_1".to_string()),
            offer_id: Some("offer_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_channel_create_response_serialization,
        website::channel::create::CreateResponse,
        website::channel::create::CreateResponse {
            channel: Some(
                openlark_hr::hire::hire::common_models::WebsiteChannelSummary {
                    channel_id: Some("channel_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    name: Some("Boss".to_string()),
                    code: Some("boss".to_string()),
                    active_status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_channel_delete_response_serialization,
        website::channel::delete::DeleteResponse,
        website::channel::delete::DeleteResponse {
            channel_id: Some("channel_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_channel_list_response_serialization,
        website::channel::list::ListResponse,
        website::channel::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::WebsiteChannelSummary {
                    channel_id: Some("channel_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    name: Some("Boss".to_string()),
                    code: Some("boss".to_string()),
                    active_status: Some(1),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_channel".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_channel_update_response_serialization,
        website::channel::update::UpdateResponse,
        website::channel::update::UpdateResponse {
            channel: Some(
                openlark_hr::hire::hire::common_models::WebsiteChannelSummary {
                    channel_id: Some("channel_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    name: Some("Boss".to_string()),
                    code: Some("boss".to_string()),
                    active_status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_create_by_attachment_response_serialization,
        website::delivery::create_by_attachment::CreateByAttachmentResponse,
        website::delivery::create_by_attachment::CreateByAttachmentResponse {
            delivery_task: Some(
                openlark_hr::hire::hire::common_models::WebsiteDeliveryTaskResult {
                    delivery_task_id: Some("task_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(1),
                    error_message: None,
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_create_by_resume_response_serialization,
        website::delivery::create_by_resume::CreateByResumeResponse,
        website::delivery::create_by_resume::CreateByResumeResponse {
            delivery_task: Some(
                openlark_hr::hire::hire::common_models::WebsiteDeliveryTaskResult {
                    delivery_task_id: Some("task_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(1),
                    error_message: None,
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_task_get_response_serialization,
        website::delivery_task::get::GetResponse,
        website::delivery_task::get::GetResponse {
            delivery_task: openlark_hr::hire::hire::common_models::WebsiteDeliveryTaskResult {
                delivery_task_id: Some("task_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(2),
                error_message: Some("invalid resume".to_string()),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_website_job_post_get_response_serialization,
        website::job_post::get::GetResponse,
        website::job_post::get::GetResponse {
            job_post: Some(
                openlark_hr::hire::hire::common_models::WebsiteJobPostSummary {
                    job_post_id: Some("post_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    title: Some("后端工程师".to_string()),
                    active_status: Some(1),
                    job_channel_id: Some("channel_1".to_string()),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_job_post_list_response_serialization,
        website::job_post::list::ListResponse,
        website::job_post::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::WebsiteJobPostSummary {
                    job_post_id: Some("post_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    title: Some("后端工程师".to_string()),
                    active_status: Some(1),
                    job_channel_id: Some("channel_1".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_post".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_job_post_search_response_serialization,
        website::job_post::search::SearchResponse,
        website::job_post::search::SearchResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::WebsiteJobPostSummary {
                    job_post_id: Some("post_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    title: Some("后端工程师".to_string()),
                    active_status: Some(1),
                    job_channel_id: Some("channel_1".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_search".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_site_user_create_response_serialization,
        website::site_user::create::CreateResponse,
        website::site_user::create::CreateResponse {
            site_user: Some(
                openlark_hr::hire::hire::common_models::WebsiteSiteUserSummary {
                    site_user_id: Some("site_user_1".to_string()),
                    website_id: Some("site_1".to_string()),
                    user_id: Some("ou_1".to_string()),
                    email: Some("test@example.com".to_string()),
                    mobile: Some("13800000000".to_string()),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_batch_query_response_serialization,
        agency::batch_query::BatchQueryResponse,
        agency::batch_query::BatchQueryResponse {
            items: vec![openlark_hr::hire::hire::common_models::AgencySummary {
                agency_id: Some("agency_1".to_string()),
                name: Some("猎企 A".to_string()),
                code: Some("A001".to_string()),
                active_status: Some(1),
                status: None,
                extra: Default::default(),
            }],
            page_token: Some("cursor_agency".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_get_response_serialization,
        agency::get::GetResponse,
        agency::get::GetResponse {
            agency: Some(openlark_hr::hire::hire::common_models::AgencySummary {
                agency_id: Some("agency_1".to_string()),
                name: Some("猎企 A".to_string()),
                code: None,
                active_status: None,
                status: Some(1),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_get_account_response_serialization,
        agency::get_agency_account::GetAgencyAccountResponse,
        agency::get_agency_account::GetAgencyAccountResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::AgencyAccountSummary {
                    agency_account_id: Some("acc_1".to_string()),
                    agency_id: Some("agency_1".to_string()),
                    user_id: Some("ou_1".to_string()),
                    name: Some("猎头甲".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_acc".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_operate_account_response_serialization,
        agency::operate_agency_account::OperateAgencyAccountResponse,
        agency::operate_agency_account::OperateAgencyAccountResponse {
            agency_account_id: Some("acc_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_protect_response_serialization,
        agency::protect::ProtectResponse,
        agency::protect::ProtectResponse {
            protection_id: Some("protect_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_protect_search_response_serialization,
        agency::protect_search::ProtectSearchResponse,
        agency::protect_search::ProtectSearchResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::AgencyProtectionSummary {
                    protection_id: Some("protect_1".to_string()),
                    agency_id: Some("agency_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    expiration_time: Some("2026-12-31T00:00:00Z".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_protect".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_agency_query_response_serialization,
        agency::query::QueryResponse,
        agency::query::QueryResponse {
            items: vec![openlark_hr::hire::hire::common_models::AgencySummary {
                agency_id: Some("agency_1".to_string()),
                name: Some("猎企 A".to_string()),
                code: None,
                active_status: None,
                status: None,
                extra: Default::default(),
            }],
            page_token: Some("cursor_query".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_eco_account_custom_field_create_response_serialization,
        eco_account_custom_field::create::CreateResponse,
        eco_account_custom_field::create::CreateResponse {
            custom_field: Some(
                openlark_hr::hire::hire::common_models::EcoCustomFieldSummary {
                    custom_field_id: Some("field_1".to_string()),
                    name: Some("账号字段".to_string()),
                    code: Some("acct_code".to_string()),
                    status: Some(1),
                    active_status: None,
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_eco_account_custom_field_batch_update_response_serialization,
        eco_account_custom_field::batch_update::BatchUpdateResponse,
        eco_account_custom_field::batch_update::BatchUpdateResponse {
            operation: openlark_hr::hire::hire::common_models::EcoOperationResult {
                custom_field_id: Some("field_1".to_string()),
                package_id: None,
                paper_id: None,
                background_check_id: None,
                order_id: None,
                status: Some(1),
                progress: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_eco_background_check_custom_field_create_response_serialization,
        eco_background_check_custom_field::create::CreateResponse,
        eco_background_check_custom_field::create::CreateResponse {
            custom_field: Some(
                openlark_hr::hire::hire::common_models::EcoCustomFieldSummary {
                    custom_field_id: Some("bg_field_1".to_string()),
                    name: Some("背调字段".to_string()),
                    code: Some("bg_code".to_string()),
                    status: Some(1),
                    active_status: None,
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_eco_background_check_package_create_response_serialization,
        eco_background_check_package::create::CreateResponse,
        eco_background_check_package::create::CreateResponse {
            package: Some(
                openlark_hr::hire::hire::common_models::EcoBackgroundCheckPackageSummary {
                    package_id: Some("pkg_1".to_string()),
                    name: Some("标准背调套餐".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_eco_exam_paper_create_response_serialization,
        eco_exam_paper::create::CreateResponse,
        eco_exam_paper::create::CreateResponse {
            paper: Some(
                openlark_hr::hire::hire::common_models::EcoExamPaperSummary {
                    paper_id: Some("paper_1".to_string()),
                    name: Some("算法笔试".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_eco_background_check_cancel_response_serialization,
        eco_background_check::cancel::CancelResponse,
        eco_background_check::cancel::CancelResponse {
            operation: openlark_hr::hire::hire::common_models::EcoOperationResult {
                custom_field_id: None,
                package_id: None,
                paper_id: None,
                background_check_id: Some("bg_order_1".to_string()),
                order_id: Some("order_1".to_string()),
                status: Some(2),
                progress: Some(80),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_employee_get_response_serialization,
        employee::get::GetResponse,
        employee::get::GetResponse {
            employee: Some(openlark_hr::hire::hire::common_models::EmployeeSummary {
                employee_id: Some("emp_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(1),
                onboard_status: Some(2),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_employee_get_by_application_response_serialization,
        employee::get_by_application::GetByApplicationResponse,
        employee::get_by_application::GetByApplicationResponse {
            employee: Some(openlark_hr::hire::hire::common_models::EmployeeSummary {
                employee_id: Some("emp_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(1),
                onboard_status: Some(2),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_employee_patch_response_serialization,
        employee::patch::PatchResponse,
        employee::patch::PatchResponse {
            employee: Some(openlark_hr::hire::hire::common_models::EmployeeSummary {
                employee_id: Some("emp_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(2),
                onboard_status: Some(2),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_referral_account_deactivate_response_serialization,
        referral_account::deactivate::DeactivateResponse,
        referral_account::deactivate::DeactivateResponse {
            operation: openlark_hr::hire::hire::common_models::ReferralAccountOperationResult {
                account_id: Some("acc_1".to_string()),
                status: Some(2),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_referral_account_enable_response_serialization,
        referral_account::enable::EnableResponse,
        referral_account::enable::EnableResponse {
            operation: openlark_hr::hire::hire::common_models::ReferralAccountOperationResult {
                account_id: Some("acc_1".to_string()),
                status: Some(1),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_referral_account_withdraw_response_serialization,
        referral_account::withdraw::WithdrawResponse,
        referral_account::withdraw::WithdrawResponse {
            operation: openlark_hr::hire::hire::common_models::ReferralAccountOperationResult {
                account_id: Some("acc_1".to_string()),
                status: Some(3),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_external_interview_assessment_create_response_serialization,
        external_interview_assessment::create::CreateResponse,
        external_interview_assessment::create::CreateResponse {
            assessment: openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentResult {
                external_interview_assessment_id: Some("assess_1".to_string()),
                external_interview_id: Some("iv_1".to_string()),
                status: Some(1),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_external_interview_assessment_patch_response_serialization,
        external_interview_assessment::patch::PatchResponse,
        external_interview_assessment::patch::PatchResponse {
            assessment: openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentResult {
                external_interview_assessment_id: Some("assess_1".to_string()),
                external_interview_id: Some("iv_1".to_string()),
                status: Some(2),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_interview_list_response_serialization,
        interview::list::ListResponse,
        interview::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::InterviewSummary {
                interview_id: Some("iv_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(2),
                interview_round_name: Some("一面".to_string()),
                start_time: Some("2026-05-01T10:00:00Z".to_string()),
                extra: Default::default(),
            }],
            page_token: Some("cursor_interview".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_get_by_talent_response_serialization,
        interview::get_by_talent::GetByTalentResponse,
        interview::get_by_talent::GetByTalentResponse {
            interview: Some(openlark_hr::hire::hire::common_models::InterviewSummary {
                interview_id: Some("iv_1".to_string()),
                application_id: Some("app_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                status: Some(2),
                interview_round_name: Some("一面".to_string()),
                start_time: Some("2026-05-01T10:00:00Z".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_background_check_order_batch_query_response_serialization,
        background_check_order::batch_query::BatchQueryResponse,
        background_check_order::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::BackgroundCheckOrderSummary {
                    order_id: Some("order_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(2),
                    vendor_name: Some("VendorA".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_order".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_background_check_order_list_response_serialization,
        background_check_order::list::ListResponse,
        background_check_order::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::BackgroundCheckOrderSummary {
                    order_id: Some("order_1".to_string()),
                    application_id: Some("app_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    status: Some(2),
                    vendor_name: Some("VendorA".to_string()),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_order".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_referral_reward_create_response_serialization,
        external_referral_reward::create::CreateResponse,
        external_referral_reward::create::CreateResponse {
            reward: openlark_hr::hire::hire::common_models::ExternalReferralRewardResult {
                external_referral_reward_id: Some("reward_1".to_string()),
                account_id: Some("acc_1".to_string()),
                amount: Some(88.5),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_external_referral_reward_delete_response_serialization,
        external_referral_reward::delete::DeleteResponse,
        external_referral_reward::delete::DeleteResponse {
            reward: openlark_hr::hire::hire::common_models::ExternalReferralRewardResult {
                external_referral_reward_id: Some("reward_1".to_string()),
                account_id: Some("acc_1".to_string()),
                amount: None,
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_talent_pool_batch_change_response_serialization,
        talent_pool::batch_change_talent_pool::BatchChangeTalentPoolResponse,
        talent_pool::batch_change_talent_pool::BatchChangeTalentPoolResponse {
            operation: openlark_hr::hire::hire::common_models::TalentPoolOperationResult {
                talent_pool_id: Some("pool_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_talent_pool_move_talent_response_serialization,
        talent_pool::move_talent::MoveTalentResponse,
        talent_pool::move_talent::MoveTalentResponse {
            operation: openlark_hr::hire::hire::common_models::TalentPoolOperationResult {
                talent_pool_id: Some("pool_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_eco_exam_login_info_response_serialization,
        eco_exam::login_info::LoginInfoResponse,
        eco_exam::login_info::LoginInfoResponse {
            exam: openlark_hr::hire::hire::common_models::EcoExamOperationResult {
                exam_id: Some("exam_1".to_string()),
                status: Some(1),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_eco_exam_update_result_response_serialization,
        eco_exam::update_result::UpdateResultResponse,
        eco_exam::update_result::UpdateResultResponse {
            exam: openlark_hr::hire::hire::common_models::EcoExamOperationResult {
                exam_id: Some("exam_1".to_string()),
                status: Some(2),
                result: Some(true),
                success: Some(true),
                extra: Default::default(),
            },
        }
    );
    roundtrip_eq!(
        test_talent_add_to_folder_response_serialization,
        talent::add_to_folder::AddToFolderResponse,
        talent::add_to_folder::AddToFolderResponse {
            talent_id: Some("talent_1".to_string()),
            folder_id: Some("folder_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_remove_to_folder_response_serialization,
        talent::remove_to_folder::RemoveToFolderResponse,
        talent::remove_to_folder::RemoveToFolderResponse {
            talent_id: Some("talent_1".to_string()),
            folder_id: Some("folder_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_onboard_status_response_serialization,
        talent::onboard_status::OnboardStatusResponse,
        talent::onboard_status::OnboardStatusResponse {
            talent_id: Some("talent_1".to_string()),
            onboard_status: Some(2),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_tag_response_serialization,
        talent::tag::TagResponse,
        talent::tag::TagResponse {
            talent_id: Some("talent_1".to_string()),
            tag_ids: Some(vec!["tag_1".to_string(), "tag_2".to_string()]),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_external_info_create_response_serialization,
        talent::external_info::create::CreateResponse,
        talent::external_info::create::CreateResponse {
            external_info: Some(
                openlark_hr::hire::hire::common_models::TalentExternalInfoRecord {
                    external_info_id: Some("ext_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    source_name: Some("Boss".to_string()),
                    external_id: Some("boss_1".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_external_info_update_response_serialization,
        talent::external_info::update::UpdateResponse,
        talent::external_info::update::UpdateResponse {
            external_info: Some(
                openlark_hr::hire::hire::common_models::TalentExternalInfoRecord {
                    external_info_id: Some("ext_1".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    source_name: Some("Boss".to_string()),
                    external_id: Some("boss_1".to_string()),
                    status: Some(2),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_requirement_create_response_serialization,
        job_requirement::create::CreateResponse,
        job_requirement::create::CreateResponse {
            job_requirement_id: Some("jr_1".to_string()),
            job_id: Some("job_1".to_string()),
            title: Some("招聘需求 A".to_string()),
            status: Some(1),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_requirement_delete_response_serialization,
        job_requirement::delete::DeleteResponse,
        job_requirement::delete::DeleteResponse {
            job_requirement_id: Some("jr_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_requirement_list_response_serialization,
        job_requirement::list::ListResponse,
        job_requirement::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::JobRequirementSummary {
                    job_requirement_id: Some("jr_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    title: Some("招聘需求 A".to_string()),
                    name: None,
                    status: Some(1),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_jr".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_requirement_list_by_id_response_serialization,
        job_requirement::list_by_id::ListByIdResponse,
        job_requirement::list_by_id::ListByIdResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::JobRequirementSummary {
                    job_requirement_id: Some("jr_1".to_string()),
                    job_id: Some("job_1".to_string()),
                    title: None,
                    name: Some("需求 A".to_string()),
                    status: Some(1),
                    extra: Default::default(),
                }
            ],
            page_token: Some("cursor_jr_id".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_requirement_update_response_serialization,
        job_requirement::update::UpdateResponse,
        job_requirement::update::UpdateResponse {
            job_requirement_id: Some("jr_1".to_string()),
            result: Some(true),
            success: Some(true),
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_evaluation_list_response_serialization,
        evaluation::list::ListResponse,
        evaluation::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_evaluation_task_list_response_serialization,
        evaluation_task::list::ListResponse,
        evaluation_task::list::ListResponse {
            items: vec![],
            page_token: None,
            has_more: None,
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_feedback_form_list_response_serialization,
        interview_feedback_form::list::ListResponse,
        interview_feedback_form::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::CatalogItem {
                id: Some("form_1".to_string()),
                title: Some(openlark_hr::hire::hire::common_models::FlexibleText::I18n(
                    openlark_hr::hire::hire::common_models::I18nText {
                        zh_cn: Some("通用评价表".to_string()),
                        en_us: Some("Default Form".to_string()),
                        extra: Default::default(),
                    },
                )),
                ..Default::default()
            }],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );

    roundtrip_eq!(
        test_note_create_response_serialization,
        note::create::CreateResponse,
        note::create::CreateResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_get_response_serialization,
        note::get::GetResponse,
        note::get::GetResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_list_response_serialization,
        note::list::ListResponse,
        note::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("备注".to_string()),
                ..Default::default()
            }],
            has_more: Some(false),
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_note_patch_response_serialization,
        note::patch::PatchResponse,
        note::patch::PatchResponse {
            note: Some(openlark_hr::hire::hire::common_models::NoteRecord {
                id: Some("note_1".to_string()),
                content: Some("更新备注".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_note_delete_response_serialization,
        note::delete::DeleteResponse,
        note::delete::DeleteResponse {}
    );

    roundtrip_eq!(
        test_offer_application_form_get_response_serialization,
        offer_application_form::get::GetResponse,
        offer_application_form::get::GetResponse {
            offer_apply_form: Some(offer_application_form::get::OfferApplyForm {
                id: Some("form_1".to_string()),
                name: None,
                schema: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_application_form_list_response_serialization,
        offer_application_form::list::ListResponse,
        offer_application_form::list::ListResponse {
            has_more: Some(false),
            page_token: None,
            items: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_schema_get_response_serialization,
        offer_schema::get::GetResponse,
        offer_schema::get::GetResponse {
            id: Some("schema_1".to_string()),
            scenario: Some(1),
            version: Some(121),
            object_list: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_advertisement_publish_response_serialization,
        advertisement::publish::PublishResponse,
        advertisement::publish::PublishResponse {
            operation: openlark_hr::hire::hire::common_models::GenericOperationResult {
                publish_id: Some("pub_001".to_string()),
                status: Some(1),
                success: Some(true),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_attachment_create_response_serialization,
        attachment::create::CreateResponse,
        attachment::create::CreateResponse {
            id: Some("att_001".to_string()),
            name: Some("resume.pdf".to_string()),
            url: Some("https://example.com/file".to_string()),
        }
    );
    roundtrip_eq!(
        test_attachment_get_response_serialization,
        attachment::get::GetResponse,
        attachment::get::GetResponse {
            attachment: Some(openlark_hr::hire::hire::common_models::HireAttachment {
                id: Some("att_001".to_string()),
                name: Some("resume.pdf".to_string()),
                ..Default::default()
            })
        }
    );
    roundtrip_eq!(
        test_attachment_preview_response_serialization,
        attachment::preview::PreviewResponse,
        attachment::preview::PreviewResponse {
            url: Some("https://example.com/file".to_string()),
        }
    );
    roundtrip_eq!(
        test_referral_search_response_serialization,
        referral::search::SearchResponse,
        referral::search::SearchResponse {
            items: vec![],
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_role_get_response_serialization,
        role::get::GetResponse,
        role::get::GetResponse {
            role: Some(role::list::RoleItem {
                id: Some("role_1".to_string()),
                ..Default::default()
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_referral_get_by_application_response_serialization,
        referral::get_by_application::GetByApplicationResponse,
        referral::get_by_application::GetByApplicationResponse {
            referral: Some(referral::search::ReferralItem {
                id: Some("ref_1".to_string()),
                ..Default::default()
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_referral_website_job_post_get_response_serialization,
        referral_website::job_post::get::GetResponse,
        referral_website::job_post::get::GetResponse {
            job_post: Some(
                referral_website::job_post::list::ReferralWebsiteJobPostItem {
                    id: Some("job_post_1".to_string()),
                    ..Default::default()
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_manager_get_response_serialization,
        job::manager::get::GetResponse,
        job::manager::get::GetResponse {
            manager: Some(openlark_hr::hire::hire::common_models::JobRecruiterRecord {
                manager_id: Some("mgr_1".to_string()),
                ..Default::default()
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_offer_custom_field_update_response_serialization,
        offer_custom_field::update::UpdateResponse,
        offer_custom_field::update::UpdateResponse {
            operation: openlark_hr::hire::hire::common_models::OfferCustomFieldOperationResult {
                offer_custom_field_id: Some("field_1".to_string()),
                success: Some(true),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_interviewer_patch_response_serialization,
        interviewer::patch::PatchResponse,
        interviewer::patch::PatchResponse {
            interviewer: openlark_hr::hire::hire::common_models::InterviewerOperationResult {
                interviewer_id: Some("interviewer_1".to_string()),
                verify_status: Some(2),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_job_manager_batch_update_response_serialization,
        job::manager::batch_update::BatchUpdateResponse,
        job::manager::batch_update::BatchUpdateResponse {
            operation: openlark_hr::hire::hire::common_models::JobManagerOperationResult {
                job_id: Some("job_1".to_string()),
                manager_id: Some("mgr_1".to_string()),
                success: Some(true),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_job_type_list_response_serialization,
        job_type::list::ListResponse,
        job_type::list::ListResponse {
            items: vec![openlark_hr::hire::hire::common_models::CatalogItem {
                id: Some("type_1".to_string()),
                name: Some(openlark_hr::hire::hire::common_models::FlexibleText::Plain(
                    "研发".to_string(),
                )),
                ..Default::default()
            }],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_interview_task_list_response_serialization,
        interview_task::list::ListResponse,
        interview_task::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::InterviewTaskSummary {
                    interview_id: Some("interview_1".to_string()),
                    status: Some(2),
                    ..Default::default()
                }
            ],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_talent_operation_log_search_response_serialization,
        talent_operation_log::search::SearchResponse,
        talent_operation_log::search::SearchResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::TalentOperationLogEntry {
                    id: Some("log_1".to_string()),
                    operation_type: Some("create".to_string()),
                    ..Default::default()
                }
            ],
            page_token: None,
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_ehr_import_task_patch_response_serialization,
        ehr_import_task::patch::PatchResponse,
        ehr_import_task::patch::PatchResponse {
            operation: openlark_hr::hire::hire::common_models::GenericOperationResult {
                task_id: Some("task_1".to_string()),
                success: Some(true),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_exam_create_response_serialization,
        exam::create::CreateResponse,
        exam::create::CreateResponse {
            operation: openlark_hr::hire::hire::common_models::GenericOperationResult {
                exam_id: Some("exam_1".to_string()),
                application_id: Some("app_1".to_string()),
                success: Some(true),
                ..Default::default()
            }
        }
    );
    roundtrip_eq!(
        test_talent_blocklist_change_response_serialization,
        talent_blocklist::change_talent_block::ChangeTalentBlockResponse,
        talent_blocklist::change_talent_block::ChangeTalentBlockResponse {
            operation: openlark_hr::hire::hire::common_models::GenericOperationResult {
                talent_id: Some("talent_1".to_string()),
                success: Some(true),
                ..Default::default()
            }
        }
    );
}
