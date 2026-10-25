//! hire response contract tests 集成测试。
#![cfg(feature = "hire")]

use openlark_hr::hire::hire::{
    v1::{
        advertisement, agency, application, attachment, background_check_order,
        diversity_inclusion, eco_account_custom_field, eco_background_check,
        eco_background_check_custom_field, eco_background_check_package, eco_exam, eco_exam_paper,
        ehr_import_task, employee, evaluation, exam, external_application,
        external_background_check, external_interview, external_interview_assessment,
        external_offer, external_referral_reward, interview, interview_feedback_form,
        interview_record as interview_record_v1, interview_registration_schema,
        interview_round_type, interview_task, interviewer, job, job_function, job_process,
        job_publish_record, job_requirement, job_requirement_schema, job_schema, job_type,
        location, note, offer, offer_application_form, offer_approval_template, offer_custom_field,
        offer_schema, portal_apply_schema, questionnaire, referral, referral_account,
        referral_website, registration_schema, resume_source, role, subject, talent,
        talent_blocklist, talent_object, talent_operation_log, talent_pool, r#test as hire_test,
        todo, tripartite_agreement, user_role, website,
    },
    v2::interview_record as interview_record_v2,
};
use serde::de::DeserializeOwned;
use serde_json::json;

fn parse_contract<T: DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).expect("contract payload should deserialize")
}

#[test]
fn role_and_subject_list_contracts_are_typed() {
    let roles: role::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "101",
            "name": {"zh_cn": "测试", "en_us": "test"},
            "description": {"zh_cn": "描述", "en_us": "desc"},
            "role_status": 1,
            "role_type": 1
        }],
        "has_more": true,
        "page_token": "cursor_1"
    }));
    assert_eq!(roles.items[0].id.as_deref(), Some("101"));
    assert_eq!(
        roles.items[0]
            .name
            .as_ref()
            .and_then(|v| v.zh_cn.as_deref()),
        Some("测试")
    );
    assert_eq!(roles.page_token.as_deref(), Some("cursor_1"));

    let subjects: subject::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "sub_1",
            "name": {"zh_cn": "秋招", "en_us": "Campus"},
            "active_status": 1,
            "creator": {"id": "ou_creator", "name": {"zh_cn": "张三", "en_us": "Zhang"}}
        }],
        "has_more": false
    }));
    assert_eq!(
        subjects.items[0]
            .creator
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("ou_creator")
    );
}

#[test]
fn website_location_and_interviewer_contracts_are_typed() {
    let websites: website::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "site_1",
            "name": {"zh_cn": "校招官网", "en_us": "Campus Site"},
            "process_type_list": [2],
            "job_channel_id": "channel_1"
        }]
    }));
    assert_eq!(
        websites.items[0].job_channel_id.as_deref(),
        Some("channel_1")
    );

    let locations: location::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "loc_1",
            "name": {"zh_cn": "成都高新区", "en_us": "Gaoxin, Chengdu"},
            "city": {"code": "CT_22", "name": {"zh_cn": "成都", "en_us": "Chengdu"}},
            "active_status": 1
        }],
        "has_more": true,
        "page_token": "cursor_2"
    }));
    assert_eq!(
        locations.items[0]
            .city
            .as_ref()
            .and_then(|v| v.code.as_deref()),
        Some("CT_22")
    );

    let interviewers: interviewer::list::ListResponse = parse_contract(json!({
        "items": [{"user_id": "ou_1", "verify_status": 2}],
        "has_more": false
    }));
    assert_eq!(interviewers.items[0].verify_status, Some(2));
}

#[test]
fn evaluation_and_todo_contracts_are_typed() {
    let evaluations: evaluation::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "eval_1",
            "application_id": "app_1",
            "commit_status": 1,
            "conclusion": 1,
            "content": "优秀"
        }],
        "has_more": false
    }));
    assert_eq!(
        evaluations.items[0].application_id.as_deref(),
        Some("app_1")
    );

    let todos: todo::list::ListResponse = parse_contract(json!({
        "items": [{
            "evaluation": {"id": "eval_task", "talent_id": "talent_1"},
            "offer": {"id": "offer_task", "application_id": "app_1"}
        }],
        "page_token": "cursor_3"
    }));
    assert_eq!(
        todos.items[0]
            .evaluation
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("eval_task")
    );
    assert_eq!(todos.page_token.as_deref(), Some("cursor_3"));
}

#[test]
fn referral_account_and_search_contracts_are_typed() {
    let created: referral_account::create::CreateResponse = parse_contract(json!({
        "account": {
            "account_id": "acc_1",
            "assets": {"confirmed_bonus": {"point_bonus": 100}},
            "status": 1
        }
    }));
    assert_eq!(
        created
            .account
            .as_ref()
            .and_then(|v| v.account_id.as_deref()),
        Some("acc_1")
    );
    assert_eq!(
        created
            .account
            .as_ref()
            .and_then(|v| v.assets.as_ref())
            .and_then(|v| v.confirmed_bonus.as_ref())
            .and_then(|v| v.point_bonus),
        Some(100)
    );

    let reconciled: referral_account::reconciliation::ReconciliationResponse =
        parse_contract(json!({
            "check_failed_list": [{
                "account_id": "acc_1",
                "total_withdraw_reward_info": {"point_bonus": 50},
                "total_recharge_reward_info": {"point_bonus": 100}
            }]
        }));
    assert_eq!(
        reconciled.check_failed_list[0].account_id.as_deref(),
        Some("acc_1")
    );

    let referrals: referral::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "ref_1",
            "application_ids": ["app_1"],
            "referral_user": {"id": "ou_ref", "name": {"zh_cn": "张三", "en_us": "Zhang"}}
        }]
    }));
    assert_eq!(
        referrals.items[0].application_ids.as_ref().map(|v| v.len()),
        Some(1)
    );

    // 官方形状：get_by_application 返回 referral 容器，含 referral_user_id 叶子
    let referral_by_application: referral::get_by_application::GetByApplicationResponse =
        parse_contract(json!({
            "referral": {
                "id": "7392247983414413606",
                "application_id": "643452344576878",
                "referral_user_id": "ou_669760b021dbd9edbf2f3dc11b3a25ed",
                "referral_user": {
                    "id": "ou_669760b021dbd9edbf2f3dc11b3a25ed",
                    "name": {"zh_cn": "张三", "en_us": "zhangsan"}
                }
            }
        }));
    assert_eq!(
        referral_by_application
            .referral
            .as_ref()
            .and_then(|v| v.referral_user_id.as_deref()),
        Some("ou_669760b021dbd9edbf2f3dc11b3a25ed")
    );
}

#[test]
fn interview_record_v2_contract_is_typed() {
    let records: interview_record_v2::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "record_1",
            "feedback_form_id": "form_1",
            "commit_status": 1,
            "record_score": {"score": 95.0, "total_score": 100.0},
            "interviewer": {"id": "ou_interviewer", "name": {"zh_cn": "面试官", "en_us": "Interviewer"}},
            "attachments": [{"file_id": "file_1", "file_name": "report.pdf", "content_type": "application/pdf"}],
            "module_assessments": [{
                "interview_feedback_form_module_id": "module_1",
                "module_name": {"zh_cn": "结论", "en_us": "Conclusion"},
                "dimension_assessments": [{
                    "interview_feedback_form_dimension_id": "dim_1",
                    "dimension_name": {"zh_cn": "知识", "en_us": "Knowledge"},
                    "dimension_score": 10.0
                }]
            }]
        }],
        "has_more": true,
        "page_token": "cursor_4"
    }));
    assert_eq!(
        records.items[0].attachments.as_ref().map(|v| v.len()),
        Some(1)
    );
    assert_eq!(
        records.items[0].module_assessments.as_ref().unwrap()[0]
            .dimension_assessments
            .as_ref()
            .unwrap()[0]
            .dimension_score,
        Some(10.0)
    );
}

#[test]
fn user_role_and_talent_pool_contracts_are_typed() {
    let roles: user_role::list::ListResponse = parse_contract(json!({
        "items": [{
            "user_id": "ou_user",
            "role_id": "101",
            "role_name": {"zh_cn": "招聘 HRBP", "en_us": "Recruitment HRBP"},
            "business_management_scopes": [{
                "entity": {"code": "application", "name": {"zh_cn": "测试", "en_us": "test"}},
                "scope_rule": {"rule_type": 1}
            }]
        }],
        "has_more": true
    }));
    assert_eq!(roles.items[0].role_id.as_deref(), Some("101"));
    assert_eq!(
        roles.items[0].business_management_scopes.as_ref().unwrap()[0]
            .scope_rule
            .as_ref()
            .unwrap()
            .rule_type,
        Some(1)
    );

    let pools: talent_pool::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "pool_1",
            "i18n_name": {"zh_cn": "公共人才库", "en_us": "Common Talent Pool"},
            "is_private": 1
        }],
        "has_more": false
    }));
    assert_eq!(pools.items[0].id.as_deref(), Some("pool_1"));
}

