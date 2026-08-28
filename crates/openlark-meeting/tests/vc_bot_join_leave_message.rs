//! #645 VC bot join / leave / message 公开契约测试。
//!
//! 验收缝：
//! - 空 meeting_id / 空 content / 缺失 join_type 或 join_identify 在发请求前失败
//! - 三个 POST 的 method/path 与 catalog url 一致，响应 data 为强类型
//! - 现有 events / user_active_meeting 读接口路径不变
#![cfg(feature = "vc")]

use openlark_core::{config::Config, req_option::RequestOption};
use openlark_meeting::vc::vc::v1::bot::{
    GetBotEventsRequest, GetUserActiveMeetingRequest, JoinBotBody, JoinBotRequest, JoinIdentify,
    LeaveBotBody, LeaveBotRequest, SendBotMessageBody, SendBotMessageRequest,
};
use serde_json::json;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, header, method, path},
};

fn test_config(base_url: impl Into<String>) -> Config {
    Config::builder()
        .app_id("ci_app_id")
        .app_secret("ci_app_secret")
        .base_url(base_url)
        .enable_token_cache(false)
        .build()
}

fn tenant_option() -> RequestOption {
    RequestOption::builder()
        .tenant_access_token("tenant-token")
        .build()
}

async fn assert_no_http(server: &MockServer) {
    assert!(
        server
            .received_requests()
            .await
            .unwrap_or_default()
            .is_empty()
    );
}

#[tokio::test]
async fn join_rejects_missing_join_type_before_sending_request() {
    let server = MockServer::start().await;

    let result = JoinBotRequest::new(test_config(server.uri()))
        .execute(JoinBotBody {
            join_type: 0,
            join_identify: JoinIdentify {
                meeting_no: "123456789".to_string(),
            },
            ..Default::default()
        })
        .await;

    let error = result.expect_err("缺失 join_type 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("join_type"));
    assert_no_http(&server).await;
}

#[tokio::test]
async fn join_rejects_missing_join_identify_before_sending_request() {
    let server = MockServer::start().await;

    let result = JoinBotRequest::new(test_config(server.uri()))
        .execute(JoinBotBody {
            join_type: 1,
            join_identify: JoinIdentify {
                meeting_no: "   ".to_string(),
            },
            ..Default::default()
        })
        .await;

    let error = result.expect_err("缺失 join_identify 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("join_identify"));
    assert_no_http(&server).await;
}

#[tokio::test]
async fn leave_rejects_empty_meeting_id_before_sending_request() {
    let server = MockServer::start().await;

    let result = LeaveBotRequest::new(test_config(server.uri()))
        .execute(LeaveBotBody {
            meeting_id: "   ".to_string(),
        })
        .await;

    let error = result.expect_err("空 meeting_id 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("meeting_id"));
    assert_no_http(&server).await;
}

#[tokio::test]
async fn message_rejects_empty_meeting_id_before_sending_request() {
    let server = MockServer::start().await;

    let result = SendBotMessageRequest::new(test_config(server.uri()))
        .execute(SendBotMessageBody {
            meeting_id: "".to_string(),
            msg_type: "text".to_string(),
            content: "hello".to_string(),
            uuid: None,
        })
        .await;

    let error = result.expect_err("空 meeting_id 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("meeting_id"));
    assert_no_http(&server).await;
}

#[tokio::test]
async fn message_rejects_empty_content_before_sending_request() {
    let server = MockServer::start().await;

    let result = SendBotMessageRequest::new(test_config(server.uri()))
        .execute(SendBotMessageBody {
            meeting_id: "om_xxx".to_string(),
            msg_type: "text".to_string(),
            content: "  ".to_string(),
            uuid: None,
        })
        .await;

    let error = result.expect_err("空 content 应在发起网络请求前被拒绝");
    assert!(error.to_string().contains("content"));
    assert_no_http(&server).await;
}

#[tokio::test]
async fn join_posts_official_contract_and_parses_typed_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/vc/v1/bots/join"))
        .and(header("authorization", "Bearer tenant-token"))
        .and(body_json(json!({
            "join_type": 1,
            "join_identify": { "meeting_no": "123456789" },
            "password": "pwd"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {
                "meeting": {
                    "id": "om_join",
                    "meeting_no": "123456789",
                    "start_time": "1600000000",
                    "topic": "standup"
                },
                "join_user": { "id": "ou_bot", "user_type": 1 }
            }
        })))
        .mount(&server)
        .await;

    let resp = JoinBotRequest::new(test_config(server.uri()))
        .execute_with_options(
            JoinBotBody {
                join_type: 1,
                join_identify: JoinIdentify {
                    meeting_no: "123456789".to_string(),
                },
                password: Some("pwd".to_string()),
                call_id: None,
            },
            tenant_option(),
        )
        .await
        .expect("加入会议应成功");

    let meeting = resp.meeting.expect("响应应包含 meeting");
    assert_eq!(meeting.id.as_deref(), Some("om_join"));
    assert_eq!(meeting.meeting_no.as_deref(), Some("123456789"));
    let join_user = resp.join_user.expect("响应应包含 join_user");
    assert_eq!(join_user.id.as_deref(), Some("ou_bot"));
    assert_eq!(join_user.user_type, Some(1));
}

#[tokio::test]
async fn leave_posts_official_contract_and_parses_typed_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/vc/v1/bots/leave"))
        .and(header("authorization", "Bearer tenant-token"))
        .and(body_json(json!({ "meeting_id": "om_join" })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": { "leave_user": { "id": "ou_bot", "user_type": 1 } }
        })))
        .mount(&server)
        .await;

    let resp = LeaveBotRequest::new(test_config(server.uri()))
        .execute_with_options(
            LeaveBotBody {
                meeting_id: "om_join".to_string(),
            },
            tenant_option(),
        )
        .await
        .expect("离开会议应成功");

    let leave_user = resp.leave_user.expect("响应应包含 leave_user");
    assert_eq!(leave_user.id.as_deref(), Some("ou_bot"));
    assert_eq!(leave_user.user_type, Some(1));
}

#[tokio::test]
async fn message_posts_official_contract_and_parses_typed_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/open-apis/vc/v1/bots/message"))
        .and(header("authorization", "Bearer tenant-token"))
        .and(body_json(json!({
            "meeting_id": "om_join",
            "msg_type": "text",
            "content": "hello"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": { "uuid": "msg-uuid-1" }
        })))
        .mount(&server)
        .await;

    let resp = SendBotMessageRequest::new(test_config(server.uri()))
        .execute_with_options(
            SendBotMessageBody {
                meeting_id: "om_join".to_string(),
                msg_type: "text".to_string(),
                content: "hello".to_string(),
                uuid: None,
            },
            tenant_option(),
        )
        .await
        .expect("发送会中消息应成功");

    assert_eq!(resp.uuid.as_deref(), Some("msg-uuid-1"));
}

#[tokio::test]
async fn existing_bot_events_get_path_is_unchanged() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/vc/v1/bots/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {}
        })))
        .mount(&server)
        .await;

    GetBotEventsRequest::new(test_config(server.uri()))
        .execute_with_options(tenant_option())
        .await
        .expect("获取会议事件应成功");
}

#[tokio::test]
async fn existing_user_active_meeting_get_path_is_unchanged() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/open-apis/vc/v1/bots/user_active_meeting"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "code": 0,
            "msg": "success",
            "data": {}
        })))
        .mount(&server)
        .await;

    GetUserActiveMeetingRequest::new(test_config(server.uri()))
        .execute_with_options(tenant_option())
        .await
        .expect("获取用户活跃会议应成功");
}
