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
        job::combined_create::CombinedCreateRequest::new(
            test_config("https://open.feishu.cn"),
            job::combined_create::CombinedCreateRequestBody::new(
                "后端研发",
                job::combined_create::JobManager {
                    recruiter_id: "ou_efk39117c300506837def50545420c6a".to_string(),
                    hiring_manager_id_list: vec!["ou_808dad5b8fb21416b4ff242a5c616016".to_string()],
                    ..job::combined_create::JobManager::default()
                },
                "6960663240925956554",
                1,
                "od-b2fafdce6fc5800b574ba5b0e2798b36",
                false,
                "6960663240925956551",
                "102",
            ),
        )
    );
    smoke_builder!(
        test_job_get_builder,
        job::get::GetRequest::new(test_config("https://open.feishu.cn"))
    );
    smoke_builder!(
        test_job_combined_update_builder,
        job::combined_update::CombinedUpdateRequest::new(
            test_config("https://open.feishu.cn"),
            job::combined_update::CombinedUpdateRequestBody::new(
                job::combined_create::JobManager {
                    recruiter_id: "ou_efk39117c300506837def50545420c6a".to_string(),
                    hiring_manager_id_list: vec!["ou_808dad5b8fb21416b4ff242a5c616016".to_string()],
                    ..job::combined_create::JobManager::default()
                },
                false,
                "6960663240925956551",
            ),
        )
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
            interviewer: Some(openlark_hr::common::shared_models::IdNameObject {
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
            id: Some("ap_001".to_string()),
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
            application: None,
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
            application_detail: Some(application::get_detail::ApplicationDetailInfo {
                basic_info: Some(application::get_detail::ApplicationDetailBasicInfo {
                    id: Some("ap_001".to_string()),
                    job_id: Some("job_001".to_string()),
                    talent_id: Some("talent_001".to_string()),
                    stage: Some(application::get_detail::ApplicationStageInfo {
                        id: Some("stage_offer".to_string()),
                        zh_name: Some("Offer".to_string()),
                        en_name: Some("Offer".to_string()),
                        stage_type: Some(5),
                        extra: Default::default(),
                    }),
                    active_status: Some(1),
                    delivery_type: Some(1),
                    resume_source_info: None,
                    website_resume_source: None,
                    talent_attachment_resume_id: None,
                    stage_time_list: None,
                    onboard_status: Some(1),
                    application_preferred_city_list: None,
                    termination_reason: None,
                    creator_id: None,
                    owner_id: None,
                    terminator_id: None,
                    create_time: None,
                    modify_time: None,
                    lock_status: None,
                    termination_reason_note: None,
                    extra: Default::default(),
                }),
                job: Some(application::get_detail::ApplicationDetailJobInfo {
                    id: Some("job_001".to_string()),
                    name: Some("后端工程师".to_string()),
                    code: None,
                    description: None,
                    requirement: None,
                    recruitment_type: None,
                    department: None,
                    process_type: None,
                    process_id: None,
                    city_list: None,
                    extra: Default::default(),
                }),
                talent: Some(application::get_detail::TalentBasicInfoV2 {
                    id: Some("talent_001".to_string()),
                    name: Some("张三".to_string()),
                    mobile_code: None,
                    mobile_number: None,
                    email: Some("zhangsan@example.com".to_string()),
                    extra: Default::default(),
                }),
                evaluations: None,
                interview_aggregation: None,
                offer: Some(application::get_detail::OfferInfoV2 {
                    offer_basic: Some(application::get_detail::OfferBasicInfoV2 {
                        id: Some("offer_001".to_string()),
                        offer_status: Some(10),
                        leader: None,
                        employee_type: None,
                        department: None,
                        sequence: None,
                        level: None,
                        company_main_body: None,
                        job_requirement_id: None,
                        probation_month: None,
                        contract_period: None,
                        onboard_date: None,
                        owner: None,
                        onboard_address: None,
                        work_address: None,
                        remark: None,
                        attachment_list: None,
                        customize_info_list: None,
                        create_time: None,
                        extra: Default::default(),
                    }),
                    offer_salary: None,
                    extra: Default::default(),
                }),
                employee: None,
                agency: None,
                portal: None,
                referral: None,
                extra: Default::default(),
            }),
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
            employee: None,
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
                openlark_hr::hire::hire::common_models::ExternalBackgroundCheck {
                    id: Some("bg_1".to_string()),
                    external_application_id: Some("app_1".to_string()),
                    date: Some(1626602069393),
                    name: Some("张三的背调".to_string()),
                    result: Some("已通过".to_string()),
                    attachment_list: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalBackgroundCheckAttachment {
                            id: Some("attach_1".to_string()),
                            name: Some("test_resume.pdf".to_string()),
                            size: Some(126371),
                            extra: Default::default(),
                        }
                    ]),
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
            external_background_check: Some(
                openlark_hr::hire::hire::common_models::ExternalBackgroundCheck {
                    id: Some("bg_1".to_string()),
                    external_application_id: Some("app_1".to_string()),
                    date: Some(1626602069393),
                    name: Some("张三的背调".to_string()),
                    result: Some("已通过".to_string()),
                    attachment_list: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalBackgroundCheckAttachment {
                            id: Some("attach_1".to_string()),
                            name: Some("test_resume.pdf".to_string()),
                            size: Some(126371),
                            extra: Default::default(),
                        }
                    ]),
                    extra: Default::default(),
                }
            ),
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
            external_background_check: Some(
                openlark_hr::hire::hire::common_models::ExternalBackgroundCheck {
                    id: Some("bg_1".to_string()),
                    external_application_id: Some("app_1".to_string()),
                    date: Some(1626602069393),
                    name: Some("张三的背调".to_string()),
                    result: Some("已通过".to_string()),
                    attachment_list: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalBackgroundCheckAttachment {
                            id: Some("attach_1".to_string()),
                            name: Some("test_resume.pdf".to_string()),
                            size: Some(2827182),
                            extra: Default::default(),
                        }
                    ]),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_interview_batch_query_response_serialization,
        external_interview::batch_query::BatchQueryResponse,
        external_interview::batch_query::BatchQueryResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalInterview {
                    external_application_id: Some("app_1".to_string()),
                    id: Some("iv_1".to_string()),
                    participate_status: Some(2),
                    begin_time: Some(1618500278638),
                    end_time: Some(1618500278639),
                    interview_assessments: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalInterviewAssessment {
                            id: Some("assess_1".to_string()),
                            username: Some("张三".to_string()),
                            conclusion: Some(2),
                            assessment_dimension_list: Some(vec![
                                openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentDimension {
                                    score: Some(99),
                                    option: Some("选项1".to_string()),
                                    options: Some(vec!["选项1".to_string()]),
                                    content: Some("这是一个测试内容".to_string()),
                                    assessment_type: Some(1),
                                    title: Some("心理测试".to_string()),
                                    description: Some("心理测试题目".to_string()),
                                    extra: Default::default(),
                                }
                            ]),
                            content: Some("这是一个心理测试题目".to_string()),
                            external_interview_id: None,
                            extra: Default::default(),
                        }
                    ]),
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
            external_interview: Some(
                openlark_hr::hire::hire::common_models::ExternalInterview {
                    external_application_id: Some("app_1".to_string()),
                    id: Some("iv_1".to_string()),
                    participate_status: Some(1),
                    begin_time: Some(1618500278638),
                    end_time: Some(1618500278639),
                    interview_assessments: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalInterviewAssessment {
                            id: Some("assess_1".to_string()),
                            username: Some("张三".to_string()),
                            conclusion: Some(1),
                            assessment_dimension_list: Some(vec![
                                openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentDimension {
                                    score: Some(99),
                                    option: None,
                                    options: None,
                                    content: Some("不错的面试内容".to_string()),
                                    assessment_type: Some(1),
                                    title: Some("面试测试".to_string()),
                                    description: Some("这是一次测试".to_string()),
                                    extra: Default::default(),
                                }
                            ]),
                            content: Some("第一次面试".to_string()),
                            external_interview_id: None,
                            extra: Default::default(),
                        }
                    ]),
                    extra: Default::default(),
                }
            ),
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
            external_interview: Some(
                openlark_hr::hire::hire::common_models::ExternalInterview {
                    external_application_id: Some("app_1".to_string()),
                    id: Some("iv_1".to_string()),
                    participate_status: Some(1),
                    begin_time: Some(1618500278638),
                    end_time: Some(1618500278639),
                    interview_assessments: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalInterviewAssessment {
                            id: Some("assess_1".to_string()),
                            username: Some("张三".to_string()),
                            conclusion: Some(1),
                            assessment_dimension_list: Some(vec![
                                openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentDimension {
                                    score: Some(99),
                                    option: Some("选项1".to_string()),
                                    options: Some(vec!["选项1".to_string()]),
                                    content: Some("面试内容结果补充".to_string()),
                                    assessment_type: Some(1),
                                    title: Some("心理测试".to_string()),
                                    description: Some("心理测试描述".to_string()),
                                    extra: Default::default(),
                                }
                            ]),
                            content: Some("面试内容结果补充".to_string()),
                            external_interview_id: None,
                            extra: Default::default(),
                        }
                    ]),
                    extra: Default::default(),
                }
            ),
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
            id: Some("agree_1".to_string()),
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
            items: vec![tripartite_agreement::list::TripartiteAgreementInfo {
                id: Some("agree_1".to_string()),
                application_id: Some("app_1".to_string()),
                state: Some(1),
                create_time: Some("1698292282660".to_string()),
                modify_time: Some("1698292282661".to_string()),
                extra: Default::default(),
            }],
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
            default_job_post: Some(job::combined_create::DefaultJobPost {
                id: Some("post_001".to_string()),
                extra: Default::default(),
            }),
            job: Some(job::models::JobDetail {
                id: Some("job_001".to_string()),
                title: Some("后端工程师".to_string()),
                description: None,
                code: None,
                requirement: None,
                active_status: Some(1),
                recruitment_type: None,
                department: None,
                city: None,
                min_job_level: None,
                max_job_level: None,
                highlight_list: None,
                job_category: None,
                job_type: None,
                create_user_id: None,
                create_time: None,
                update_time: None,
                process_type: None,
                process_id: None,
                process_name: None,
                process_en_name: None,
                customized_data_list: None,
                job_function: None,
                subject: None,
                head_count: None,
                experience: None,
                expiry_time: None,
                min_salary: None,
                max_salary: None,
                required_degree: None,
                city_list: None,
                job_attribute: None,
                create_timestamp: None,
                update_timestamp: None,
                expiry_timestamp: None,
                target_major_list: None,
                job_manager: None,
                interview_registration_schema_info: None,
                onboard_registration_schema_info: None,
                portal_website_apply_form_schema_info: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_get_response_serialization,
        job::get::GetResponse,
        job::get::GetResponse {
            job: None,
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
            job_detail: Some(job::get_detail::JobDetailInfo {
                basic_info: Some(job::get_detail::JobDetailBasicInfo {
                    id: Some("job_001".to_string()),
                    title: Some("后端工程师".to_string()),
                    description: Some("负责核心服务开发".to_string()),
                    code: Some("R18".to_string()),
                    requirement: None,
                    recruitment_type: None,
                    department: Some(openlark_hr::common::shared_models::IdNameObject {
                        id: Some("dept_1".to_string()),
                        name: Some(openlark_hr::common::shared_models::I18nText {
                            zh_cn: Some("飞书".to_string()),
                            en_us: Some("Lark".to_string()),
                            extra: Default::default(),
                        }),
                        extra: Default::default(),
                    }),
                    min_job_level: None,
                    max_job_level: None,
                    highlight_list: None,
                    job_category: None,
                    job_type: None,
                    active_status: Some(1),
                    creator_id: None,
                    create_time: None,
                    update_time: None,
                    process_type: Some(1),
                    process_id: Some("process_1".to_string()),
                    process_name: None,
                    customized_data_list: None,
                    job_function: None,
                    subject: None,
                    head_count: Some(1),
                    experience: None,
                    expiry_time: None,
                    min_salary: None,
                    max_salary: None,
                    required_degree: None,
                    city_list: None,
                    job_attribute: None,
                    target_major_list: None,
                    storefront_mode: Some(1),
                    extra: Default::default(),
                }),
                recruiter: Some(job::get_detail::JobUserInfo {
                    id: Some("ou_1".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("招聘负责人".to_string()),
                        en_us: Some("Recruiter".to_string()),
                        extra: Default::default(),
                    }),
                    extra: Default::default(),
                }),
                assistant_list: None,
                hiring_manager_list: None,
                job_requirement_list: Some(vec![job::get_detail::JobRequirementSimple {
                    id: Some("req_1".to_string()),
                    short_code: Some("2232".to_string()),
                    name: Some("华中大区部门".to_string()),
                    department_id: Some("dept_1".to_string()),
                    extra: Default::default(),
                }]),
                address_list: None,
                job_config: None,
                storefront_list: None,
                tag_list: Some(vec![job::get_detail::JobDetailTag {
                    id: Some("tag_1".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("985、211".to_string()),
                        en_us: Some("985、211".to_string()),
                        extra: Default::default(),
                    }),
                    order: Some(1),
                    extra: Default::default(),
                }]),
                extra: Default::default(),
            }),
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
            default_job_post: Some(job::combined_update::DefaultJobPostRef {
                id: Some("6960663240925956568".to_string()),
                extra: Default::default(),
            }),
            job: Some(job::combined_update::CombinedJob {
                id: Some("7281257045172308287".to_string()),
                title: Some("测试职位".to_string()),
                head_count: Some(100),
                experience: Some(1),
                ..Default::default()
            }),
            job_manager: Some(job::combined_update::JobManagerView {
                id: Some("1618209327096".to_string()),
                recruiter_id: Some("ou_efk39117c300506837def50545420c6a".to_string()),
                hiring_manager_id_list: Some(vec![
                    "on_94a1ee5551019f18cd73d9f111898cf2".to_string()
                ]),
                assistant_id_list: None,
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_job_list_response_serialization,
        job::list::ListResponse,
        job::list::ListResponse {
            items: vec![job::models::JobDetail {
                id: Some("job_001".to_string()),
                title: Some("后端工程师".to_string()),
                description: Some("负责核心服务开发".to_string()),
                code: Some("R18".to_string()),
                requirement: None,
                active_status: Some(1),
                recruitment_type: None,
                department: None,
                city: None,
                min_job_level: None,
                max_job_level: None,
                highlight_list: None,
                job_category: None,
                job_type: None,
                create_user_id: None,
                create_time: None,
                update_time: None,
                process_type: None,
                process_id: None,
                process_name: None,
                process_en_name: None,
                customized_data_list: None,
                job_function: None,
                subject: None,
                head_count: None,
                experience: None,
                expiry_time: None,
                min_salary: None,
                max_salary: None,
                required_degree: None,
                city_list: None,
                job_attribute: None,
                create_timestamp: None,
                update_timestamp: None,
                expiry_timestamp: None,
                target_major_list: None,
                job_manager: None,
                interview_registration_schema_info: None,
                onboard_registration_schema_info: None,
                portal_website_apply_form_schema_info: None,
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
            job_config: Some(job::config::JobConfigResult {
                id: Some("job_001".to_string()),
                offer_apply_schema: Some(openlark_hr::common::shared_models::IdNameObject {
                    id: Some("form_offer".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("申请表1".to_string()),
                        en_us: Some("Offer Apply 1".to_string()),
                        extra: Default::default(),
                    }),
                    extra: Default::default(),
                }),
                recommended_evaluator_list: Some(vec![
                    openlark_hr::common::shared_models::IdNameObject {
                        id: Some("ou_1".to_string()),
                        name: Some(openlark_hr::common::shared_models::I18nText {
                            zh_cn: Some("张三".to_string()),
                            en_us: Some("ZhangSan".to_string()),
                            extra: Default::default(),
                        }),
                        extra: Default::default(),
                    }
                ]),
                job_attribute: Some(1),
                interview_registration: Some(job::config::RegistrationInfo {
                    schema_id: Some("schema_interview".to_string()),
                    name: Some("默认登记表".to_string()),
                    extra: Default::default(),
                }),
                ..Default::default()
            }),
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
            info: Some(job::recruiter::JobRecruiterInfo {
                id: Some("job_001".to_string()),
                recruiter_id: Some("rec_1".to_string()),
                hiring_manager_id_list: Some(vec!["mgr_1".to_string()]),
                assistant_id_list: None,
                extra: Default::default(),
            }),
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
            job_config: Some(job::update_config::JobConfigResult {
                offer_apply_schema: Some(openlark_hr::common::shared_models::IdNameObject {
                    id: Some("7392444731470563625".to_string()),
                    ..Default::default()
                }),
                id: Some("6960663240925956574".to_string()),
                interview_registration: Some(job::update_config::RegistrationInfo {
                    schema_id: Some("6930815272790114324".to_string()),
                    name: Some("默认登记表".to_string()),
                    extra: Default::default(),
                }),
                job_attribute: Some(1),
                ..Default::default()
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_create_response_serialization,
        external_application::create::CreateResponse,
        external_application::create::CreateResponse {
            external_application: Some(
                openlark_hr::hire::hire::common_models::ExternalApplication {
                    id: Some("ext_app_1".to_string()),
                    job_recruitment_type: Some(1),
                    job_title: Some("高级 Java".to_string()),
                    resume_source: Some("内推".to_string()),
                    stage: Some("简历初筛".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    termination_reason: None,
                    delivery_type: Some(1),
                    modify_time: Some(1618500278645),
                    create_time: Some(1618500278644),
                    termination_type: None,
                    extra: Default::default(),
                }
            ),
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
            external_application: Some(
                openlark_hr::hire::hire::common_models::ExternalApplication {
                    id: Some("ext_app_1".to_string()),
                    job_recruitment_type: Some(1),
                    job_title: Some("高级Java".to_string()),
                    resume_source: Some("lagou".to_string()),
                    stage: Some("简历初筛".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    termination_reason: Some("不合适".to_string()),
                    delivery_type: Some(1),
                    modify_time: Some(1618500278645),
                    create_time: None,
                    termination_type: Some("HR 主动终止".to_string()),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_application_list_response_serialization,
        external_application::list::ListResponse,
        external_application::list::ListResponse {
            items: vec![
                openlark_hr::hire::hire::common_models::ExternalApplication {
                    id: Some("ext_app_1".to_string()),
                    job_recruitment_type: Some(1),
                    job_title: Some("高级Java".to_string()),
                    resume_source: Some("Boss".to_string()),
                    stage: Some("简历初筛".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    termination_reason: None,
                    delivery_type: Some(2),
                    modify_time: Some(1618500278645),
                    create_time: Some(1618500278644),
                    termination_type: None,
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
            external_application: Some(
                openlark_hr::hire::hire::common_models::ExternalApplication {
                    id: Some("ext_app_1".to_string()),
                    job_recruitment_type: Some(1),
                    job_title: Some("高级Java".to_string()),
                    resume_source: Some("内推".to_string()),
                    stage: Some("简历初筛".to_string()),
                    talent_id: Some("talent_1".to_string()),
                    termination_reason: None,
                    delivery_type: Some(1),
                    modify_time: Some(1618500278645),
                    create_time: Some(1618500278644),
                    termination_type: None,
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_create_response_serialization,
        external_offer::create::CreateResponse,
        external_offer::create::CreateResponse {
            external_offer: Some(openlark_hr::hire::hire::common_models::ExternalOffer {
                id: Some("ext_offer_1".to_string()),
                external_application_id: Some("app_1".to_string()),
                biz_create_time: Some("1721899352428".to_string()),
                owner: Some("张三".to_string()),
                offer_status: Some("已发送".to_string()),
                attachment_list: Some(vec![
                    openlark_hr::hire::hire::common_models::ExternalCommonAttachment {
                        id: Some("attach_1".to_string()),
                        name: Some("test_resume.pdf".to_string()),
                        size: Some(126371),
                        extra: Default::default(),
                    }
                ]),
                extra: Default::default(),
            }),
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
            items: vec![openlark_hr::hire::hire::common_models::ExternalOffer {
                id: Some("ext_offer_1".to_string()),
                external_application_id: Some("app_1".to_string()),
                biz_create_time: Some("1721899352428".to_string()),
                owner: Some("张三".to_string()),
                offer_status: Some("已发送".to_string()),
                attachment_list: Some(vec![
                    openlark_hr::hire::hire::common_models::ExternalCommonAttachment {
                        id: Some("attach_1".to_string()),
                        name: Some("test_resume.pdf".to_string()),
                        size: Some(126371),
                        extra: Default::default(),
                    }
                ]),
                extra: Default::default(),
            }],
            page_token: Some("cursor_ext_offer".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_external_offer_update_response_serialization,
        external_offer::update::UpdateResponse,
        external_offer::update::UpdateResponse {
            external_offer: Some(openlark_hr::hire::hire::common_models::ExternalOffer {
                id: Some("ext_offer_1".to_string()),
                external_application_id: Some("app_1".to_string()),
                biz_create_time: Some("1721899352428".to_string()),
                owner: Some("张三".to_string()),
                offer_status: Some("已发送".to_string()),
                attachment_list: Some(vec![
                    openlark_hr::hire::hire::common_models::ExternalCommonAttachment {
                        id: Some("attach_1".to_string()),
                        name: Some("test_resume.pdf".to_string()),
                        size: Some(126371),
                        extra: Default::default(),
                    }
                ]),
                extra: Default::default(),
            }),
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
            id: Some("7085989097067563300".to_string()),
            name: Some("官网微信渠道".to_string()),
            link: Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5".to_string()),
            code: Some("A1KM6A5".to_string()),
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
            items: vec![openlark_hr::hire::hire::common_models::WebsiteChannelInfo {
                id: Some("7085989097067563300".to_string()),
                name: Some("微信推广渠道".to_string()),
                link: Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5".to_string()),
                code: Some("A1KM6A5".to_string()),
                extra: Default::default(),
            }],
            page_token: Some("cursor_channel".to_string()),
            has_more: Some(false),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_channel_update_response_serialization,
        website::channel::update::UpdateResponse,
        website::channel::update::UpdateResponse {
            id: Some("7085989097067563300".to_string()),
            name: Some("微信推广渠道".to_string()),
            link: Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5".to_string()),
            code: Some("A1KM6A5".to_string()),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_create_by_attachment_response_serialization,
        website::delivery::create_by_attachment::CreateByAttachmentResponse,
        website::delivery::create_by_attachment::CreateByAttachmentResponse {
            task_id: Some("f1c2a0f138ec492d99d7ab73594158ad".to_string()),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_create_by_resume_response_serialization,
        website::delivery::create_by_resume::CreateByResumeResponse,
        website::delivery::create_by_resume::CreateByResumeResponse {
            delivery: Some(openlark_hr::hire::hire::common_models::WebsiteDeliveryDto {
                application_id: Some("app_1".to_string()),
                id: Some("delivery_1".to_string()),
                job_id: Some("job_1".to_string()),
                job_post_id: Some("post_1".to_string()),
                portal_resume_id: Some("resume_1".to_string()),
                user_id: Some("user_1".to_string()),
                talent_id: Some("talent_1".to_string()),
                extra: Default::default(),
            }),
            extra: Default::default(),
        }
    );
    roundtrip_eq!(
        test_website_delivery_task_get_response_serialization,
        website::delivery_task::get::GetResponse,
        website::delivery_task::get::GetResponse {
            status: Some(2),
            delivery: Some(website::delivery_task::get::WebsiteDeliveryDto {
                application_id: Some("6960663240925956657".to_string()),
                id: Some("6960663240925956655".to_string()),
                job_id: Some("6960663240925956659".to_string()),
                job_post_id: Some("6960663240925956658".to_string()),
                portal_resume_id: Some("6960663240925956660".to_string()),
                user_id: Some("6960663240925956656".to_string()),
                talent_id: Some("7095600054216542508".to_string()),
                extra: Default::default(),
            }),
            status_msg: Some(" ".to_string()),
            extra_info: Some("  ".to_string()),
            extra: Default::default(),
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
                    id: Some("post_1".to_string()),
                    job_code: Some("007".to_string()),
                    job_expire_time: None,
                    job_active_status: Some(1),
                    job_process_type: Some(1),
                    job_recruitment_type: None,
                    job_department: None,
                    job_type: None,
                    min_job_level: None,
                    max_job_level: None,
                    address: None,
                    min_salary: None,
                    max_salary: None,
                    required_degree: None,
                    experience: None,
                    headcount: None,
                    high_light_list: None,
                    description: None,
                    requirement: None,
                    creator: None,
                    create_time: None,
                    modify_time: None,
                    job_function: None,
                    subject: None,
                    address_list: None,
                    target_major_list: None,
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
                    id: Some("post_1".to_string()),
                    job_code: Some("007".to_string()),
                    job_expire_time: None,
                    job_active_status: Some(1),
                    job_process_type: Some(1),
                    job_recruitment_type: None,
                    job_department: None,
                    job_type: None,
                    min_job_level: None,
                    max_job_level: None,
                    address: None,
                    min_salary: None,
                    max_salary: None,
                    required_degree: None,
                    experience: None,
                    headcount: None,
                    high_light_list: None,
                    description: None,
                    requirement: None,
                    creator: None,
                    create_time: None,
                    modify_time: None,
                    job_function: None,
                    subject: None,
                    address_list: None,
                    target_major_list: None,
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
                    id: Some("post_1".to_string()),
                    job_code: Some("007".to_string()),
                    job_expire_time: None,
                    job_active_status: Some(1),
                    job_process_type: Some(1),
                    job_recruitment_type: None,
                    job_department: None,
                    job_type: None,
                    min_job_level: None,
                    max_job_level: None,
                    address: None,
                    min_salary: None,
                    max_salary: None,
                    required_degree: None,
                    experience: None,
                    headcount: None,
                    high_light_list: None,
                    description: None,
                    requirement: None,
                    creator: None,
                    create_time: None,
                    modify_time: None,
                    job_function: None,
                    subject: None,
                    address_list: None,
                    target_major_list: None,
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
                    name: Some("张敏".to_string()),
                    external_id: Some("ext_1".to_string()),
                    mobile_country_code: Some("CN_1".to_string()),
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
            items: vec![agency::batch_query::AgencySupplier {
                id: Some("7398493486516799788".to_string()),
                name: Some("北极无敌猎头".to_string()),
                label_list: Some(vec![agency::batch_query::AgencySupplierLabel {
                    id: Some("6887469228283299336".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("东方树叶".to_string()),
                        en_us: Some("oriental Leaves".to_string()),
                        extra: Default::default(),
                    }),
                    extra: Default::default(),
                }]),
                admin_list: Some(vec![agency::batch_query::AgencySupplierAdmin {
                    user_id: Some("7398493486516799788".to_string()),
                    name: Some(openlark_hr::common::shared_models::I18nText {
                        zh_cn: Some("张三".to_string()),
                        en_us: Some("ZhangSan".to_string()),
                        extra: Default::default(),
                    }),
                    email: Some("283xxxx2171813@qq.com".to_string()),
                    extra: Default::default(),
                }]),
                agency_protect_time: Some(agency::batch_query::AgencySupplierProtectTime {
                    day: Some(180),
                    use_default: Some(true),
                    extra: Default::default(),
                }),
                cooperation_create_time: Some("1639992265035".to_string()),
                cooperation_start_time: Some("1639992265035".to_string()),
                cooperation_end_time: Some("1639992265035".to_string()),
                cooperation_status: Some(1),
                invite_email: Some("28933718393.qq.com".to_string()),
                supplier_area: Some(1),
                talent_protect_time: Some(agency::batch_query::AgencySupplierTalentProtectTime {
                    day: Some(180),
                    use_default: Some(true),
                    forever: Some(true),
                    extra: Default::default(),
                }),
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
            is_onboarded: Some(false),
            onboarded_in_protection: Some(false),
            onboarded_protection: Some(agency::protect_search::AgencyProtection {
                protection_type: Some(1),
                application_id: Some("app_1".to_string()),
                start_time: Some("1700023694629".to_string()),
                expire_time: Some("1700023694630".to_string()),
                agency_supplier_id: Some("supplier_1".to_string()),
                agency_supplier_name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("超越猎头公司".to_string()),
                    en_us: Some("chaoyue head-hunting company".to_string()),
                    extra: Default::default(),
                }),
                agency_supplier_user_id: Some("supplier_user_1".to_string()),
                agency_supplier_user_name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("张三".to_string()),
                    en_us: Some("zhangsan".to_string()),
                    extra: Default::default(),
                }),
                extra: Default::default(),
            }),
            protection_list: vec![agency::protect_search::AgencyProtection {
                protection_type: Some(2),
                application_id: Some("app_1".to_string()),
                start_time: Some("1700023694629".to_string()),
                expire_time: Some("1700023694630".to_string()),
                agency_supplier_id: Some("supplier_1".to_string()),
                agency_supplier_name: None,
                agency_supplier_user_id: None,
                agency_supplier_user_name: None,
                extra: Default::default(),
            }],
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
                ..Default::default()
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
                ..Default::default()
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
                ..Default::default()
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
            account: Some(referral_account::create::ReferralAccount {
                account_id: Some("acc_1".to_string()),
                assets: Some(referral_account::create::ReferralAccountAssets {
                    confirmed_bonus: Some(openlark_hr::hire::hire::common_models::BonusAmount {
                        point_bonus: Some(100),
                        cash_bonus: Some(vec![
                            openlark_hr::hire::hire::common_models::CashAmount {
                                currency_type: Some("CNY".to_string()),
                                amount: Some(100.0),
                                extra: Default::default(),
                            }
                        ]),
                        extra: Default::default(),
                    }),
                    extra: Default::default(),
                }),
                status: Some(2),
                extra: Default::default(),
            }),
        }
    );
    roundtrip_eq!(
        test_referral_account_enable_response_serialization,
        referral_account::enable::EnableResponse,
        referral_account::enable::EnableResponse {
            account: Some(referral_account::create::ReferralAccount {
                account_id: Some("acc_1".to_string()),
                assets: Some(referral_account::create::ReferralAccountAssets {
                    confirmed_bonus: Some(openlark_hr::hire::hire::common_models::BonusAmount {
                        point_bonus: Some(100),
                        cash_bonus: Some(vec![
                            openlark_hr::hire::hire::common_models::CashAmount {
                                currency_type: Some("CNY".to_string()),
                                amount: Some(100.0),
                                extra: Default::default(),
                            }
                        ]),
                        extra: Default::default(),
                    }),
                    extra: Default::default(),
                }),
                status: Some(1),
                extra: Default::default(),
            }),
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
            external_order_id: Some("order_1".to_string()),
            trans_time: Some("1683634459543".to_string()),
            withdrawal_details: Some(openlark_hr::hire::hire::common_models::BonusAmount {
                point_bonus: Some(100),
                cash_bonus: Some(vec![openlark_hr::hire::hire::common_models::CashAmount {
                    currency_type: Some("CNY".to_string()),
                    amount: Some(100.0),
                    extra: Default::default(),
                }]),
                extra: Default::default(),
            }),
        }
    );
    roundtrip_eq!(
        test_external_interview_assessment_create_response_serialization,
        external_interview_assessment::create::CreateResponse,
        external_interview_assessment::create::CreateResponse {
            external_interview_assessment: Some(
                openlark_hr::hire::hire::common_models::ExternalInterviewAssessment {
                    id: Some("assess_1".to_string()),
                    username: Some("shaojiale".to_string()),
                    conclusion: Some(1),
                    assessment_dimension_list: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentDimension {
                            score: Some(99),
                            option: Some("opt".to_string()),
                            options: Some(vec!["opt".to_string()]),
                            content: Some("content".to_string()),
                            assessment_type: Some(1),
                            title: Some("title".to_string()),
                            description: Some("desc".to_string()),
                            extra: Default::default(),
                        }
                    ]),
                    content: Some("hello world".to_string()),
                    external_interview_id: Some("iv_1".to_string()),
                    extra: Default::default(),
                }
            ),
            extra: Default::default(),
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
            external_interview_assessment: Some(
                openlark_hr::hire::hire::common_models::ExternalInterviewAssessment {
                    id: Some("assess_1".to_string()),
                    username: Some("shaojiale".to_string()),
                    conclusion: Some(2),
                    assessment_dimension_list: Some(vec![
                        openlark_hr::hire::hire::common_models::ExternalInterviewAssessmentDimension {
                            score: Some(99),
                            option: Some("opt".to_string()),
                            options: Some(vec!["opt1".to_string()]),
                            content: Some("content".to_string()),
                            assessment_type: Some(1),
                            title: Some("title".to_string()),
                            description: Some("desc".to_string()),
                            extra: Default::default(),
                        }
                    ]),
                    content: Some("hello world".to_string()),
                    external_interview_id: Some("iv_1".to_string()),
                    extra: Default::default(),
                }
            ),
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
                id: Some("iv_1".to_string()),
                begin_time: None,
                end_time: None,
                round: Some(0),
                interview_record_list: None,
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
            items: vec![interview::get_by_talent::TalentInterviewItem {
                application_id: Some("6949805467799537964".to_string()),
                interview_list: vec![interview::get_by_talent::InterviewExtendItem {
                    id: Some("6949805467799537964".to_string()),
                    begin_time: Some(1618899376474),
                    end_time: Some(1618999376474),
                    round: Some(0),
                    interview_type: Some(1),
                    contact_mobile: Some("13333333333".to_string()),
                    video_type: Some(1),
                    arrangement_status: Some(1),
                    arrangement_type: Some(1),
                    arrangement_appointment_kind: Some(1),
                    ..Default::default()
                }],
                extra: Default::default(),
            }],
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
            id: Some("reward_1".to_string()),
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
            job_requirement: Some(openlark_hr::hire::hire::common_models::JobRequirementDto {
                id: Some("jr_1".to_string()),
                short_code: Some("R01905".to_string()),
                name: Some("HR部门春季招聘需求".to_string()),
                display_progress: Some(1),
                head_count: Some(10),
                recruitment_type: None,
                employee_type: None,
                max_level: None,
                min_level: None,
                sequence: None,
                category: Some(1),
                department: None,
                recruiter_list: None,
                jr_hiring_managers: None,
                direct_leader_list: None,
                start_time: None,
                deadline: None,
                priority: Some(1),
                required_degree: None,
                max_salary: None,
                min_salary: None,
                address: None,
                description: None,
                customized_data_list: None,
                process_type: Some(1),
                job_type: None,
                create_time: None,
                creator_id: None,
                update_time: None,
                employment_job_id: None,
                position_id: None,
                completion_time: None,
                approval_status: Some(3),
                job_id_list: None,
                count_data: None,
                extra: Default::default(),
            }),
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
            items: vec![openlark_hr::hire::hire::common_models::JobRequirementDto {
                id: Some("jr_1".to_string()),
                short_code: Some("xx1".to_string()),
                name: Some("需求 A".to_string()),
                display_progress: Some(1),
                head_count: Some(11),
                job_id_list: Some(vec!["job_1".to_string()]),
                count_data: Some(
                    openlark_hr::hire::hire::common_models::JobRequirementCountData {
                        offer_count: Some(1),
                        pre_hire_count: Some(2),
                        complete_count: Some(3),
                        extra: Default::default(),
                    }
                ),
                ..Default::default()
            }],
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
            items: vec![interview_feedback_form::list::InterviewFeedbackForm {
                id: Some("form_1".to_string()),
                version: Some(1),
                name: Some(openlark_hr::common::shared_models::I18nText {
                    zh_cn: Some("通用评价表".to_string()),
                    en_us: Some("Default Form".to_string()),
                    extra: Default::default(),
                }),
                form_type: Some(2),
                score_calculation_config: Some(
                    interview_feedback_form::list::ScoreCalculationConfig {
                        enabled: Some(true),
                        calculation_mode: Some(1),
                        extra: Default::default(),
                    },
                ),
                modules: Some(vec![
                    interview_feedback_form::list::InterviewFeedbackFormModule {
                        id: Some("module_1".to_string()),
                        name: Some(openlark_hr::common::shared_models::I18nText {
                            zh_cn: Some("模块一".to_string()),
                            en_us: Some("Module 1".to_string()),
                            extra: Default::default(),
                        }),
                        description: None,
                        module_type: Some(2),
                        sequence: Some(1),
                        weight: Some(1.0),
                        dimensions: Some(vec![
                            interview_feedback_form::list::InterviewFeedbackFormDimension {
                                id: Some("dim_1".to_string()),
                                name: Some(openlark_hr::common::shared_models::I18nText {
                                    zh_cn: Some("维度一".to_string()),
                                    en_us: Some("Dimension 1".to_string()),
                                    extra: Default::default(),
                                }),
                                description: None,
                                dimension_type: Some(1),
                                enabled: Some(true),
                                sequence: Some(1),
                                is_required: Some(true),
                                weight: Some(1.0),
                                extra: Default::default(),
                            }
                        ]),
                        extra: Default::default(),
                    }
                ]),
                extra: Default::default(),
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
            info: None,
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
            interviewer: Some(
                openlark_hr::hire::hire::common_models::InterviewerOperationResult {
                    interviewer_id: Some("interviewer_1".to_string()),
                    verify_status: Some(2),
                    ..Default::default()
                }
            )
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
            },
            job_manager: Some(job::manager::batch_update::JobManagerView {
                id: Some("job_1".to_string()),
                recruiter_id: Some("rec_1".to_string()),
                hiring_manager_id_list: Some(vec!["mgr_1".to_string()]),
                assistant_id_list: None,
                extra: Default::default(),
            }),
        }
    );
    roundtrip_eq!(
        test_job_type_list_response_serialization,
        job_type::list::ListResponse,
        job_type::list::ListResponse {
            items: vec![openlark_hr::common::shared_models::CatalogItem {
                id: Some("type_1".to_string()),
                name: Some(openlark_hr::common::shared_models::FlexibleText::Plain(
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
                    id: Some("6989202908470446380".to_string()),
                    job_id: Some("6897921873523247373".to_string()),
                    talent_id: Some("7046263097923537196".to_string()),
                    application_id: Some("7046315599510833452".to_string()),
                    activity_status: Some(1),
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
            exam_id: Some("7031822096580868396".to_string()),
            application_id: Some("7031822096580868396".to_string()),
            exam_resource_name: Some("测试名字".to_string()),
            score: Some(95.5),
            uuid: Some("6949805467799537964".to_string()),
            operator_id: Some("ou_xxx".to_string()),
            operate_time: Some("1637295516451".to_string()),
            extra: Default::default(),
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

/// #556 surface seam：共享原语只走 `common::shared_models`；hire 专属模型仍在
/// `hire::hire::common_models`。
#[cfg(test)]
mod common_models_surface_tests {
    use openlark_hr::common::shared_models::{
        CatalogItem, CodeNameObject, FlexibleText, I18nText, IdNameObject, LocalizedLabel,
        PaginatedResponse,
    };
    use openlark_hr::hire::hire::common_models::{ApplicationJobInfo, AttachmentMeta, NoteRecord};

    #[test]
    fn shared_primitives_live_at_shared_models() {
        let i18n = I18nText {
            zh_cn: Some("中文".to_string()),
            en_us: Some("en".to_string()),
            extra: Default::default(),
        };
        assert_eq!(i18n.zh_cn.as_deref(), Some("中文"));

        let plain = FlexibleText::Plain("plain".to_string());
        assert_eq!(plain.zh_cn_or_plain(), Some("plain"));
        let i18n_flex = FlexibleText::I18n(I18nText {
            zh_cn: Some("标题".to_string()),
            en_us: None,
            extra: Default::default(),
        });
        assert_eq!(i18n_flex.zh_cn_or_plain(), Some("标题"));

        let _id = IdNameObject {
            id: Some("ou_1".to_string()),
            name: Some(i18n),
            extra: Default::default(),
        };
        let _code = CodeNameObject {
            code: Some("CN".to_string()),
            ..Default::default()
        };
        let page: PaginatedResponse<String> = PaginatedResponse {
            items: vec!["a".to_string()],
            page_token: Some("t".to_string()),
            has_more: Some(true),
            extra: Default::default(),
        };
        assert_eq!(page.items.len(), 1);
        let _cat = CatalogItem {
            id: Some("c1".to_string()),
            name: Some(FlexibleText::Plain("目录".to_string())),
            ..Default::default()
        };
        let _label = LocalizedLabel {
            zh_name: Some("标签".to_string()),
            en_name: Some("label".to_string()),
            extra: Default::default(),
        };
    }

    #[test]
    fn hire_specific_models_remain_on_common_models() {
        let job = ApplicationJobInfo {
            job_id: Some("job_1".to_string()),
            job_name: Some("后端".to_string()),
            ..Default::default()
        };
        assert_eq!(job.job_id.as_deref(), Some("job_1"));

        let attachment = AttachmentMeta {
            file_id: Some("att_1".to_string()),
            file_name: Some("cv.pdf".to_string()),
            ..Default::default()
        };
        assert_eq!(attachment.file_id.as_deref(), Some("att_1"));

        let note = NoteRecord {
            id: Some("note_1".to_string()),
            content: Some("备注".to_string()),
            ..Default::default()
        };
        assert_eq!(note.content.as_deref(), Some("备注"));
    }
}