#[test]
fn attachment_talent_object_and_talent_v2_contracts_are_typed() {
    let attachment: interview_record_v1::attachment::get::GetResponse = parse_contract(json!({
        "attachment": {
            "id": "att_1",
            "url": "https://hire.feishu.cn/blob/xx/",
            "name": "面试记录.pdf",
            "mime": "application/pdf"
        }
    }));
    assert_eq!(
        attachment.attachment.as_ref().and_then(|v| v.id.as_deref()),
        Some("att_1")
    );

    let talent_object_resp: talent_object::query::QueryResponse = parse_contract(json!({
        "items": [{
            "id": "obj_1",
            "name": {"zh_cn": "教育经历", "en_us": "Education"},
            "setting": {"object_type": 11, "config": {"options": [{"key": "1", "name": {"zh_cn": "选项1", "en_us": "Option1"}}]}},
            "children_list": [{
                "id": "child_1",
                "name": {"zh_cn": "学历", "en_us": "Degree"},
                "parent_id": "obj_1"
            }]
        }]
    }));
    assert_eq!(
        talent_object_resp.items[0].children_list.as_ref().unwrap()[0]
            .parent_id
            .as_deref(),
        Some("obj_1")
    );

    let talent_resp: talent::GetResponse = parse_contract(json!({
        "talent": {
            "talent_id": "talent_1",
            "name": "张三",
            "email": "test@example.com"
        },
        "education_list": [{
            "school": "四川大学",
            "major": "计算机"
        }]
    }));
    assert_eq!(talent_resp.talent.talent_id.as_str(), "talent_1");
    assert_eq!(
        talent_resp.education_list.as_ref().unwrap()[0]
            .school
            .as_deref(),
        Some("四川大学")
    );
}

#[test]
fn note_and_attachment_contracts_are_typed() {
    let created: note::create::CreateResponse = parse_contract(json!({
        "note": {
            "id": "note_1",
            "talent_id": "talent_1",
            "application_id": "app_1",
            "is_private": false,
            "content": "测试"
        }
    }));
    assert_eq!(
        created.note.as_ref().and_then(|v| v.id.as_deref()),
        Some("note_1")
    );

    let notes: note::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "note_1",
            "creator_id": "ou_creator",
            "content": "备注列表项"
        }],
        "has_more": true,
        "page_token": "cursor_note"
    }));
    assert_eq!(notes.items[0].creator_id.as_deref(), Some("ou_creator"));
    assert_eq!(notes.page_token.as_deref(), Some("cursor_note"));

    let attachment_created: attachment::create::CreateResponse = parse_contract(json!({
        "id": "att_1",
        "name": "test.rar",
        "url": "https://example.com/blob"
    }));
    assert_eq!(attachment_created.id.as_deref(), Some("att_1"));

    let attachment_get: attachment::get::GetResponse = parse_contract(json!({
        "attachment": {
            "id": "att_1",
            "name": "resume.pdf",
            "mime": "application/pdf"
        }
    }));
    assert_eq!(
        attachment_get
            .attachment
            .as_ref()
            .and_then(|v| v.name.as_deref()),
        Some("resume.pdf")
    );

    let attachment_preview: attachment::preview::PreviewResponse = parse_contract(json!({
        "url": "https://example.com/blob"
    }));
    assert_eq!(
        attachment_preview.url.as_deref(),
        Some("https://example.com/blob")
    );
}

#[test]
fn offer_contracts_are_typed() {
    let offers: offer::list::ListResponse = parse_contract(json!({
        "has_more": true,
        "page_token": "cursor_offer",
        "items": [{
            "id": "offer_1",
            "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
            "offer_status": 1,
            "employee_type": {"id": "1", "zh_name": "正式", "en_name": "Regular"}
        }]
    }));
    assert_eq!(
        offers.items[0]
            .job_info
            .as_ref()
            .and_then(|v| v.job_id.as_deref()),
        Some("job_1")
    );

    let created: offer::create::CreateResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "application_id": "app_1",
        "schema_id": "schema_1",
        "offer_type": 1,
        "basic_info": {"department_id": "od_1", "leader_user_id": "ou_leader"},
        "salary_info": {"currency": "CNY", "basic_salary": "1000000"},
        "customized_info_list": [{"id": "field_1", "value": "1"}]
    }));
    assert_eq!(created.offer_id.as_deref(), Some("offer_1"));
    assert_eq!(
        created
            .salary_info
            .as_ref()
            .and_then(|v| v.currency.as_deref()),
        Some("CNY")
    );

    let updated: offer::update::UpdateResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "schema_id": "schema_1",
        "offer_type": 1,
        "basic_info": {"department_id": "od_1"}
    }));
    assert_eq!(updated.schema_id.as_deref(), Some("schema_1"));

    let status: offer::offer_status::OfferStatusResponse = parse_contract(json!({}));
    let intern: offer::intern_offer_status::InternOfferStatusResponse = parse_contract(json!({
        "offer_id": "offer_1",
        "operation": "confirm_onboarding",
        "onboarding_info": {"actual_onboarding_date": "2022-01-01"}
    }));
    assert_eq!(
        intern
            .onboarding_info
            .as_ref()
            .and_then(|v| v.actual_onboarding_date.as_deref()),
        Some("2022-01-01")
    );
    let _ = status;

    let forms: offer_application_form::list::ListResponse = parse_contract(json!({
        "has_more": false,
        "items": [{
            "id": "form_1",
            "name": {"zh_cn": "校招 Offer 申请表", "en_us": "campus offer application form"},
            "create_time": "1628512038000"
        }]
    }));
    assert_eq!(forms.items[0].id.as_deref(), Some("form_1"));
}

#[test]
fn offer_detail_and_schema_contracts_are_typed() {
    let detail: offer::get::GetResponse = parse_contract(json!({
        "offer": {
            "id": "offer_1",
            "application_id": "app_1",
            "basic_info": {
                "offer_type": 1,
                "employee_type": {"id": "1", "zh_name": "正式", "en_name": "Regular"},
                "onboard_address": {
                    "id": "addr_1",
                    "zh_name": "名字",
                    "en_name": "name",
                    "city": {"zh_name": "中文", "en_name": "eng", "code": "400700", "location_type": 3}
                }
            },
            "salary_plan": {"currency": "CNY", "basic_salary": "1000000"}
        }
    }));
    assert_eq!(
        detail
            .offer
            .as_ref()
            .and_then(|v| v.salary_plan.as_ref())
            .and_then(|v| v.currency.as_deref()),
        Some("CNY")
    );

    let form: offer_application_form::get::GetResponse = parse_contract(json!({
        "offer_apply_form": {
            "id": "form_1",
            "name": {"zh_cn": "校招 Offer 申请表", "en_us": "campus offer application form"},
            "schema": {
                "id": "schema_1",
                "module_list": [{
                    "id": "module_1",
                    "name": {"zh_cn": "基础信息模块", "en_us": "basic info module"},
                    "object_list": [{
                        "id": "obj_1",
                        "name": {"zh_cn": "薪资字段", "en_us": "salary field"},
                        "config": {
                            "options": [{"id": "opt_1", "name": {"zh_cn": "全年薪资", "en_us": "annual salary"}}],
                            "object_display_config": {"display_condition": 1}
                        }
                    }]
                }]
            }
        }
    }));
    assert_eq!(
        form.offer_apply_form
            .as_ref()
            .and_then(|v| v.schema.as_ref())
            .and_then(|v| v.module_list.as_ref())
            .map(|v| v.len()),
        Some(1)
    );

    let schema: offer_schema::get::GetResponse = parse_contract(json!({
        "id": "schema_1",
        "scenario": 1,
        "version": 121,
        "object_list": [{
            "id": "obj_1",
            "name": {"zh_cn": "名字", "en_us": "name"},
            "type": "number",
            "is_customized": true,
            "option_list": [{
                "name": {"zh_cn": "名字", "en_us": "name"},
                "index": 121,
                "active_status": 1
            }]
        }]
    }));
    assert_eq!(schema.object_list[0].object_type.as_deref(), Some("number"));
}

