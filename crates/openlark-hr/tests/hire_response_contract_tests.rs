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

    let detail: application::get_detail::GetDetailResponse = parse_contract(json!({
        "application_id": "app_1",
        "talent_info": {
            "talent_id": "talent_1",
            "talent_name": "张三",
            "email": "zhangsan@example.com"
        },
        "job_info": {"job_id": "job_1", "job_name": "后端工程师"},
        "offer_info": {"offer_id": "offer_1", "offer_status": 2}
    }));
    assert_eq!(
        detail
            .talent_info
            .as_ref()
            .and_then(|v| v.talent_name.as_deref()),
        Some("张三")
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
        "records": [{
            "interview_id": "iv_1",
            "interview_round_id": "round_1",
            "interview_round_name": "一面",
            "status": 2,
            "interviewer": {"id": "ou_1", "name": {"zh_cn": "面试官", "en_us": "Interviewer"}},
            "score": {"score": 88.0, "total_score": 100.0}
        }],
        "has_more": false
    }));
    assert_eq!(interviews.records[0].interview_id.as_deref(), Some("iv_1"));
    assert_eq!(
        interviews.records[0].score.as_ref().and_then(|v| v.score),
        Some(88.0)
    );
}

#[test]
fn job_contracts_are_typed() {
    let created: job::combined_create::CombinedCreateResponse = parse_contract(json!({
        "job_id": "job_1",
        "active_status": 1,
        "success": true
    }));
    assert_eq!(created.job_id.as_deref(), Some("job_1"));

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

    let detail: job::get_detail::GetDetailResponse = parse_contract(json!({
        "job_id": "job_1",
        "job_name": "后端工程师",
        "job_description": "负责核心服务开发",
        "recruiters": [{
            "recruiter_id": "rec_1",
            "name": "招聘负责人"
        }]
    }));
    assert_eq!(detail.job_description.as_deref(), Some("负责核心服务开发"));

    let listed: job::list::ListResponse = parse_contract(json!({
        "items": [{
            "job_id": "job_1",
            "job_name": "后端工程师",
            "active_status": 1,
            "process_name": "标准流程"
        }],
        "has_more": true,
        "page_token": "cursor_job"
    }));
    assert_eq!(listed.items[0].job_id.as_deref(), Some("job_1"));
    assert_eq!(listed.page_token.as_deref(), Some("cursor_job"));

    let config: job::config::ConfigResponse = parse_contract(json!({
        "job_id": "job_1",
        "process_id": "process_1",
        "process_name": "标准流程",
        "job_requirement_schema_id": "schema_job",
        "offer_application_form_id": "form_offer"
    }));
    assert_eq!(config.config.process_id.as_deref(), Some("process_1"));

    let recruiters: job::recruiter::RecruiterResponse = parse_contract(json!({
        "recruiters": [{
            "recruiter_id": "rec_1",
            "user_id": "ou_1",
            "name": "招聘负责人",
            "role_type": 1
        }],
        "has_more": false
    }));
    assert_eq!(recruiters.recruiters[0].user_id.as_deref(), Some("ou_1"));

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
        "external_application_id": "ext_app_1",
        "application_id": "app_1",
        "status": 1,
        "success": true
    }));
    assert_eq!(
        ext_app_created.external_application_id.as_deref(),
        Some("ext_app_1")
    );

    let ext_app_list: external_application::list::ListResponse = parse_contract(json!({
        "items": [{
            "external_application_id": "ext_app_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "job_id": "job_1",
            "status": 1,
            "source_name": "Boss"
        }],
        "has_more": false
    }));
    assert_eq!(ext_app_list.items[0].source_name.as_deref(), Some("Boss"));

    let ext_app_updated: external_application::update::UpdateResponse = parse_contract(json!({
        "external_application_id": "ext_app_1",
        "result": true
    }));
    assert_eq!(ext_app_updated.result, Some(true));

    let ext_offer_created: external_offer::create::CreateResponse = parse_contract(json!({
        "external_offer_id": "ext_offer_1",
        "offer_id": "offer_1",
        "status": 2,
        "success": true
    }));
    assert_eq!(ext_offer_created.offer_id.as_deref(), Some("offer_1"));

    let ext_offer_list: external_offer::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "external_offer_id": "ext_offer_1",
            "offer_id": "offer_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 2
        }],
        "page_token": "cursor_ext_offer"
    }));
    assert_eq!(ext_offer_list.items[0].status, Some(2));
    assert_eq!(
        ext_offer_list.page_token.as_deref(),
        Some("cursor_ext_offer")
    );

    let ext_offer_updated: external_offer::update::UpdateResponse = parse_contract(json!({
        "external_offer_id": "ext_offer_1",
        "offer_id": "offer_1",
        "result": true
    }));
    assert_eq!(
        ext_offer_updated.external_offer_id.as_deref(),
        Some("ext_offer_1")
    );
}

