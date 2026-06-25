//! attendance tests 集成测试。
#![cfg(feature = "attendance")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_hr::attendance::attendance::v1::{
    approval_info, file, group, shift, user_approval, user_setting, user_stats_data,
    user_stats_field, user_task, user_task_remedy,
};
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
        test_create_group_request_builder,
        group::CreateGroupRequest::new(test_config("https://open.feishu.cn"))
            .group_name("技术部考勤组".to_string())
            .group_type(0)
    );
    smoke_builder!(
        test_update_group_request_builder,
        group::CreateGroupRequest::new(test_config("https://open.feishu.cn"))
            .group_id("g_update_1".to_string())
            .group_name("技术部考勤组-更新".to_string())
            .group_type(1)
    );
    smoke_builder!(
        test_get_group_request_builder,
        group::GetGroupRequest::new(test_config("https://open.feishu.cn"))
            .group_id("g_1".to_string())
    );
    smoke_builder!(
        test_delete_group_request_builder,
        group::DeleteGroupRequest::new(test_config("https://open.feishu.cn"))
            .group_id("g_1".to_string())
    );
    smoke_builder!(
        test_list_group_request_builder,
        group::ListGroupRequest::new(test_config("https://open.feishu.cn"))
            .page_size(50)
            .page_token("next_1".to_string())
    );
    smoke_builder!(
        test_search_group_request_builder,
        group::SearchGroupRequest::new(test_config("https://open.feishu.cn"))
            .group_name("技术部".to_string())
            .page_size(20)
            .page_token("next_2".to_string())
    );
    smoke_builder!(
        test_list_user_group_request_builder,
        group::list_user::ListUserRequest::new(
            test_config("https://open.feishu.cn"),
            "g_1".to_string()
        )
        .page_size(20)
        .page_token("next_3".to_string())
    );

    smoke_builder!(
        test_create_shift_request_builder,
        shift::CreateShiftRequest::new(test_config("https://open.feishu.cn"))
            .shift_name("标准班次".to_string())
            .shift_type(0)
            .punch_times(vec![shift::PunchTime {
                on_duty_time: 540,
                off_duty_time: 1080,
                earliest_on_duty_time: None,
                latest_off_duty_time: None,
                on_duty_places: None,
                off_duty_places: None,
                on_duty_wifis: None,
                off_duty_wifis: None,
            }])
    );
    smoke_builder!(
        test_update_shift_request_builder,
        shift::CreateShiftRequest::new(test_config("https://open.feishu.cn"))
            .shift_id("s_1".to_string())
            .shift_name("标准班次-更新".to_string())
            .shift_type(0)
            .punch_times(vec![shift::PunchTime {
                on_duty_time: 540,
                off_duty_time: 1080,
                earliest_on_duty_time: None,
                latest_off_duty_time: None,
                on_duty_places: None,
                off_duty_places: None,
                on_duty_wifis: None,
                off_duty_wifis: None,
            }])
    );
    smoke_builder!(
        test_get_shift_request_builder,
        shift::GetShiftRequest::new(test_config("https://open.feishu.cn"))
            .shift_id("s_1".to_string())
    );
    smoke_builder!(
        test_delete_shift_request_builder,
        shift::DeleteShiftRequest::new(test_config("https://open.feishu.cn"))
            .shift_id("s_1".to_string())
    );
    smoke_builder!(
        test_list_shift_request_builder,
        shift::ListShiftRequest::new(test_config("https://open.feishu.cn"))
            .page_size(50)
            .page_token("next_shift".to_string())
    );

    smoke_builder!(
        test_get_user_task_request_builder,
        user_task::QueryUserTaskRequest::new(test_config("https://open.feishu.cn"))
            .start_date("2026-01-01".to_string())
            .end_date("2026-01-31".to_string())
            .user_ids(vec!["ou_1".to_string()])
            .check_in_type(1)
    );
    smoke_builder!(
        test_remedy_user_task_request_builder,
        user_task_remedy::create::CreateRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
            1735689600,
            1735693200,
            "补卡".to_string(),
        )
    );
    smoke_builder!(
        test_query_user_task_remedy_request_builder,
        user_task_remedy::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .user_id("ou_1".to_string())
            .start_time(1735689600)
            .end_time(1735776000)
            .page_size(20)
    );
    smoke_builder!(
        test_query_user_allowed_remedys_request_builder,
        user_task_remedy::query_user_allowed_remedys::QueryUserAllowedRemedysRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
        )
    );

    smoke_builder!(
        test_get_user_stats_data_request_builder,
        user_stats_data::QueryRequest::new(test_config("https://open.feishu.cn"))
            .start_date("2026-01-01".to_string())
            .end_date("2026-01-31".to_string())
            .user_ids(vec!["ou_1".to_string()])
            .stats_type("day".to_string())
            .page_size(100)
    );
    smoke_builder!(
        test_create_user_approval_request_builder,
        user_approval::create::CreateRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
            1,
            1,
            1735689600,
        )
        .remark("同意".to_string())
    );
    smoke_builder!(
        test_get_approval_info_request_builder,
        user_approval::query::QueryRequest::new(test_config("https://open.feishu.cn"))
            .user_id("ou_1".to_string())
            .approval_type(1)
            .page_size(20)
    );
    smoke_builder!(
        test_process_approval_request_builder,
        approval_info::process::ProcessRequest::new(
            test_config("https://open.feishu.cn"),
            "ins_1".to_string(),
            1,
        )
        .comment("通过".to_string())
    );

    smoke_builder!(
        test_upload_file_request_builder,
        file::upload::UploadRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
            "face.jpg".to_string(),
            vec![1, 2, 3],
        )
    );
    smoke_builder!(
        test_download_file_request_builder,
        file::download::DownloadRequest::new(
            test_config("https://open.feishu.cn"),
            "p_1".to_string()
        )
    );
    smoke_builder!(
        test_get_user_setting_request_builder,
        user_setting::query::QueryRequest::new(
            test_config("https://open.feishu.cn"),
            vec!["ou_1".to_string()],
        )
    );
    smoke_builder!(
        test_update_user_setting_request_builder,
        user_setting::modify::ModifyRequest::new(
            test_config("https://open.feishu.cn"),
            "ou_1".to_string(),
            true,
        )
    );
    smoke_builder!(
        test_get_user_stats_field_request_builder,
        user_stats_field::QueryRequest::new(test_config("https://open.feishu.cn"))
            .unit_id("g_1".to_string())
            .stat_type("daily".to_string())
            .add_user_id("ou_1".to_string())
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[rstest]
    #[case("")]
    #[case("   ")]
    #[tokio::test]
    async fn test_group_name_validation(#[case] group_name: &str) {
        let result = group::CreateGroupRequest::new(test_config("https://127.0.0.1:9"))
            .group_name(group_name.to_string())
            .group_type(0)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("考勤组名称不能为空"));
    }

    #[tokio::test]
    async fn test_shift_validation_requires_punch_times() {
        let result = shift::CreateShiftRequest::new(test_config("https://127.0.0.1:9"))
            .shift_name("标准班次".to_string())
            .shift_type(0)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("打卡时段不能为空"));
    }

    #[tokio::test]
    async fn test_user_stats_data_validation_requires_user_or_group() {
        let result = user_stats_data::QueryRequest::new(test_config("https://127.0.0.1:9"))
            .start_date("2026-01-01".to_string())
            .end_date("2026-01-31".to_string())
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("至少需要指定用户 ID 或考勤组 ID"));
    }

    #[rstest]
    #[case(0)]
    #[case(201)]
    #[tokio::test]
    async fn test_user_stats_data_page_size_boundary(#[case] page_size: i32) {
        let result = user_stats_data::QueryRequest::new(test_config("https://127.0.0.1:9"))
            .start_date("2026-01-01".to_string())
            .end_date("2026-01-31".to_string())
            .user_ids(vec!["ou_1".to_string()])
            .page_size(page_size)
            .execute_with_options(auth_option())
            .await;

        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("分页大小必须在 1-200 之间"));
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/groups"))
            .and(header("Authorization", "Bearer test_token"))
            .and(body_json(
                json!({"group_name":"技术部考勤组","group_type":0}),
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"group_id":"g_123"}})),
            )
            .mount(&mock_server)
            .await;

        let resp = group::CreateGroupRequest::new(test_config(&mock_server.uri()))
            .group_name("技术部考勤组".to_string())
            .group_type(0)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.group_id, "g_123");
    }

    #[tokio::test]
    async fn test_get_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/groups/g_123"))
            .and(header("Authorization", "Bearer test_token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"group":{"group_id":"g_123","group_name":"技术部","group_type":0}}
            })))
            .mount(&mock_server)
            .await;

        let resp = group::GetGroupRequest::new(test_config(&mock_server.uri()))
            .group_id("g_123".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.group.group_id, "g_123");
    }

    #[tokio::test]
    async fn test_delete_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/attendance/v1/groups/g_123"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"result":true}})),
            )
            .mount(&mock_server)
            .await;

        let resp = group::DeleteGroupRequest::new(test_config(&mock_server.uri()))
            .group_id("g_123".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_list_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/groups"))
            .and(query_param("page_size", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"group_list":[{"group_id":"g_1","group_name":"技术部","group_type":0}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = group::ListGroupRequest::new(test_config(&mock_server.uri()))
            .page_size(20)
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.group_list.len(), 1);
    }

    #[tokio::test]
    async fn test_search_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/groups/search"))
            .and(query_param("group_name", "技术部"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"group_list":[{"group_id":"g_1","group_name":"技术部","group_type":0}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = group::SearchGroupRequest::new(test_config(&mock_server.uri()))
            .group_name("技术部".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.group_list[0].group_name, "技术部");
    }

    #[tokio::test]
    async fn test_list_user_group_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/groups/g_1/list_user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"items":[{"user_id":"ou_1","join_time":1735689600}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = group::list_user::ListUserRequest::new(
            test_config(&mock_server.uri()),
            "g_1".to_string(),
        )
        .execute_with_options(auth_option())
        .await
        .unwrap();
        assert_eq!(resp.items[0].user_id, "ou_1");
    }

    #[tokio::test]
    async fn test_create_shift_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/shifts"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"shift_id":"s_123"}})),
            )
            .mount(&mock_server)
            .await;

        let resp = shift::CreateShiftRequest::new(test_config(&mock_server.uri()))
            .shift_name("标准班次".to_string())
            .shift_type(0)
            .punch_times(vec![shift::PunchTime {
                on_duty_time: 540,
                off_duty_time: 1080,
                earliest_on_duty_time: None,
                latest_off_duty_time: None,
                on_duty_places: None,
                off_duty_places: None,
                on_duty_wifis: None,
                off_duty_wifis: None,
            }])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.shift_id, "s_123");
    }

    #[tokio::test]
    async fn test_get_shift_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/shifts/s_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"shift":{"shift_id":"s_123","shift_name":"标准班次","shift_type":0,"punch_times":[]}}
            })))
            .mount(&mock_server)
            .await;

        let resp = shift::GetShiftRequest::new(test_config(&mock_server.uri()))
            .shift_id("s_123".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.shift.shift_name, "标准班次");
    }

    #[tokio::test]
    async fn test_delete_shift_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/attendance/v1/shifts/s_123"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"code":0,"msg":"success","data":{"result":true}})),
            )
            .mount(&mock_server)
            .await;

        let resp = shift::DeleteShiftRequest::new(test_config(&mock_server.uri()))
            .shift_id("s_123".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert!(resp.result);
    }

    #[tokio::test]
    async fn test_list_shift_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/shifts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"shift_list":[{"shift_id":"s_1","shift_name":"标准班次","shift_type":0}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = shift::ListShiftRequest::new(test_config(&mock_server.uri()))
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.shift_list.len(), 1);
    }

    #[tokio::test]
    async fn test_query_shift_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/shifts/query"))
            .and(query_param("shift_name", "标准班次"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"shift_list":[{"shift_id":"s_1","shift_name":"标准班次","shift_type":0}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = shift::QueryShiftRequest::new(test_config(&mock_server.uri()))
            .shift_name("标准班次".to_string())
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.shift_list[0].shift_id, "s_1");
    }

    #[tokio::test]
    async fn test_query_user_task_http_mock() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_tasks/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code":0,"msg":"success","data":{"records":[{"user_id":"ou_1","date":"2026-01-01","check_in_type":1,"check_in_time":"2026-01-01 09:00:00","check_in_result":1,"check_in_method":1}],"has_more":false}
            })))
            .mount(&mock_server)
            .await;

        let resp = user_task::QueryUserTaskRequest::new(test_config(&mock_server.uri()))
            .start_date("2026-01-01".to_string())
            .end_date("2026-01-01".to_string())
            .user_ids(vec!["ou_1".to_string()])
            .execute_with_options(auth_option())
            .await
            .unwrap();
        assert_eq!(resp.records.len(), 1);
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
        test_create_group_response_serialization,
        group::CreateGroupResponse,
        group::CreateGroupResponse {
            group_id: "g_1".to_string()
        }
    );
    roundtrip_eq!(
        test_delete_group_response_serialization,
        group::DeleteGroupResponse,
        group::DeleteGroupResponse { result: true }
    );
    roundtrip_eq!(
        test_get_group_response_serialization,
        group::GetGroupResponse,
        group::GetGroupResponse {
            group: group::GroupInfo {
                group_id: "g_1".to_string(),
                group_name: "技术部".to_string(),
                group_type: 0,
                user_list: None,
                excluded_user_list: None,
                manager_list: None,
                dept_list: None,
                shift_list: None,
                allow_out_punch: None,
                out_punch_need_approval: None,
                allow_pc_punch: None,
                need_photo: None,
                photo_punch_type: None,
                allow_remedy: None,
                remedy_limit: None,
                remedy_period: None,
                work_day_config: None,
                overtime_info: None,
            }
        }
    );
    roundtrip_eq!(
        test_list_group_response_serialization,
        group::ListGroupResponse,
        group::ListGroupResponse {
            group_list: vec![group::GroupListItem {
                group_id: "g_1".to_string(),
                group_name: "技术部".to_string(),
                group_type: 0,
                user_count: Some(10),
                manager_list: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_search_group_response_serialization,
        group::SearchGroupResponse,
        group::SearchGroupResponse {
            group_list: vec![],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );
    roundtrip_eq!(
        test_list_user_response_serialization,
        group::list_user::ListUserResponse,
        group::list_user::ListUserResponse {
            items: vec![group::list_user::GroupMember {
                user_id: "ou_1".to_string(),
                user_name: Some("张三".to_string()),
                join_time: 1735689600,
            }],
            has_more: false,
            page_token: None,
        }
    );

    roundtrip_eq!(
        test_create_shift_response_serialization,
        shift::CreateShiftResponse,
        shift::CreateShiftResponse {
            shift_id: "s_1".to_string()
        }
    );
    roundtrip_eq!(
        test_delete_shift_response_serialization,
        shift::DeleteShiftResponse,
        shift::DeleteShiftResponse { result: true }
    );
    roundtrip_eq!(
        test_get_shift_response_serialization,
        shift::GetShiftResponse,
        shift::GetShiftResponse {
            shift: shift::ShiftInfo {
                shift_id: "s_1".to_string(),
                shift_name: "标准班次".to_string(),
                shift_type: 0,
                flexible_minutes: None,
                flexible_on_duty_time: None,
                flexible_off_duty_time: None,
                punch_times: vec![],
                late_rule: None,
                early_leave_rule: None,
                rest_rule: None,
                overtime_rule: None,
                allow_out_punch: None,
                out_punch_need_approval: None,
                allow_pc_punch: None,
                need_photo: None,
                photo_punch_type: None,
                allow_remedy: None,
                remedy_limit: None,
                remedy_period: None,
            }
        }
    );
    roundtrip_eq!(
        test_list_shift_response_serialization,
        shift::ListShiftResponse,
        shift::ListShiftResponse {
            shift_list: vec![shift::ShiftListItem {
                shift_id: "s_1".to_string(),
                shift_name: "标准班次".to_string(),
                shift_type: 0,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_query_shift_response_serialization,
        shift::QueryShiftResponse,
        shift::QueryShiftResponse {
            shift_list: vec![],
            has_more: false,
            page_token: Some("next".to_string()),
        }
    );

    roundtrip_eq!(
        test_query_user_task_response_serialization,
        user_task::models::QueryUserTaskResponse,
        user_task::models::QueryUserTaskResponse {
            records: vec![user_task::models::UserTaskRecord {
                user_id: "ou_1".to_string(),
                date: "2026-01-01".to_string(),
                check_in_type: 1,
                check_in_time: "2026-01-01 09:00:00".to_string(),
                check_in_place_name: None,
                check_in_place_id: None,
                check_in_result: 1,
                check_in_method: 1,
                device_id: None,
                device_name: None,
                wifi_name: None,
                wifi_mac: None,
                remark: None,
                photo_list: None,
                longitude: None,
                latitude: None,
                out_address: None,
                out_remark: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_create_user_task_remedy_response_serialization,
        user_task_remedy::create::CreateResponse,
        user_task_remedy::create::CreateResponse {
            success: true,
            remedy_id: "r_1".to_string(),
            approval_instance_id: "ins_1".to_string(),
        }
    );
    roundtrip_eq!(
        test_query_user_task_remedy_response_serialization,
        user_task_remedy::query::QueryResponse,
        user_task_remedy::query::QueryResponse {
            items: vec![user_task_remedy::query::RemedyRecord {
                remedy_id: "r_1".to_string(),
                user_id: "ou_1".to_string(),
                original_time: 1735689600,
                remedy_time: 1735693200,
                reason: "补卡".to_string(),
                approval_status: 1,
                created_at: 1735696800,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_query_user_allowed_remedys_response_serialization,
        user_task_remedy::query_user_allowed_remedys::QueryUserAllowedRemedysResponse,
        user_task_remedy::query_user_allowed_remedys::QueryUserAllowedRemedysResponse {
            items: vec![
                user_task_remedy::query_user_allowed_remedys::AllowedRemedy {
                    start_time: 1735689600,
                    end_time: 1735776000,
                    day_of_week: 1,
                }
            ],
        }
    );

    roundtrip_eq!(
        test_query_user_stats_data_response_serialization,
        user_stats_data::QueryResponse,
        user_stats_data::QueryResponse {
            items: vec![user_stats_data::UserStatsDataItem {
                user_id: "ou_1".to_string(),
                user_name: Some("张三".to_string()),
                date: "2026-01-01".to_string(),
                group_id: Some("g_1".to_string()),
                group_name: None,
                basic_info: None,
                attendance_stats: Some(json!({"work_hours":8})),
                abnormal_stats: None,
                leave_stats: None,
                overtime_stats: None,
                punch_time: None,
                check_result: None,
                custom_fields: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_create_user_approval_response_serialization,
        user_approval::create::CreateResponse,
        user_approval::create::CreateResponse {
            success: true,
            approval_id: "a_1".to_string(),
        }
    );
    roundtrip_eq!(
        test_query_user_approval_response_serialization,
        user_approval::query::QueryResponse,
        user_approval::query::QueryResponse {
            items: vec![user_approval::query::UserApproval {
                approval_id: "a_1".to_string(),
                user_id: "ou_1".to_string(),
                approval_type: 1,
                status: 1,
                approval_time: 1735689600,
                content: Some(json!({"reason":"正常"})),
            }],
            has_more: false,
            page_token: None,
        }
    );
    roundtrip_eq!(
        test_process_approval_response_serialization,
        approval_info::process::ProcessResponse,
        approval_info::process::ProcessResponse {
            success: true,
            approval_instance_id: "ins_1".to_string(),
        }
    );

    roundtrip_eq!(
        test_upload_file_response_serialization,
        file::upload::UploadResponse,
        file::upload::UploadResponse {
            success: true,
            photo_id: "p_1".to_string(),
            user_id: "ou_1".to_string(),
            upload_time: 1735689600,
        }
    );
    roundtrip_eq!(
        test_download_file_response_serialization,
        file::download::DownloadResponse,
        file::download::DownloadResponse {
            photo_id: "p_1".to_string(),
            user_id: "ou_1".to_string(),
            photo_data: "ZmFrZV9iYXNlNjQ=".to_string(),
            content_type: "image/jpeg".to_string(),
        }
    );
    roundtrip_eq!(
        test_get_user_setting_response_serialization,
        user_setting::query::QueryResponse,
        user_setting::query::QueryResponse {
            items: vec![user_setting::query::UserSetting {
                user_id: "ou_1".to_string(),
                face_recognition_enabled: true,
                photo_id: Some("p_1".to_string()),
                photo_upload_time: Some(1735689600),
            }],
        }
    );
    roundtrip_eq!(
        test_update_user_setting_response_serialization,
        user_setting::modify::ModifyResponse,
        user_setting::modify::ModifyResponse {
            success: true,
            user_id: "ou_1".to_string(),
        }
    );
    roundtrip_eq!(
        test_query_user_stats_field_response_serialization,
        user_stats_field::QueryResponse,
        user_stats_field::QueryResponse {
            stat_fields: vec![user_stats_field::StatField {
                field_id: "f_1".to_string(),
                field_name: Some("出勤天数".to_string()),
                field_type: Some("number".to_string()),
                field_desc: None,
            }],
            has_more: false,
            page_token: None,
        }
    );
}