#[test]
fn application_contracts_are_typed() {
    let created: application::create::CreateResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_id": "talent_1",
        "job_id": "job_1",
        "application_status": 1,
        "stage_id": "stage_resume",
        "stage_name": "简历筛选",
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"}
    }));
    assert_eq!(created.application_id.as_deref(), Some("app_1"));

    // 官方形状：create 响应 data 顶层仅含投递 ID
    let created_official: application::create::CreateResponse =
        parse_contract(json!({ "id": "6920175883425581324" }));
    assert_eq!(created_official.id.as_deref(), Some("6920175883425581324"));

    let get_resp: application::get::GetResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_id": "talent_1",
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
        "stage_name": "初筛"
    }));
    assert_eq!(
        get_resp
            .job_info
            .as_ref()
            .and_then(|v| v.job_name.as_deref()),
        Some("后端工程师")
    );

    // 官方形状：data 下唯一键为 application 容器（含嵌套 stage/resume_source_info 等）
    let get_official: application::get::GetResponse = parse_contract(json!({
        "application": {
            "id": "6949805467799537964",
            "job_id": "6843547872837273223",
            "talent_id": "6843547872837273223",
            "resume_source_id": "6583482347283472832",
            "stage": {
                "id": "614218419274131",
                "zh_name": "面试",
                "en_name": "Interview",
                "type": 1
            },
            "active_status": 1,
            "delivery_type": 1,
            "resume_source_info": {
                "id": "614218419274131",
                "name": {"zh_cn": "测试", "en_us": "test"},
                "resume_source_type": 10002
            },
            "stage_time_list": [
                {"stage_id": "614218419274131", "enter_time": "1632991542615"}
            ],
            "application_preferred_city_list": [
                {"code": "1111", "name": {"zh_cn": "测试", "en_us": "test"}}
            ]
        }
    }));
    let application_obj = get_official
        .application
        .as_ref()
        .expect("application 应存在");
    assert_eq!(application_obj.id.as_deref(), Some("6949805467799537964"));
    assert_eq!(application_obj.delivery_type, Some(1));
    assert_eq!(
        application_obj.stage.as_ref().and_then(|v| v.stage_type),
        Some(1)
    );
    assert_eq!(
        application_obj
            .resume_source_info
            .as_ref()
            .and_then(|v| v.resume_source_type),
        Some(10002)
    );
    assert_eq!(
        application_obj.stage_time_list.as_ref().map(|v| v.len()),
        Some(1)
    );

    let detail: application::get_detail::GetDetailResponse = parse_contract(json!({
        "application_detail": {
            "basic_info": {
                "id": "app_1",
                "talent_id": "talent_1",
                "stage": {"id": "stage_1", "zh_name": "面试", "en_name": "Interview", "type": 4},
                "delivery_type": 1,
                "resume_source_info": {
                    "id": "source_1",
                    "name": {"zh_cn": "内推", "en_us": "Referral"},
                    "resume_source_type": 10002
                }
            },
            "talent": {"id": "talent_1", "name": "张三", "email": "zhangsan@example.com"},
            "job": {"id": "job_1", "name": "后端工程师"},
            "offer": {"offer_basic": {"id": "offer_1", "offer_status": 10}}
        }
    }));
    assert_eq!(
        detail
            .application_detail
            .as_ref()
            .and_then(|v| v.talent.as_ref())
            .and_then(|v| v.name.as_deref()),
        Some("张三")
    );
    assert_eq!(
        detail
            .application_detail
            .as_ref()
            .and_then(|v| v.basic_info.as_ref())
            .and_then(|v| v.delivery_type),
        Some(1)
    );
    assert_eq!(
        detail
            .application_detail
            .as_ref()
            .and_then(|v| v.offer.as_ref())
            .and_then(|v| v.offer_basic.as_ref())
            .and_then(|v| v.id.as_deref()),
        Some("offer_1")
    );

    let list: application::list::ListResponse = parse_contract(json!({
        "items": [{
            "application_id": "app_1",
            "talent_id": "talent_1",
            "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
            "application_status": 1
        }],
        "has_more": true,
        "page_token": "cursor_app"
    }));
    assert_eq!(list.items[0].application_id.as_deref(), Some("app_1"));
    assert_eq!(list.page_token.as_deref(), Some("cursor_app"));

    let interviews: application::interview::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "6949805467799537964",
            "begin_time": 1618899376474i64,
            "end_time": 1618999376474i64,
            "round": 0,
            "stage_id": "634324253532232",
            "interview_record_list": [{
                "id": "6949805467799537964",
                "user_id": "1618899376474",
                "content": "代码还行，只是不够聪明",
                "min_job_level_id": "6435238827342432",
                "max_job_level_id": "643523885843573",
                "commit_status": 1,
                "conclusion": 1,
                "interview_score": {
                    "id": "6949805467799537964",
                    "level": 3,
                    "zh_name": "3",
                    "zh_description": "通过, 能力达到要求, 建议录用",
                    "en_name": "three",
                    "en_description": "Pass, ability to meet the requirements, suggest to hire"
                }
            }]
        }],
        "has_more": false
    }));
    assert_eq!(
        interviews.records[0].id.as_deref(),
        Some("6949805467799537964")
    );
    assert_eq!(interviews.records[0].begin_time, Some(1618899376474));
    assert_eq!(interviews.records[0].round, Some(0));
    assert_eq!(
        interviews.records[0].stage_id.as_deref(),
        Some("634324253532232")
    );
    let record = interviews.records[0]
        .interview_record_list
        .as_ref()
        .and_then(|v| v.first())
        .expect("interview_record_list 应存在");
    assert_eq!(record.min_job_level_id.as_deref(), Some("6435238827342432"));
    assert_eq!(record.commit_status, Some(1));
    assert_eq!(
        record.interview_score.as_ref().and_then(|v| v.level),
        Some(3)
    );
}

#[test]
fn job_contracts_are_typed() {
    let created: job::combined_create::CombinedCreateResponse = parse_contract(json!({
        "default_job_post": { "id": "post_1" },
        "job": {
            "id": "job_1",
            "title": "后端工程师",
            "recruitment_type": {
                "id": "6001",
                "zh_name": "全职",
                "en_name": "FullTime",
                "active_status": 1
            }
        }
    }));
    assert_eq!(
        created.job.as_ref().and_then(|j| j.id.as_deref()),
        Some("job_1")
    );

    let get_resp: job::get::GetResponse = parse_contract(json!({
        "job_id": "job_1",
        "job_name": "后端工程师",
        "active_status": 1,
        "process_id": "process_1",
        "process_name": "标准流程",
        "department_id": "dept_1",
        "recruiters": [{
            "recruiter_id": "rec_1",
            "user_id": "ou_1",
            "name": "招聘负责人",
            "role": "owner",
            "role_type": 1
        }]
    }));
    assert_eq!(get_resp.job_name.as_deref(), Some("后端工程师"));
    assert_eq!(get_resp.recruiters.as_ref().map(|v| v.len()), Some(1));

    // 官方形状：data 下唯一键为 job 容器（含 id/title/嵌套 recruitment_type/department 等）
    let get_official: job::get::GetResponse = parse_contract(json!({
        "job": {
            "id": "6001",
            "title": "测试职位",
            "description": "职位描述文本",
            "code": "R18",
            "requirement": "职位要求文本",
            "recruitment_type": {
                "id": "6001",
                "zh_name": "全职",
                "en_name": "FullTime",
                "active_status": 1
            },
            "department": {
                "id": "od-xxxx",
                "zh_name": "字节跳动",
                "en_name": "Bytedance"
            }
        }
    }));
    let job_obj = get_official.job.as_ref().expect("job 容器应存在");
    assert_eq!(job_obj.id.as_deref(), Some("6001"));
    assert_eq!(job_obj.title.as_deref(), Some("测试职位"));
    assert_eq!(job_obj.requirement.as_deref(), Some("职位要求文本"));
    assert_eq!(
        job_obj.recruitment_type.as_ref().and_then(|v| v.id.clone()),
        Some("6001".to_string())
    );
    assert_eq!(
        job_obj.department.as_ref().and_then(|v| v.en_name.clone()),
        Some("Bytedance".to_string())
    );

    let detail: job::get_detail::GetDetailResponse = parse_contract(json!({
        "job_detail": {
            "basic_info": {
                "id": "job_1",
                "title": "后端工程师",
                "description": "负责核心服务开发"
            },
            "recruiter": {
                "id": "ou_1",
                "name": {"zh_cn": "招聘负责人", "en_us": "Recruiter"}
            }
        }
    }));
    assert_eq!(
        detail
            .job_detail
            .as_ref()
            .and_then(|v| v.basic_info.as_ref())
            .and_then(|v| v.description.as_deref()),
        Some("负责核心服务开发")
    );
    assert_eq!(
        detail
            .job_detail
            .as_ref()
            .and_then(|v| v.recruiter.as_ref())
            .and_then(|v| v.id.as_deref()),
        Some("ou_1")
    );

    let listed: job::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "job_1",
            "title": "后端工程师",
            "description": "这是一个测试职位",
            "code": "R18",
            "active_status": 1,
            "recruitment_type": {
                "id": "6001",
                "zh_name": "全职",
                "en_name": "FullTime",
                "active_status": 1
            },
            "department": {
                "id": "od-xxxx",
                "zh_name": "字节跳动",
                "en_name": "Bytedance"
            },
            "process_name": "标准流程"
        }],
        "has_more": true,
        "page_token": "cursor_job"
    }));
    assert_eq!(listed.items[0].id.as_deref(), Some("job_1"));
    assert_eq!(
        listed.items[0]
            .recruitment_type
            .as_ref()
            .and_then(|r| r.zh_name.as_deref()),
        Some("全职")
    );
    assert_eq!(listed.page_token.as_deref(), Some("cursor_job"));

    let config: job::config::ConfigResponse = parse_contract(json!({
        "job_config": {
            "offer_apply_schema": {
                "id": "form_offer",
                "name": { "zh_cn": "申请表1", "en_us": "Offer Apply 1" }
            },
            "recommended_evaluator_list": [
                { "id": "ou_1", "name": { "zh_cn": "张三", "en_us": "ZhangSan" } }
            ],
            "id": "job_1",
            "job_attribute": 1,
            "interview_registration": { "schema_id": "schema_interview", "name": "默认登记表" }
        }
    }));
    assert_eq!(
        config
            .job_config
            .as_ref()
            .and_then(|c| c.offer_apply_schema.as_ref())
            .and_then(|s| s.id.as_deref()),
        Some("form_offer")
    );
    assert_eq!(
        config
            .job_config
            .as_ref()
            .and_then(|c| c.interview_registration.as_ref())
            .and_then(|r| r.schema_id.as_deref()),
        Some("schema_interview")
    );

    let recruiters: job::recruiter::RecruiterResponse = parse_contract(json!({
        "info": {
            "id": "job_1",
            "recruiter_id": "rec_1",
            "hiring_manager_id_list": ["mgr_1"],
            "assistant_id_list": ["ast_1"]
        }
    }));
    let recruiter_info = recruiters.info.as_ref().expect("info 应被类型化解析");
    assert_eq!(recruiter_info.recruiter_id.as_deref(), Some("rec_1"));
    assert_eq!(
        recruiter_info.hiring_manager_id_list.as_deref(),
        Some(&["mgr_1".to_string()][..])
    );

    let opened: job::open::OpenResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(opened.result, Some(true));

    let closed: job::close::CloseResponse = parse_contract(json!({
        "job_id": "job_1",
        "success": true
    }));
    assert_eq!(closed.success, Some(true));

    let updated: job::combined_update::CombinedUpdateResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(updated.job_id.as_deref(), Some("job_1"));

    let update_config: job::update_config::UpdateConfigResponse = parse_contract(json!({
        "job_id": "job_1",
        "result": true
    }));
    assert_eq!(update_config.result, Some(true));
}

