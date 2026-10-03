//! Representative contract tests for high-frequency communication request/response models.
#![cfg(all(feature = "im", feature = "contact"))]

use openlark_communication::contact::contact::v3::user::batch::BatchGetUsersResponse;
use openlark_communication::im::v1::chat::models::GetChatLinkResponse;
use openlark_communication::im::v1::file::models::CreateFileResponse;
use openlark_communication::im::v1::image::models::CreateImageResponse;
use openlark_communication::im::v1::message::create::CreateMessageBody;
use openlark_communication::im::v1::message::reply::ReplyMessageBody;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, from_value, json, to_value};

fn assert_json_contract<T>(value: &T, expected: Value)
where
    T: Serialize,
{
    assert_eq!(to_value(value).unwrap(), expected);
}

fn parse_contract<T>(payload: Value) -> T
where
    T: DeserializeOwned,
{
    from_value(payload).unwrap()
}

#[test]
fn message_request_contracts() {
    let create = CreateMessageBody {
        receive_id: "ou_xxx".to_string(),
        msg_type: "text".to_string(),
        content: r#"{"text":"Hello OpenLark"}"#.to_string(),
        uuid: Some("uuid-123".to_string()),
    };
    assert_json_contract(
        &create,
        json!({
            "receive_id": "ou_xxx",
            "msg_type": "text",
            "content": r#"{"text":"Hello OpenLark"}"#,
            "uuid": "uuid-123"
        }),
    );

    let reply = ReplyMessageBody {
        content: r#"{"text":"收到"}"#.to_string(),
        msg_type: "text".to_string(),
        reply_in_thread: Some(true),
        uuid: Some("reply-uuid-1".to_string()),
    };
    assert_json_contract(
        &reply,
        json!({
            "content": r#"{"text":"收到"}"#,
            "msg_type": "text",
            "reply_in_thread": true,
            "uuid": "reply-uuid-1"
        }),
    );
}

#[test]
fn media_response_contracts() {
    let image: CreateImageResponse = parse_contract(json!({
        "image_key": "img_v3_123"
    }));
    assert_eq!(image.image_key, "img_v3_123");
    assert_json_contract(
        &image,
        json!({
            "image_key": "img_v3_123"
        }),
    );

    let file: CreateFileResponse = parse_contract(json!({
        "file_key": "file_v3_456"
    }));
    assert_eq!(file.file_key, "file_v3_456");
    assert_json_contract(
        &file,
        json!({
            "file_key": "file_v3_456"
        }),
    );
}

#[test]
fn contact_batch_user_response_contract() {
    let response: BatchGetUsersResponse = parse_contract(json!({
        "items": [
            {
                "union_id": "union_123",
                "user_id": "user_123",
                "open_id": "ou_123",
                "name": "张三",
                "en_name": "Zhang San",
                "email": "zhangsan@example.com",
                "mobile": "13800138000",
                "mobile_visible": true,
                "gender": 1,
                "avatar_key": "avatar_key_123",
                "avatar": {
                    "avatar_72": "https://cdn.example/avatar_72.png",
                    "avatar_240": "https://cdn.example/avatar_240.png",
                    "avatar_640": "https://cdn.example/avatar_640.png"
                },
                "department_ids": ["od-001", "od-002"]
            }
        ]
    }));
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].open_id.as_deref(), Some("ou_123"));
    assert_eq!(
        response.items[0]
            .avatar
            .as_ref()
            .unwrap()
            .avatar_72
            .as_deref(),
        Some("https://cdn.example/avatar_72.png")
    );
    assert_json_contract(
        &response,
        json!({
            "items": [
                {
                    "union_id": "union_123",
                    "user_id": "user_123",
                    "open_id": "ou_123",
                    "name": "张三",
                    "en_name": "Zhang San",
                    "email": "zhangsan@example.com",
                    "mobile": "13800138000",
                    "mobile_visible": true,
                    "gender": 1,
                    "avatar_key": "avatar_key_123",
                    "avatar": {
                        "avatar_72": "https://cdn.example/avatar_72.png",
                        "avatar_240": "https://cdn.example/avatar_240.png",
                        "avatar_640": "https://cdn.example/avatar_640.png"
                    },
                    "department_ids": ["od-001", "od-002"]
                }
            ]
        }),
    );
}

#[test]
fn chat_link_response_contract() {
    let response: GetChatLinkResponse = parse_contract(json!({
        "share_link": "https://open.feishu.cn/chat/invite/abc",
        "expire_time": "2026-12-31T23:59:59Z",
        "is_permanent": false
    }));
    assert_eq!(
        response.share_link,
        "https://open.feishu.cn/chat/invite/abc"
    );
    assert_eq!(
        response.expire_time.as_deref(),
        Some("2026-12-31T23:59:59Z")
    );
    assert_eq!(response.is_permanent, Some(false));
    assert_json_contract(
        &response,
        json!({
            "share_link": "https://open.feishu.cn/chat/invite/abc",
            "expire_time": "2026-12-31T23:59:59Z",
            "is_permanent": false
        }),
    );
}