#[test]
fn website_channel_delivery_and_job_post_contracts_are_typed() {
    let channel_created: website::channel::create::CreateResponse = parse_contract(json!({
        "channel": {
            "channel_id": "channel_1",
            "website_id": "site_1",
            "name": "Boss",
            "code": "boss",
            "active_status": 1
        }
    }));
    assert_eq!(
        channel_created
            .channel
            .as_ref()
            .and_then(|v| v.channel_id.as_deref()),
        Some("channel_1")
    );

    let channels: website::channel::list::ListResponse = parse_contract(json!({
        "items": [{
            "channel_id": "channel_1",
            "website_id": "site_1",
            "name": "Boss",
            "code": "boss",
            "active_status": 1
        }],
        "has_more": false
    }));
    assert_eq!(channels.items[0].code.as_deref(), Some("boss"));

    let delivery: website::delivery_task::get::GetResponse = parse_contract(json!({
        "delivery_task_id": "task_1",
        "application_id": "app_1",
        "talent_id": "talent_1",
        "status": 2,
        "error_message": "invalid resume"
    }));
    assert_eq!(delivery.delivery_task.status, Some(2));

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
            "agency_id": "agency_1",
            "name": "猎企 A",
            "code": "A001",
            "active_status": 1
        }],
        "has_more": false
    }));
    assert_eq!(batch.items[0].agency_id.as_deref(), Some("agency_1"));

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
        "items": [{
            "protection_id": "protect_1",
            "agency_id": "agency_1",
            "talent_id": "talent_1",
            "status": 1
        }],
        "page_token": "cursor_protect"
    }));
    assert_eq!(protect_search.page_token.as_deref(), Some("cursor_protect"));

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
            "job_requirement_id": "jr_1",
            "job_id": "job_1",
            "name": "需求 A"
        }]
    }));
    assert_eq!(
        requirement_by_id.items[0].job_requirement_id.as_deref(),
        Some("jr_1")
    );

    let requirement_create: job_requirement::create::CreateResponse = parse_contract(json!({
        "job_requirement_id": "jr_1",
        "job_id": "job_1",
        "title": "招聘需求 A",
        "status": 1
    }));
    assert_eq!(requirement_create.job_id.as_deref(), Some("job_1"));

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

    let transfer_onboard: application::transfer_onboard::TransferOnboardResponse =
        parse_contract(json!({
            "application_id": "app_1",
            "employee_id": "emp_1",
            "onboard_status": 1,
            "success": true
        }));
    assert_eq!(
        transfer_onboard.operation.employee_id.as_deref(),
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
            "external_background_check_id": "bg_1",
            "application_id": "app_1",
            "status": 1,
            "success": true
        }));
    assert_eq!(
        background_created.external_background_check_id.as_deref(),
        Some("bg_1")
    );

    let interviews: external_interview::batch_query::BatchQueryResponse = parse_contract(json!({
        "items": [{
            "external_interview_id": "iv_1",
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

    let interview_updated: external_interview::update::UpdateResponse = parse_contract(json!({
        "external_interview_id": "iv_1",
        "application_id": "app_1",
        "result": true
    }));
    assert_eq!(interview_updated.result, Some(true));

    let agreements: tripartite_agreement::list::ListResponse = parse_contract(json!({
        "items": [{
            "agreement_id": "agree_1",
            "application_id": "app_1",
            "talent_id": "talent_1",
            "status": 1,
            "sign_status": 2
        }],
        "has_more": false
    }));
    assert_eq!(agreements.items[0].agreement_id.as_deref(), Some("agree_1"));

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
        "account_id": "acc_1",
        "result": true,
        "success": true
    }));
    assert_eq!(
        referral_enable.operation.account_id.as_deref(),
        Some("acc_1")
    );

    let assessment: external_interview_assessment::create::CreateResponse = parse_contract(json!({
        "external_interview_assessment_id": "assess_1",
        "external_interview_id": "iv_1",
        "status": 1
    }));
    assert_eq!(
        assessment
            .assessment
            .external_interview_assessment_id
            .as_deref(),
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
        "interviewer_id": "int_1",
        "user_id": "ou_1",
        "verify_status": 2
    }));
    assert_eq!(
        interviewer_patch.interviewer.user_id.as_deref(),
        Some("ou_1")
    );

    let manager_update: job::manager::batch_update::BatchUpdateResponse = parse_contract(json!({
        "job_id": "job_1",
        "manager_id": "mgr_1",
        "success": true
    }));
    assert_eq!(
        manager_update.operation.manager_id.as_deref(),
        Some("mgr_1")
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
        "exam_id": "exam_1",
        "application_id": "app_1",
        "success": true
    }));
    assert_eq!(exam_result.operation.exam_id.as_deref(), Some("exam_1"));

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
            "id": "schema_1",
            "title": {"zh_cn": "标准职位模板", "en_us": "Default Job Schema"},
            "version": 3
        }]
    }));
    assert_eq!(job_schemas.items[0].version, Some(3));

    let interview_forms: interview_feedback_form::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "form_1",
            "title": {"zh_cn": "通用评价表", "en_us": "Default Form"}
        }],
        "has_more": false
    }));
    assert_eq!(
        interview_forms.items[0]
            .title
            .as_ref()
            .and_then(|value| value.zh_cn_or_plain()),
        Some("通用评价表")
    );

    let interview_schemas: interview_registration_schema::list::ListResponse =
        parse_contract(json!({
            "items": [{
                "id": "schema_interview_1",
                "name": {"zh_cn": "校招登记表", "en_us": "Campus Schema"},
                "status": 1
            }],
            "page_token": "cursor_5"
        }));
    assert_eq!(interview_schemas.page_token.as_deref(), Some("cursor_5"));

    let round_types: interview_round_type::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "round_1",
            "description": {"zh_cn": "技术一面", "en_us": "Tech Round 1"}
        }]
    }));
    assert_eq!(
        round_types.items[0]
            .description
            .as_ref()
            .and_then(|value| value.zh_cn_or_plain()),
        Some("技术一面")
    );

    let interview_tasks: interview_task::list::ListResponse = parse_contract(json!({
        "items": [{
            "interview_id": "interview_1",
            "application_id": "app_1",
            "status": 2,
            "interviewer": {"id": "ou_interviewer"}
        }]
    }));
    assert_eq!(
        interview_tasks.items[0].interview_id.as_deref(),
        Some("interview_1")
    );

    let registrations: registration_schema::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "reg_schema_1",
            "name": {"zh_cn": "默认申请表", "en_us": "Apply Schema"}
        }]
    }));
    assert_eq!(registrations.items[0].id.as_deref(), Some("reg_schema_1"));

    let portal_schemas: portal_apply_schema::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "portal_schema_1",
            "title": {"zh_cn": "官网投递", "en_us": "Portal Apply"}
        }]
    }));
    assert_eq!(
        portal_schemas.items[0].id.as_deref(),
        Some("portal_schema_1")
    );

    let questionnaires: questionnaire::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "questionnaire_1",
            "name": {"zh_cn": "问卷 A", "en_us": "Questionnaire A"}
        }]
    }));
    assert_eq!(
        questionnaires.items[0].id.as_deref(),
        Some("questionnaire_1")
    );

    let resume_sources: resume_source::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "source_1",
            "code": "campus",
            "name": {"zh_cn": "校园招聘", "en_us": "Campus"}
        }]
    }));
    assert_eq!(resume_sources.items[0].code.as_deref(), Some("campus"));

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
            "id": "process_1",
            "active_status": 1,
            "name": {"zh_cn": "标准流程", "en_us": "Default Process"}
        }]
    }));
    assert_eq!(job_processes.items[0].active_status, Some(1));

    let job_requirement_schemas: job_requirement_schema::list::ListResponse =
        parse_contract(json!({
            "items": [{
                "id": "jr_schema_1",
                "version": 7
            }]
        }));
    assert_eq!(job_requirement_schemas.items[0].version, Some(7));

    let offer_templates: offer_approval_template::list::ListResponse = parse_contract(json!({
        "items": [{
            "id": "tpl_1",
            "name": {"zh_cn": "默认 Offer 审批", "en_us": "Default Offer Approval"}
        }]
    }));
    assert_eq!(offer_templates.items[0].id.as_deref(), Some("tpl_1"));

    let tests: hire_test::search::SearchResponse = parse_contract(json!({
        "items": [{
            "id": "test_1",
            "title": {"zh_cn": "Java 笔试", "en_us": "Java Test"}
        }],
        "has_more": false
    }));
    assert_eq!(tests.items[0].id.as_deref(), Some("test_1"));

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
            "job_id": "job_1",
            "channel_id": "channel_1",
            "publish_status": 2
        }]
    }));
    assert_eq!(
        publish_records.items[0].channel_id.as_deref(),
        Some("channel_1")
    );

    let queried_locations: location::query::QueryResponse = parse_contract(json!({
        "items": [{
            "id": "loc_1",
            "name": {"zh_cn": "上海", "en_us": "Shanghai"}
        }]
    }));
    assert_eq!(queried_locations.items[0].id.as_deref(), Some("loc_1"));
}