#[test]
fn external_application_and_offer_contracts_are_typed() {
    let ext_app_created: external_application::create::CreateResponse = parse_contract(json!({
        "external_application": {
            "id": "ext_app_1",
            "job_recruitment_type": 1,
            "job_title": "高级 Java",
            "resume_source": "内推",
            "stage": "简历初筛",
            "talent_id": "talent_1",
            "delivery_type": 1,
            "modify_time": 1618500278645i64,
            "create_time": 1618500278644i64
        }
    }));
    assert_eq!(
        ext_app_created
            .external_application
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("ext_app_1")
    );

    let ext_app_list: external_application::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "ext_app_1",
            "job_recruitment_type": 1,
            "job_title": "高级Java",
            "resume_source": "Boss",
            "stage": "简历初筛",
            "talent_id": "talent_1",
            "delivery_type": 2,
            "modify_time": 1618500278645i64,
            "create_time": 1618500278644i64
        }],
        "has_more": false
    }));
    assert_eq!(ext_app_list.items[0].id.as_deref(), Some("ext_app_1"));
    assert_eq!(ext_app_list.items[0].resume_source.as_deref(), Some("Boss"));

    let ext_app_updated: external_application::update::UpdateResponse = parse_contract(json!({
        "external_application": {
            "id": "ext_app_1",
            "job_recruitment_type": 1,
            "job_title": "高级Java",
            "resume_source": "内推",
            "stage": "简历初筛",
            "talent_id": "talent_1",
            "delivery_type": 1,
            "modify_time": 1618500278645i64,
            "create_time": 1618500278644i64,
            "termination_type": "HR 主动终止"
        }
    }));
    assert_eq!(
        ext_app_updated
            .external_application
            .as_ref()
            .and_then(|a| a.id.as_deref()),
        Some("ext_app_1")
    );

    let ext_offer_created: external_offer::create::CreateResponse = parse_contract(json!({
        "external_offer": {
            "id": "ext_offer_1",
            "external_application_id": "app_1",
            "biz_create_time": "1721899352428",
            "owner": "张三",
            "offer_status": "已发送",
            "attachment_list": [
                {"id": "attach_1", "name": "test_resume.pdf", "size": 126371}
            ]
        }
    }));
    assert_eq!(
        ext_offer_created
            .external_offer
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("ext_offer_1")
    );

    let ext_offer_list: external_offer::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "id": "6989202908470446380",
            "external_application_id": "7395015673275697419",
            "biz_create_time": "1721899352428",
            "owner": "张三",
            "offer_status": "已发送",
            "attachment_list": [
                { "id": "6987954043925432620", "name": "test_resume.pdf", "size": 126371 }
            ]
        }],
        "page_token": "cursor_ext_offer"
    }));
    assert_eq!(
        ext_offer_list.items[0].id.as_deref(),
        Some("6989202908470446380")
    );
    assert_eq!(
        ext_offer_list.items[0].offer_status.as_deref(),
        Some("已发送")
    );
    assert_eq!(
        ext_offer_list.items[0]
            .attachment_list
            .as_ref()
            .and_then(|l| l.first())
            .and_then(|a| a.size),
        Some(126371)
    );
    assert_eq!(
        ext_offer_list.page_token.as_deref(),
        Some("cursor_ext_offer")
    );

    let ext_offer_updated: external_offer::update::UpdateResponse = parse_contract(json!({
        "external_offer": {
            "id": "6989202908470446380",
            "external_application_id": "7395015673275697419",
            "biz_create_time": "1721899352428",
            "owner": "张三",
            "offer_status": "已发送",
            "attachment_list": [
                { "id": "6987954043925432620", "name": "test_resume.pdf", "size": 126371 }
            ]
        }
    }));
    assert_eq!(
        ext_offer_updated
            .external_offer
            .as_ref()
            .and_then(|o| o.id.as_deref()),
        Some("6989202908470446380")
    );
}

#[test]
fn website_channel_delivery_and_job_post_contracts_are_typed() {
    let channel_created: website::channel::create::CreateResponse = parse_contract(json!({
        "id": "7085989097067563300",
        "name": "官网微信渠道",
        "link": "http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5",
        "code": "A1KM6A5"
    }));
    assert_eq!(channel_created.id.as_deref(), Some("7085989097067563300"));
    assert_eq!(
        channel_created.link.as_deref(),
        Some("http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5")
    );

    let channels: website::channel::list::ListResponse = parse_contract(json!({
        "website_channel_list": [{
            "id": "7085989097067563300",
            "name": "微信推广渠道",
            "link": "http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5",
            "code": "A1KM6A5"
        }],
        "has_more": false
    }));
    assert_eq!(channels.items.len(), 1);
    assert_eq!(channels.items[0].code.as_deref(), Some("A1KM6A5"));
    assert_eq!(channels.items[0].name.as_deref(), Some("微信推广渠道"));

    let channel_updated: website::channel::update::UpdateResponse = parse_contract(json!({
        "id": "7085989097067563300",
        "name": "微信推广渠道",
        "link": "http://recrui-demo.jobs.xxx.cn/485083/?spread=A1KM6A5",
        "code": "A1KM6A5"
    }));
    assert_eq!(channel_updated.name.as_deref(), Some("微信推广渠道"));

    let delivery: website::delivery_task::get::GetResponse = parse_contract(json!({
        "status": 2,
        "delivery": {
            "application_id": "6960663240925956657",
            "id": "6960663240925956655",
            "job_id": "6960663240925956659",
            "job_post_id": "6960663240925956658",
            "portal_resume_id": "6960663240925956660",
            "user_id": "6960663240925956656",
            "talent_id": "7095600054216542508"
        },
        "status_msg": " ",
        "extra_info": "  "
    }));
    assert_eq!(delivery.status, Some(2));
    assert_eq!(
        delivery
            .delivery
            .as_ref()
            .and_then(|v| v.portal_resume_id.as_deref()),
        Some("6960663240925956660")
    );
    assert_eq!(delivery.status_msg.as_deref(), Some(" "));

    let post: website::job_post::get::GetResponse = parse_contract(json!({
        "job_post": {
            "job_post_id": "post_1",
            "website_id": "site_1",
            "job_id": "job_1",
            "title": "后端工程师",
            "active_status": 1,
            "job_channel_id": "channel_1"
        }
    }));
    assert_eq!(
        post.job_post
            .as_ref()
            .and_then(|v| v.job_channel_id.as_deref()),
        Some("channel_1")
    );

    let post_list: website::job_post::list::ListResponse = parse_contract(json!({
        "items": [{
            "job_post_id": "post_1",
            "website_id": "site_1",
            "job_id": "job_1",
            "title": "后端工程师"
        }],
        "page_token": "cursor_post"
    }));
    assert_eq!(post_list.page_token.as_deref(), Some("cursor_post"));

    let site_user: website::site_user::create::CreateResponse = parse_contract(json!({
        "site_user": {
            "site_user_id": "site_user_1",
            "website_id": "site_1",
            "user_id": "ou_1",
            "email": "test@example.com"
        }
    }));
    assert_eq!(
        site_user
            .site_user
            .as_ref()
            .and_then(|v| v.user_id.as_deref()),
        Some("ou_1")
    );
}

#[test]
fn agency_contracts_are_typed() {
    let batch: agency::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "id": "7398493486516799788",
            "name": "北极无敌猎头",
            "admin_list": [{
                "user_id": "7398493486516799788",
                "name": {"zh_cn": "张三", "en_us": "ZhangSan"},
                "email": "283xxxx2171813@qq.com"
            }],
            "agency_protect_time": { "day": 180, "use_default": true },
            "cooperation_status": 1,
            "supplier_area": 1,
            "talent_protect_time": { "day": 180, "use_default": true, "forever": true }
        }],
        "has_more": false
    }));
    assert_eq!(batch.items[0].id.as_deref(), Some("7398493486516799788"));
    assert_eq!(
        batch.items[0]
            .admin_list
            .as_ref()
            .and_then(|l| l.first())
            .and_then(|a| a.email.as_deref()),
        Some("283xxxx2171813@qq.com")
    );

    let get_resp: agency::get::GetResponse = parse_contract(json!({
        "agency": {
            "agency_id": "agency_1",
            "name": "猎企 A",
            "status": 1
        }
    }));
    assert_eq!(
        get_resp.agency.as_ref().and_then(|v| v.name.as_deref()),
        Some("猎企 A")
    );

    let accounts: agency::get_agency_account::GetAgencyAccountResponse = parse_contract(json!({
        "items": [{
            "agency_account_id": "acc_1",
            "agency_id": "agency_1",
            "user_id": "ou_1",
            "name": "猎头甲",
            "status": 1
        }]
    }));
    assert_eq!(
        accounts.items[0].agency_account_id.as_deref(),
        Some("acc_1")
    );

    let protect_search: agency::protect_search::ProtectSearchResponse = parse_contract(json!({
        "is_onboarded": true,
        "onboarded_in_protection": true,
        "onboarded_protection": {
            "protection_type": 1,
            "application_id": "app_1",
            "start_time": "1700023694629",
            "expire_time": "1700023694630",
            "agency_supplier_id": "supplier_1",
            "agency_supplier_name": {"zh_cn": "超越猎头公司", "en_us": "chaoyue head-hunting company"},
            "agency_supplier_user_id": "supplier_user_1",
            "agency_supplier_user_name": {"zh_cn": "张三", "en_us": "zhangsan"}
        },
        "protection_list": [
            {
                "protection_type": 2,
                "application_id": "app_1",
                "start_time": "1700023694629",
                "expire_time": "1700023694630",
                "agency_supplier_id": "supplier_1",
                "agency_supplier_name": {"zh_cn": "超越猎头公司", "en_us": "chaoyue head-hunting company"},
                "agency_supplier_user_id": "supplier_user_1",
                "agency_supplier_user_name": {"zh_cn": "张三", "en_us": "zhangsan"}
            }
        ]
    }));
    assert_eq!(protect_search.is_onboarded, Some(true));
    assert_eq!(protect_search.protection_list.len(), 1);
    assert_eq!(
        protect_search
            .onboarded_protection
            .as_ref()
            .and_then(|v| v.agency_supplier_id.as_deref()),
        Some("supplier_1")
    );

    let operate: agency::operate_agency_account::OperateAgencyAccountResponse =
        parse_contract(json!({
            "agency_account_id": "acc_1",
            "result": true
        }));
    assert_eq!(operate.result, Some(true));

    let protect: agency::protect::ProtectResponse = parse_contract(json!({
        "protection_id": "protect_1",
        "success": true
    }));
    assert_eq!(protect.success, Some(true));

    let query: agency::query::QueryResponse = parse_contract(json!({
        "items": [{
            "agency_id": "agency_1",
            "name": "猎企 A"
        }],
        "page_token": "cursor_agency"
    }));
    assert_eq!(query.page_token.as_deref(), Some("cursor_agency"));
}

#[test]
fn talent_and_job_requirement_contracts_are_typed() {
    let add_folder: talent::add_to_folder::AddToFolderResponse = parse_contract(json!({
        "talent_id": "talent_1",
        "folder_id": "folder_1",
        "result": true
    }));
    assert_eq!(add_folder.folder_id.as_deref(), Some("folder_1"));

    let onboard: talent::onboard_status::OnboardStatusResponse = parse_contract(json!({
        "talent_id": "talent_1",
        "onboard_status": 2,
        "success": true
    }));
    assert_eq!(onboard.onboard_status, Some(2));

    let tag: talent::tag::TagResponse = parse_contract(json!({
        "talent_id": "talent_1",
        "tag_ids": ["tag_1", "tag_2"],
        "result": true
    }));
    assert_eq!(tag.tag_ids.as_ref().map(|v| v.len()), Some(2));

    let ext_info: talent::external_info::create::CreateResponse = parse_contract(json!({
        "external_info": {
            "external_info_id": "ext_1",
            "talent_id": "talent_1",
            "source_name": "Boss",
            "external_id": "boss_1",
            "status": 1
        }
    }));
    assert_eq!(
        ext_info
            .external_info
            .as_ref()
            .and_then(|v| v.external_id.as_deref()),
        Some("boss_1")
    );

    let requirements: job_requirement::list::ListResponse = parse_contract(json!({
        "items": [{
            "job_requirement_id": "jr_1",
            "job_id": "job_1",
            "title": "招聘需求 A",
            "status": 1
        }],
        "page_token": "cursor_jr"
    }));
    assert_eq!(requirements.page_token.as_deref(), Some("cursor_jr"));

    let requirement_by_id: job_requirement::list_by_id::ListByIdResponse = parse_contract(json!({
        "items": [{
            "id": "jr_1",
            "short_code": "xx1",
            "name": "需求 A",
            "display_progress": 1,
            "head_count": 11,
            "job_id_list": ["job_1"],
            "count_data": {
                "offer_count": 1,
                "pre_hire_count": 2,
                "complete_count": 3
            }
        }]
    }));
    assert_eq!(requirement_by_id.items[0].id.as_deref(), Some("jr_1"));
    assert_eq!(
        requirement_by_id.items[0].short_code.as_deref(),
        Some("xx1")
    );
    assert_eq!(requirement_by_id.items[0].head_count, Some(11));
    assert_eq!(
        requirement_by_id.items[0]
            .count_data
            .as_ref()
            .and_then(|c| c.offer_count),
        Some(1)
    );

    let requirement_create: job_requirement::create::CreateResponse = parse_contract(json!({
        "job_requirement": {
            "id": "jr_1",
            "short_code": "R01905",
            "name": "招聘需求 A",
            "display_progress": 1,
            "head_count": 10,
            "recruitment_type": {
                "id": "101",
                "name": { "zh_cn": "社招-全职", "en_us": "Social recruitment - Full time" }
            }
        }
    }));
    assert_eq!(
        requirement_create
            .job_requirement
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("jr_1")
    );

    let requirement_update: job_requirement::update::UpdateResponse = parse_contract(json!({
        "job_requirement_id": "jr_1",
        "result": true
    }));
    assert_eq!(requirement_update.result, Some(true));
}

#[test]
fn application_mutation_contracts_are_typed() {
    let cancel: application::cancel_onboard::CancelOnboardResponse = parse_contract(json!({
        "application_id": "app_1",
        "result": true,
        "success": true
    }));
    assert_eq!(cancel.operation.application_id.as_deref(), Some("app_1"));

    let offer: application::offer::OfferResponse = parse_contract(json!({
        "offer": {
            "id": "offer_1",
            "application_id": "app_1",
            "basic_info": {
                "department_id": "dept_1"
            }
        }
    }));
    assert_eq!(
        offer.offer.as_ref().and_then(|v| v.id.as_deref()),
        Some("offer_1")
    );

    let recover: application::recover::RecoverResponse = parse_contract(json!({
        "application_id": "app_1",
        "stage_id": "stage_resume",
        "stage_name": "简历筛选",
        "result": true
    }));
    assert_eq!(recover.operation.stage_name.as_deref(), Some("简历筛选"));

    let terminate: application::terminate::TerminateResponse = parse_contract(json!({
        "application_id": "app_1",
        "result": true
    }));
    assert_eq!(terminate.operation.result, Some(true));

    // 官方形状：data 下唯一键为 employee 容器（16 个叶子字段）
    let transfer_onboard: application::transfer_onboard::TransferOnboardResponse =
        parse_contract(json!({
            "employee": {
                "id": "7095600054216542508",
                "application_id": "7073372582620416300",
                "onboard_status": 1,
                "conversion_status": 1,
                "onboard_time": 1637596800000i64,
                "expected_conversion_time": 1637596800000i64,
                "actual_conversion_time": 1637596800000i64,
                "overboard_time": 1637596800000i64,
                "overboard_note": "职业发展考虑",
                "onboard_city_code": "CT_2",
                "department": "6966123381141866028",
                "leader": "ou-xxx",
                "sequence": "6937934036379650311",
                "level": "7006234385490345986",
                "employee_type": "1",
                "job_requirement_id": "123123123213"
            }
        }));
    let onboard_employee = transfer_onboard.employee.as_ref().expect("employee 应存在");
    assert_eq!(onboard_employee.id.as_deref(), Some("7095600054216542508"));
    assert_eq!(onboard_employee.conversion_status, Some(1));
    assert_eq!(onboard_employee.onboard_time, Some(1637596800000));
    assert_eq!(
        onboard_employee.overboard_note.as_deref(),
        Some("职业发展考虑")
    );
    assert_eq!(onboard_employee.leader.as_deref(), Some("ou-xxx"));

    // 兼容旧平铺载荷：operation flatten 兜底仍可反序列化
    let transfer_onboard_legacy: application::transfer_onboard::TransferOnboardResponse =
        parse_contract(json!({
            "application_id": "app_1",
            "employee_id": "emp_1",
            "onboard_status": 1,
            "success": true
        }));
    assert_eq!(
        transfer_onboard_legacy.operation.employee_id.as_deref(),
        Some("emp_1")
    );

    let transfer_stage: application::transfer_stage::TransferStageResponse =
        parse_contract(json!({
            "application_id": "app_1",
            "stage_id": "stage_interview",
            "stage_name": "面试",
            "success": true
        }));
    assert_eq!(
        transfer_stage.operation.stage_id.as_deref(),
        Some("stage_interview")
    );
}

#[test]
fn external_background_interview_and_agreement_contracts_are_typed() {
    let background_checks: external_background_check::batch_query::BatchQueryResponse =
        parse_contract(json!({
            "items": [{
                "external_background_check_id": "bg_1",
                "application_id": "app_1",
                "talent_id": "talent_1",
                "status": 1,
                "vendor_name": "iBackground"
            }],
            "page_token": "cursor_bg"
        }));
    assert_eq!(background_checks.page_token.as_deref(), Some("cursor_bg"));

    let background_created: external_background_check::create::CreateResponse =
        parse_contract(json!({
            "external_background_check": {
                "id": "bg_1",
                "external_application_id": "app_1",
                "date": 1626602069393i64,
                "name": "张三的背调",
                "result": "已通过",
                "attachment_list": [
                    { "id": "attach_1", "name": "test_resume.pdf", "size": 126371 }
                ]
            }
        }));
    assert_eq!(
        background_created
            .external_background_check
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("bg_1")
    );

    let interviews: external_interview::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "external_application_id": "app_1",
            "id": "iv_1",
            "participate_status": 2,
            "begin_time": 1618500278638i64,
            "end_time": 1618500278639i64,
            "interview_assessments": [{
                "id": "assess_1",
                "username": "张三",
                "conclusion": 2,
                "assessment_dimension_list": [{
                    "score": 99,
                    "assessment_type": 1,
                    "title": "心理测试"
                }],
                "content": "这是一个心理测试题目"
            }]
        }]
    }));
    assert_eq!(interviews.items[0].id.as_deref(), Some("iv_1"));
    assert_eq!(
        interviews.items[0]
            .interview_assessments
            .as_ref()
            .and_then(|v| v.first())
            .and_then(|v| v.username.as_deref()),
        Some("张三")
    );

    let interview_updated: external_interview::update::UpdateResponse = parse_contract(json!({
        "external_interview": {
            "external_application_id": "6960663240925956437",
            "id": "6960663240925956436",
            "participate_status": 1,
            "begin_time": 1618500278638i64,
            "end_time": 1618500278639i64,
            "interview_assessments": [{
                "id": "6989181065243969836",
                "username": "张三",
                "conclusion": 1,
                "assessment_dimension_list": [{
                    "score": 99,
                    "option": "选项1",
                    "options": ["选项1"],
                    "content": "面试内容结果补充",
                    "assessment_type": 1,
                    "title": "心理测试",
                    "description": "心理测试描述"
                }],
                "content": "面试内容结果补充"
            }]
        }
    }));
    assert_eq!(
        interview_updated
            .external_interview
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("6960663240925956436")
    );
    assert_eq!(
        interview_updated
            .external_interview
            .as_ref()
            .and_then(|v| v.interview_assessments.as_ref())
            .and_then(|l| l.first())
            .and_then(|a| a.username.as_deref()),
        Some("张三")
    );

    let agreements: tripartite_agreement::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "agree_1",
            "application_id": "app_1",
            "state": 1,
            "create_time": "1698292282660",
            "modify_time": "1698292282661"
        }],
        "has_more": false
    }));
    assert_eq!(agreements.items[0].id.as_deref(), Some("agree_1"));

    let agreement_create: tripartite_agreement::create::CreateResponse = parse_contract(json!({
        "agreement_id": "agree_1",
        "application_id": "app_1",
        "status": 1,
        "success": true
    }));
    assert_eq!(agreement_create.status, Some(1));

    let agreement_delete: tripartite_agreement::delete::DeleteResponse = parse_contract(json!({
        "agreement_id": "agree_1",
        "result": true
    }));
    assert_eq!(agreement_delete.result, Some(true));
}

#[test]
fn eco_operation_contracts_are_typed() {
    let account_field: eco_account_custom_field::create::CreateResponse = parse_contract(json!({
        "custom_field": {
            "custom_field_id": "field_1",
            "name": "账号字段",
            "code": "acct_code",
            "status": 1
        }
    }));
    assert_eq!(
        account_field
            .custom_field
            .as_ref()
            .and_then(|v| v.custom_field_id.as_deref()),
        Some("field_1")
    );

    let bg_field_update: eco_background_check_custom_field::batch_update::BatchUpdateResponse =
        parse_contract(json!({
            "custom_field_id": "field_2",
            "result": true
        }));
    assert_eq!(bg_field_update.operation.result, Some(true));

    let package_create: eco_background_check_package::create::CreateResponse =
        parse_contract(json!({
            "package": {
                "package_id": "pkg_1",
                "name": "标准背调套餐",
                "status": 1
            }
        }));
    assert_eq!(
        package_create
            .package
            .as_ref()
            .and_then(|v| v.package_id.as_deref()),
        Some("pkg_1")
    );

    let paper_create: eco_exam_paper::create::CreateResponse = parse_contract(json!({
        "paper": {
            "paper_id": "paper_1",
            "name": "算法笔试",
            "status": 1
        }
    }));
    assert_eq!(
        paper_create
            .paper
            .as_ref()
            .and_then(|v| v.paper_id.as_deref()),
        Some("paper_1")
    );

    let bg_progress: eco_background_check::update_progress::UpdateProgressResponse =
        parse_contract(json!({
            "background_check_id": "bg_order_1",
            "progress": 80,
            "status": 2,
            "success": true
        }));
    assert_eq!(bg_progress.operation.progress, Some(80));
}

#[test]
fn remaining_low_volume_contracts_are_typed() {
    let employee: employee::get::GetResponse = parse_contract(json!({
        "employee": {
            "employee_id": "emp_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 1
        }
    }));
    assert_eq!(
        employee
            .employee
            .as_ref()
            .and_then(|v| v.employee_id.as_deref()),
        Some("emp_1")
    );

    let referral_enable: referral_account::enable::EnableResponse = parse_contract(json!({
        "account": {
            "account_id": "6942778198054125570",
            "assets": {
                "confirmed_bonus": {
                    "point_bonus": 100,
                    "cash_bonus": [{ "currency_type": "CNY", "amount": 100 }]
                }
            },
            "status": 1
        }
    }));
    assert_eq!(
        referral_enable
            .account
            .as_ref()
            .and_then(|a| a.account_id.as_deref()),
        Some("6942778198054125570")
    );
    assert_eq!(
        referral_enable
            .account
            .as_ref()
            .and_then(|a| a.assets.as_ref())
            .and_then(|s| s.confirmed_bonus.as_ref())
            .and_then(|b| b.point_bonus),
        Some(100)
    );

    let assessment: external_interview_assessment::create::CreateResponse = parse_contract(json!({
        "external_interview_assessment": {
            "id": "assess_1",
            "username": "shaojiale",
            "conclusion": 1,
            "assessment_dimension_list": [
                {
                    "score": 99,
                    "option": "opt",
                    "options": ["opt"],
                    "content": "content",
                    "assessment_type": 1,
                    "title": "title",
                    "description": "desc"
                }
            ],
            "content": "hello world",
            "external_interview_id": "iv_1"
        }
    }));
    assert_eq!(
        assessment
            .external_interview_assessment
            .as_ref()
            .and_then(|v| v.id.as_deref()),
        Some("assess_1")
    );

    let interviews: interview::list::ListResponse = parse_contract(json!({
        "items": [{
            "interview_id": "iv_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 2,
            "interview_round_name": "一面"
        }]
    }));
    assert_eq!(
        interviews.items[0].interview_round_name.as_deref(),
        Some("一面")
    );

    let bg_orders: background_check_order::list::ListResponse = parse_contract(json!({
        "items": [{
            "order_id": "order_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 2,
            "vendor_name": "VendorA"
        }]
    }));
    assert_eq!(bg_orders.items[0].order_id.as_deref(), Some("order_1"));

    let reward: external_referral_reward::create::CreateResponse = parse_contract(json!({
        "external_referral_reward_id": "reward_1",
        "account_id": "acc_1",
        "amount": 88.5,
        "success": true
    }));
    assert_eq!(reward.reward.amount, Some(88.5));

    let talent_pool: talent_pool::move_talent::MoveTalentResponse = parse_contract(json!({
        "talent_pool_id": "pool_1",
        "talent_id": "talent_1",
        "success": true
    }));
    assert_eq!(
        talent_pool.operation.talent_pool_id.as_deref(),
        Some("pool_1")
    );

    let exam: eco_exam::login_info::LoginInfoResponse = parse_contract(json!({
        "exam_id": "exam_1",
        "status": 1,
        "success": true
    }));
    assert_eq!(exam.exam.exam_id.as_deref(), Some("exam_1"));
}

#[test]
fn tail_get_and_operation_contracts_are_typed() {
    let role_detail: role::get::GetResponse = parse_contract(json!({
        "role": {
            "id": "role_1",
            "name": {"zh_cn": "面试官", "en_us": "Interviewer"}
        }
    }));
    assert_eq!(
        role_detail
            .role
            .as_ref()
            .and_then(|value| value.id.as_deref()),
        Some("role_1")
    );

    let referral_detail: referral::get_by_application::GetByApplicationResponse =
        parse_contract(json!({
            "referral": {
                "id": "ref_1",
                "application_ids": ["app_1"]
            }
        }));
    assert_eq!(
        referral_detail
            .referral
            .as_ref()
            .and_then(|value| value.id.as_deref()),
        Some("ref_1")
    );

    let website_job_post: referral_website::job_post::get::GetResponse = parse_contract(json!({
        "job_post": {
            "id": "jp_1",
            "job_id": "job_1"
        }
    }));
    assert_eq!(
        website_job_post
            .job_post
            .as_ref()
            .and_then(|value| value.id.as_deref()),
        Some("jp_1")
    );

    let manager: job::manager::get::GetResponse = parse_contract(json!({
        "manager": {
            "manager_id": "mgr_1",
            "name": "张三"
        }
    }));
    assert_eq!(
        manager
            .manager
            .as_ref()
            .and_then(|value| value.manager_id.as_deref()),
        Some("mgr_1")
    );

    let custom_field: offer_custom_field::update::UpdateResponse = parse_contract(json!({
        "offer_custom_field_id": "field_1",
        "success": true
    }));
    assert_eq!(
        custom_field.operation.offer_custom_field_id.as_deref(),
        Some("field_1")
    );

    let interviewer_patch: interviewer::patch::PatchResponse = parse_contract(json!({
        "interviewer": {
            "interviewer_id": "int_1",
            "user_id": "ou_1",
            "verify_status": 2
        }
    }));
    assert_eq!(
        interviewer_patch
            .interviewer
            .as_ref()
            .and_then(|v| v.user_id.as_deref()),
        Some("ou_1")
    );

    let manager_update: job::manager::batch_update::BatchUpdateResponse = parse_contract(json!({
        "job_manager": {
            "id": "job_1",
            "recruiter_id": "rec_1",
            "hiring_manager_id_list": ["mgr_1"],
            "assistant_id_list": ["ast_1"]
        }
    }));
    let job_manager = manager_update
        .job_manager
        .as_ref()
        .expect("job_manager 应被类型化解析");
    assert_eq!(job_manager.id.as_deref(), Some("job_1"));
    assert_eq!(job_manager.recruiter_id.as_deref(), Some("rec_1"));
    assert_eq!(
        job_manager.hiring_manager_id_list.as_deref(),
        Some(&["mgr_1".to_string()][..])
    );

    let interview_record: interview_record_v1::get::GetResponse = parse_contract(json!({
        "id": "record_1",
        "conclusion": 1,
        "interviewer": {"id": "ou_interviewer"}
    }));
    assert_eq!(interview_record.id.as_deref(), Some("record_1"));

    let interview_record_v2_detail: interview_record_v2::get::GetResponse = parse_contract(json!({
        "id": "record_v2_1",
        "feedback_form_id": "form_1",
        "interviewer": {"id": "ou_interviewer"}
    }));
    assert_eq!(
        interview_record_v2_detail.feedback_form_id.as_deref(),
        Some("form_1")
    );

    let publish: advertisement::publish::PublishResponse = parse_contract(json!({
        "publish_id": "pub_1",
        "status": 1,
        "success": true
    }));
    assert_eq!(publish.operation.publish_id.as_deref(), Some("pub_1"));

    let exam_result: exam::create::CreateResponse = parse_contract(json!({
        "exam_id": "7031822096580868396",
        "application_id": "7031822096580868396",
        "exam_resource_name": "测试名字",
        "score": 95.5,
        "uuid": "6949805467799537964",
        "operator_id": "ou_xxx",
        "operate_time": "1637295516451"
    }));
    assert_eq!(exam_result.exam_id.as_deref(), Some("7031822096580868396"));
    assert_eq!(exam_result.exam_resource_name.as_deref(), Some("测试名字"));
    assert_eq!(exam_result.score, Some(95.5));
    assert_eq!(exam_result.uuid.as_deref(), Some("6949805467799537964"));

    let ehr_task: ehr_import_task::patch::PatchResponse = parse_contract(json!({
        "task_id": "task_1",
        "status": 2,
        "success": true
    }));
    assert_eq!(ehr_task.operation.task_id.as_deref(), Some("task_1"));

    let block_result: talent_blocklist::change_talent_block::ChangeTalentBlockResponse =
        parse_contract(json!({
            "talent_id": "talent_1",
            "status": 1,
            "success": true
        }));
    assert_eq!(
        block_result.operation.talent_id.as_deref(),
        Some("talent_1")
    );
}

#[test]
fn tail_catalog_and_search_contracts_are_typed() {
    let job_types: job_type::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "type_1",
            "name": {"zh_cn": "研发", "en_us": "Engineering"},
            "status": 1
        }]
    }));
    assert_eq!(job_types.items[0].id.as_deref(), Some("type_1"));
    assert_eq!(
        job_types.items[0]
            .name
            .as_ref()
            .and_then(|value| value.zh_cn_or_plain()),
        Some("研发")
    );

    let job_schemas: job_schema::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "6949805467799537964",
            "name": {"zh_cn": "社招模板", "en_us": "Social Job Template"},
            "scenario_type": 1,
            "object_list": [{
                "id": "6949805467799537964",
                "name": {"zh_cn": "基础信息", "en_us": "Basic info"},
                "description": {"zh_cn": "职位基础信息", "en_us": "Job basic info"},
                "setting": {
                    "object_type": 1,
                    "config": {
                        "options": [{
                            "key": "1",
                            "name": {"zh_cn": "选项A", "en_us": "Option A"},
                            "description": {"zh_cn": "选项A描述", "en_us": "Description of option A"},
                            "active_status": 1
                        }]
                    }
                },
                "is_customized": true,
                "is_required": false,
                "active_status": 1,
                "children_list": [{
                    "id": "6949805467799537964",
                    "name": {"zh_cn": "职位类型", "en_us": "Job type"},
                    "setting": {"object_type": 1, "config": {}},
                    "parent_id": "6949805467799537964",
                    "is_customized": true,
                    "is_required": false,
                    "active_status": 1
                }]
            }]
        }],
        "has_more": true,
        "page_token": "1234452132"
    }));
    assert_eq!(job_schemas.items[0].scenario_type, Some(1));
    assert_eq!(
        job_schemas.items[0]
            .name
            .as_ref()
            .and_then(|v| v.zh_cn.as_deref()),
        Some("社招模板")
    );
    let module = job_schemas.items[0]
        .object_list
        .as_ref()
        .and_then(|v| v.first())
        .expect("object_list 应存在");
    assert_eq!(module.setting.as_ref().and_then(|v| v.object_type), Some(1));
    assert_eq!(
        module
            .children_list
            .as_ref()
            .and_then(|v| v.first())
            .and_then(|c| c.parent_id.as_deref()),
        Some("6949805467799537964")
    );

    let interview_forms: interview_feedback_form::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "form_1",
            "name": {"zh_cn": "通用评价表", "en_us": "Default Form"},
            "type": 2,
            "score_calculation_config": {"enabled": true, "calculation_mode": 1},
            "modules": [{
                "id": "module_1",
                "name": {"zh_cn": "模块一", "en_us": "Module 1"},
                "type": 2,
                "sequence": 1,
                "weight": 1.0
            }]
        }],
        "has_more": false
    }));
    assert_eq!(
        interview_forms.items[0]
            .name
            .as_ref()
            .and_then(|value| value.zh_cn.as_deref()),
        Some("通用评价表")
    );
    assert_eq!(
        interview_forms.items[0]
            .score_calculation_config
            .as_ref()
            .and_then(|value| value.calculation_mode),
        Some(1)
    );

    let interview_schemas: interview_registration_schema::list::ListResponse = parse_contract(
        json!({
            "items": [{
                "id": "7044739584859326764",
                "name": "默认面试登记表",
                "is_used_as_interview": true,
                "object_list": [{
                    "id": "6949805467799537964",
                    "name": {"zh_cn": "登记表模块", "en_us": "interview module"},
                    "description": {"zh_cn": "用于面试登记的模块", "en_us": "module used for interview"},
                    "setting": {
                        "object_type": 11,
                        "config": {
                            "options": [{
                                "key": "test",
                                "name": {"zh_cn": "模块选项", "en_us": "module option"},
                                "description": {"zh_cn": "用于模块的选项", "en_us": "option for module"},
                                "active_status": 1
                            }]
                        }
                    },
                    "is_customized": true,
                    "is_required": false,
                    "is_visible": true,
                    "active_status": 1,
                    "children_list": [{
                        "id": "6949805467799537964",
                        "name": {"zh_cn": "登记表字段", "en_us": "interview registration field"},
                        "setting": { "object_type": 1, "config": { "options": [] } },
                        "parent_id": "6949805467799537964",
                        "is_customized": true,
                        "is_required": false,
                        "is_visible": true,
                        "active_status": 1
                    }]
                }]
            }],
            "has_more": true,
            "page_token": "cursor_5"
        }),
    );
    let schema_item = &interview_schemas.items[0];
    assert_eq!(schema_item.is_used_as_interview, Some(true));
    assert_eq!(schema_item.name.as_deref(), Some("默认面试登记表"));
    let schema_object = schema_item.object_list.as_ref().unwrap().first().unwrap();
    assert_eq!(
        schema_object.setting.as_ref().unwrap().object_type,
        Some(11)
    );
    assert_eq!(
        schema_object
            .children_list
            .as_ref()
            .unwrap()
            .first()
            .unwrap()
            .parent_id
            .as_deref(),
        Some("6949805467799537964")
    );
    assert_eq!(interview_schemas.page_token.as_deref(), Some("cursor_5"));

    let round_types: interview_round_type::list::ListResponse = parse_contract(json!({
        "active_status": 1,
        "items": [{
            "id": "7012129842917869868",
            "biz_id": "7012129842917869868",
            "name": {"zh_cn": "默认面试轮次类型", "en_us": "default interview round type"},
            "process_type": 1,
            "active_status": 1,
            "interview_assessment_template_info": {
                "id": "7012129842917869868",
                "biz_id": "7012129842917869868",
                "name": {"zh_cn": "默认面试评价表", "en_us": "default interview assessment template"}
            }
        }]
    }));
    assert_eq!(round_types.active_status, Some(1));
    let round_type_item = &round_types.items[0];
    assert_eq!(
        round_type_item.biz_id.as_deref(),
        Some("7012129842917869868")
    );
    assert_eq!(
        round_type_item
            .name
            .as_ref()
            .and_then(|value| value.zh_cn.as_deref()),
        Some("默认面试轮次类型")
    );
    assert_eq!(
        round_type_item
            .interview_assessment_template_info
            .as_ref()
            .and_then(|value| value.biz_id.as_deref()),
        Some("7012129842917869868")
    );

    let interview_tasks: interview_task::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "6989202908470446380",
            "job_id": "6897921873523247373",
            "talent_id": "7046263097923537196",
            "application_id": "7046315599510833452",
            "activity_status": 1
        }],
        "has_more": true
    }));
    let task_item = &interview_tasks.items[0];
    assert_eq!(task_item.id.as_deref(), Some("6989202908470446380"));
    assert_eq!(task_item.job_id.as_deref(), Some("6897921873523247373"));
    assert_eq!(task_item.talent_id.as_deref(), Some("7046263097923537196"));
    assert_eq!(task_item.activity_status, Some(1));

    let registrations: registration_schema::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "reg_schema_1",
            "name": "默认登记表",
            "scenarios": [5, 6],
            "objects": [{
                "id": "obj_1",
                "name": {"zh_cn": "基本信息", "en_us": "Basic Info"},
                "is_required": true
            }]
        }]
    }));
    assert_eq!(registrations.items[0].id.as_deref(), Some("reg_schema_1"));
    assert_eq!(
        registrations.items[0].scenarios.as_deref(),
        Some(&[5, 6][..])
    );
    assert_eq!(
        registrations.items[0]
            .objects
            .as_ref()
            .and_then(|l| l.first())
            .and_then(|o| o.id.as_deref()),
        Some("obj_1")
    );

    let portal_schemas: portal_apply_schema::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "portal_schema_1",
            "name": "默认官网申请表",
            "scenarios": [14],
            "objects": [{
                "id": "obj_1",
                "name": {"zh_cn": "基本信息", "en_us": "Basic Information"},
                "setting": {
                    "object_type": 1,
                    "config": {
                        "options": [{
                            "key": "Option1",
                            "name": {"zh_cn": "选项一", "en_us": "Option1"},
                            "active_status": 1
                        }]
                    }
                },
                "is_customized": true,
                "is_required": false,
                "is_visible": true,
                "active_status": 1,
                "children_list": [{
                    "id": "field_1",
                    "name": {"zh_cn": "自定义字段", "en_us": "Custom Field"},
                    "parent_id": "obj_1",
                    "is_customized": true,
                    "is_required": false,
                    "is_visible": true,
                    "active_status": 1
                }]
            }]
        }]
    }));
    assert_eq!(
        portal_schemas.items[0].id.as_deref(),
        Some("portal_schema_1")
    );
    assert_eq!(
        portal_schemas.items[0].scenarios.as_deref(),
        Some(&[14][..])
    );
    assert_eq!(
        portal_schemas.items[0]
            .objects
            .as_ref()
            .and_then(|l| l.first())
            .and_then(|o| o.children_list.as_ref())
            .and_then(|l| l.first())
            .and_then(|c| c.parent_id.as_deref()),
        Some("obj_1")
    );

    let questionnaires: questionnaire::list::ListResponse = parse_contract(json!({
        "items": [{
            "questionnaire_id": "7037986982531778860",
            "application_id": "6985833807195212076",
            "interview_id": "7038435261598763308",
            "version": 59,
            "questions": [{
                "question_id": "6950930570311829804",
                "question_name": "你的总体评价如何？",
                "question_en_name": "How would you describe your overall experience?",
                "question_desc": "测试描述内容",
                "question_en_desc": "Test Text",
                "question_type": 4,
                "is_required": true,
                "select_option_result_list": [{
                    "option_id": "6950930570311993644",
                    "option_name": "选项一",
                    "option_en_name": "Option 1",
                    "option_desc": "测试选项描述",
                    "option_en_desc": "Test Option Desc",
                    "is_selected": true
                }],
                "five_start_scoring_result": {
                    "highest_score_desc": "非常好",
                    "highest_score_en_desc": "Very Good",
                    "lowest_score_desc": "非常差",
                    "lowest_score_en_desc": "Very Bad",
                    "score_result": 4
                },
                "description_result": "测试描述内容"
            }],
            "has_answers": true,
            "update_time": "1644412504000"
        }],
        "has_more": true
    }));
    let questionnaire_item = &questionnaires.items[0];
    assert_eq!(
        questionnaire_item.questionnaire_id.as_deref(),
        Some("7037986982531778860")
    );
    assert_eq!(questionnaire_item.version, Some(59));
    let question = questionnaire_item
        .questions
        .as_ref()
        .unwrap()
        .first()
        .unwrap();
    assert_eq!(question.question_id.as_deref(), Some("6950930570311829804"));
    assert_eq!(question.question_type, Some(4));
    assert_eq!(
        question
            .five_start_scoring_result
            .as_ref()
            .and_then(|value| value.score_result),
        Some(4.0)
    );

    let resume_sources: resume_source::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "10000",
            "zh_name": "内推",
            "en_name": "referral",
            "active_status": 1,
            "resume_source_type": "10001"
        }],
        "page_token": "1",
        "has_more": true
    }));
    assert_eq!(resume_sources.items[0].id.as_deref(), Some("10000"));
    assert_eq!(resume_sources.items[0].zh_name.as_deref(), Some("内推"));
    assert_eq!(
        resume_sources.items[0].resume_source_type.as_deref(),
        Some("10001")
    );

    let job_functions: job_function::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "func_1",
            "parent_id": "func_root",
            "name": {"zh_cn": "后端研发", "en_us": "Backend"}
        }]
    }));
    assert_eq!(
        job_functions.items[0].parent_id.as_deref(),
        Some("func_root")
    );

    let job_processes: job_process::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "7383889964737988883",
            "zh_name": "社招流程",
            "en_name": "Social Process",
            "type": 1,
            "stage_list": [{
                "id": "6793210103211510024",
                "zh_name": "面试阶段",
                "en_name": "Interivew",
                "type": 1
            }]
        }],
        "has_more": true,
        "page_token": "7342029494511126794"
    }));
    assert_eq!(
        job_processes.items[0].id.as_deref(),
        Some("7383889964737988883")
    );
    assert_eq!(job_processes.items[0].process_type, Some(1));
    let stage = job_processes.items[0]
        .stage_list
        .as_ref()
        .and_then(|v| v.first())
        .expect("stage_list 应存在");
    assert_eq!(stage.zh_name.as_deref(), Some("面试阶段"));
    assert_eq!(stage.stage_type, Some(1));

    let job_requirement_schemas: job_requirement_schema::list::ListResponse = parse_contract(
        json!({
            "items": [{
                "id": "6949805467799537964",
                "name": {"zh_cn": "关键职位招聘需求模板", "en_us": "Recruitment requirements template"},
                "object_list": [{
                    "id": "6949805467799537964",
                    "name": {"zh_cn": "默认模块", "en_us": "Default module"},
                    "description": {"zh_cn": "通用模块", "en_us": "Generic module"},
                    "setting": {"object_type": 11, "config": {}},
                    "is_customized": true,
                    "is_required": false,
                    "active_status": 1,
                    "children_list": [{
                        "id": "6949805467799537964",
                        "name": {"zh_cn": "工作地点", "en_us": "workplace"},
                        "setting": {
                            "object_type": 1,
                            "config": {
                                "options": [{
                                    "key": "Beijing",
                                    "name": {"zh_cn": "北京", "en_us": "Beijing"},
                                    "description": {"zh_cn": "北京", "en_us": "Beijing"},
                                    "active_status": 1
                                }]
                            }
                        },
                        "parent_id": "6949805467799537964",
                        "is_customized": true,
                        "is_required": false,
                        "active_status": 1
                    }]
                }]
            }],
            "has_more": true
        }),
    );
    let jr_schema = &job_requirement_schemas.items[0];
    assert_eq!(jr_schema.id.as_deref(), Some("6949805467799537964"));
    assert_eq!(
        jr_schema.name.as_ref().and_then(|v| v.zh_cn.as_deref()),
        Some("关键职位招聘需求模板")
    );
    let module = jr_schema
        .object_list
        .as_ref()
        .and_then(|v| v.first())
        .expect("object_list 应存在");
    assert_eq!(
        module.setting.as_ref().and_then(|v| v.object_type),
        Some(11)
    );
    let child = module
        .children_list
        .as_ref()
        .and_then(|v| v.first())
        .expect("children_list 应存在");
    assert_eq!(child.parent_id.as_deref(), Some("6949805467799537964"));
    let option = child
        .setting
        .as_ref()
        .and_then(|v| v.config.as_ref())
        .and_then(|v| v.options.as_ref())
        .and_then(|v| v.first())
        .expect("options 应存在");
    assert_eq!(option.key.as_deref(), Some("Beijing"));

    let offer_templates: offer_approval_template::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "tpl_1",
            "name": {"zh_cn": "默认 Offer 审批", "en_us": "Default Offer Approval"}
        }]
    }));
    assert_eq!(offer_templates.items[0].id.as_deref(), Some("tpl_1"));

    let tests: hire_test::search::SearchResponse = parse_contract(json!({
        "items": [{
            "test_id": "test_1",
            "application_id": "app_1",
            "test_paper_id": "paper_1",
            "test_paper_name": "Java 笔试",
            "reply_status": 1,
            "test_status": 3,
            "test_schedule": {
                "start_time": "1621390695000",
                "end_time": "1632839400000"
            },
            "result_detail_list": [{ "subject": "Java 基础", "result": "合格" }],
            "score": "95"
        }],
        "has_more": false
    }));
    assert_eq!(tests.items[0].test_id.as_deref(), Some("test_1"));
    assert_eq!(tests.items[0].test_paper_id.as_deref(), Some("paper_1"));
    assert_eq!(tests.items[0].test_status, Some(3));
    assert_eq!(
        tests.items[0]
            .test_schedule
            .as_ref()
            .and_then(|s| s.start_time.as_deref()),
        Some("1621390695000")
    );

    let logs: talent_operation_log::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "log_1",
            "talent_id": "talent_1",
            "operator": {"id": "ou_operator"},
            "operation_type": "create"
        }]
    }));
    assert_eq!(logs.items[0].operation_type.as_deref(), Some("create"));

    let diversity: diversity_inclusion::search::SearchResponse = parse_contract(json!({
        "items": [{
            "application_id": "app_1",
            "talent_id": "talent_1",
            "gender": "female"
        }]
    }));
    assert_eq!(diversity.items[0].gender.as_deref(), Some("female"));

    let publish_records: job_publish_record::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "record_1",
            "title": "后端开发",
            "job_id": "job_1",
            "job_code": "A106043",
            "job_expire_time": "1722441599999",
            "job_active_status": 1,
            "job_process_type": 1,
            "job_recruitment_type": {
                "id": "101",
                "name": {"zh_cn": "全职", "en_us": "Full-time"}
            }
        }]
    }));
    assert_eq!(
        publish_records.items[0].job_code.as_deref(),
        Some("A106043")
    );
    assert_eq!(publish_records.items[0].job_active_status, Some(1));
    assert_eq!(
        publish_records.items[0]
            .job_recruitment_type
            .as_ref()
            .and_then(|t| t.id.as_deref()),
        Some("101")
    );

    let queried_locations: location::query::QueryResponse = parse_contract(json!({
        "items": [{
            "country": {
                "country_code": "CN_1",
                "country_name_info": {"zh_name": "上海", "en_name": "Shanghai", "py_name": "shanghai"}
            }
        }]
    }));
    assert_eq!(
        queried_locations.items[0]
            .country
            .as_ref()
            .and_then(|v| v.country_code.as_deref()),
        Some("CN_1")
    );
}
