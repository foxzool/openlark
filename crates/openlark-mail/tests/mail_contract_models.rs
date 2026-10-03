//! 邮件模块代表性契约测试，覆盖核心 request/response 模型的序列化/反序列化双向一致性。
#![cfg(feature = "v1")]

use openlark_mail::mail::mail::v1::mailgroup::alias::list::{
    ListMailGroupAliasData, MailGroupAlias,
};
use openlark_mail::mail::mail::v1::mailgroup::models::{
    CreateMailGroupBody, DeleteMailGroupResponse, GetMailGroupResponse, MailGroupItem,
    MailGroupListResponse, UpdateMailGroupBody,
};
use openlark_mail::mail::mail::v1::public_mailbox::alias::models::{
    CreatePublicMailboxAliasBody, CreatePublicMailboxAliasResponse,
    DeletePublicMailboxAliasResponse, PublicMailboxAliasItem, PublicMailboxAliasListResponse,
};
use openlark_mail::mail::mail::v1::public_mailbox::member::models::{
    BatchCreatePublicMailboxMemberBody, BatchDeletePublicMailboxMemberBody,
    ClearPublicMailboxMemberResponse, CreatePublicMailboxMemberBody,
    CreatePublicMailboxMemberResponse, DeletePublicMailboxMemberResponse,
    GetPublicMailboxMemberResponse, PublicMailboxMemberItem, PublicMailboxMemberListResponse,
};
use openlark_mail::mail::mail::v1::public_mailbox::models::{
    CreatePublicMailboxBody, CreatePublicMailboxResponse, DeletePublicMailboxResponse,
    GetPublicMailboxResponse, PatchPublicMailboxBody, PatchPublicMailboxResponse,
    PublicMailboxItem, PublicMailboxListResponse, UpdatePublicMailboxBody,
    UpdatePublicMailboxResponse,
};
use openlark_mail::mail::mail::v1::user_mailbox::message::get::MessageData;
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

// ── 邮件组（MailGroup）──

#[test]
fn mailgroup_create_body_contract() {
    let body = CreateMailGroupBody {
        mail_group_id: "eng@example.com".to_string(),
        description: Some("工程团队".to_string()),
        owner: Some("ou_owner".to_string()),
        members: Some(vec![
            "a@example.com".to_string(),
            "b@example.com".to_string(),
        ]),
        only_admins_send: Some(true),
    };
    assert_json_contract(
        &body,
        json!({
            "mail_group_id": "eng@example.com",
            "description": "工程团队",
            "owner": "ou_owner",
            "members": ["a@example.com", "b@example.com"],
            "only_admins_send": true
        }),
    );

    let minimal = CreateMailGroupBody {
        mail_group_id: "minimal@example.com".to_string(),
        description: None,
        owner: None,
        members: None,
        only_admins_send: None,
    };
    assert_json_contract(
        &minimal,
        json!({
            "mail_group_id": "minimal@example.com"
        }),
    );
}

#[test]
fn mailgroup_update_body_contract() {
    let body = UpdateMailGroupBody {
        description: Some("更新描述".to_string()),
        only_admins_send: Some(false),
    };
    assert_json_contract(
        &body,
        json!({
            "description": "更新描述",
            "only_admins_send": false
        }),
    );
}

#[test]
fn mailgroup_get_response_contract() {
    let resp: GetMailGroupResponse = parse_contract(json!({
        "mail_group_id": "team@example.com",
        "description": "项目邮件组",
        "owner": "ou_admin",
        "only_admins_send": true,
        "created_at": "1700000000",
        "updated_at": "1700001000"
    }));
    assert_eq!(resp.mail_group_id, "team@example.com");
    assert_eq!(resp.description.as_deref(), Some("项目邮件组"));
    assert_eq!(resp.owner.as_deref(), Some("ou_admin"));
    assert_eq!(resp.only_admins_send, Some(true));
    assert_eq!(resp.created_at, "1700000000");
    assert_eq!(resp.updated_at.as_deref(), Some("1700001000"));
}

#[test]
fn mailgroup_delete_response_contract() {
    let resp: DeleteMailGroupResponse = parse_contract(json!({"success": true}));
    assert!(resp.success);
}

#[test]
fn mailgroup_list_response_contract() {
    let resp: MailGroupListResponse = parse_contract(json!({
        "mail_groups": [
            {
                "mail_group_id": "eng@example.com",
                "description": "工程组",
                "created_at": "1700000000"
            },
            {
                "mail_group_id": "hr@example.com",
                "created_at": "1700000001"
            }
        ],
        "page_token": "next_page",
        "has_more": true
    }));
    assert_eq!(resp.mail_groups.len(), 2);
    assert_eq!(resp.mail_groups[0].mail_group_id, "eng@example.com");
    assert_eq!(resp.mail_groups[0].description.as_deref(), Some("工程组"));
    assert_eq!(resp.mail_groups[1].description, None);
    assert_eq!(resp.page_token.as_deref(), Some("next_page"));
    assert_eq!(resp.has_more, Some(true));
}

#[test]
fn mailgroup_item_roundtrip() {
    let item: MailGroupItem = parse_contract(json!({
        "mail_group_id": "dev@example.com",
        "description": "开发者",
        "created_at": "1700000000"
    }));
    assert_eq!(item.mail_group_id, "dev@example.com");
    assert_eq!(item.description.as_deref(), Some("开发者"));
    assert_eq!(item.created_at, "1700000000");
}

// ── 邮件组别名（MailGroup Alias）──

#[test]
fn mailgroup_alias_contract() {
    let alias: MailGroupAlias = parse_contract(json!({
        "alias_id": "alias_123",
        "alias": "dev-alias@example.com"
    }));
    assert_eq!(alias.alias_id, "alias_123");
    assert_eq!(alias.alias, "dev-alias@example.com");

    let data: ListMailGroupAliasData = parse_contract(json!({
        "aliases": [
            {"alias_id": "a1", "alias": "a1@example.com"},
            {"alias_id": "a2", "alias": "a2@example.com"}
        ]
    }));
    assert_eq!(data.aliases.len(), 2);
    assert_eq!(data.aliases[0].alias_id, "a1");
}

// ── 公共邮箱（Public Mailbox）──

#[test]
fn public_mailbox_create_body_contract() {
    let body = CreatePublicMailboxBody {
        name: "support@example.com".to_string(),
        description: Some("客服邮箱".to_string()),
        owner_user_id: Some("ou_admin".to_string()),
    };
    assert_json_contract(
        &body,
        json!({
            "name": "support@example.com",
            "description": "客服邮箱",
            "owner_user_id": "ou_admin"
        }),
    );
}

#[test]
fn public_mailbox_create_response_contract() {
    let resp: CreatePublicMailboxResponse = parse_contract(json!({
        "public_mailbox_id": "pm_001",
        "name": "support@example.com",
        "description": "客服邮箱",
        "owner_user_id": "ou_admin",
        "is_deleted": false,
        "created_time": "1700000000",
        "update_time": "1700001000"
    }));
    assert_eq!(resp.public_mailbox_id, "pm_001");
    assert_eq!(resp.name, "support@example.com");
    assert_eq!(resp.description.as_deref(), Some("客服邮箱"));
    assert!(!resp.is_deleted);
}

#[test]
fn public_mailbox_get_response_contract() {
    let resp: GetPublicMailboxResponse = parse_contract(json!({
        "public_mailbox_id": "pm_002",
        "name": "info@example.com",
        "is_deleted": false,
        "created_time": "1700000000"
    }));
    assert_eq!(resp.public_mailbox_id, "pm_002");
    assert_eq!(resp.name, "info@example.com");
    assert_eq!(resp.description, None);
    assert_eq!(resp.owner_user_id, None);
    assert!(!resp.is_deleted);
}

#[test]
fn public_mailbox_update_body_and_response_contract() {
    let body = UpdatePublicMailboxBody {
        name: Some("new-name@example.com".to_string()),
        description: Some("更新描述".to_string()),
        owner_user_id: None,
    };
    assert_json_contract(
        &body,
        json!({
            "name": "new-name@example.com",
            "description": "更新描述"
        }),
    );

    let resp: UpdatePublicMailboxResponse = parse_contract(json!({
        "public_mailbox_id": "pm_003",
        "update_time": "1700002000"
    }));
    assert_eq!(resp.public_mailbox_id, "pm_003");
    assert_eq!(resp.update_time, "1700002000");
}

#[test]
fn public_mailbox_patch_body_and_response_contract() {
    let body = PatchPublicMailboxBody {
        name: Some("patched@example.com".to_string()),
        description: None,
    };
    assert_json_contract(&body, json!({"name": "patched@example.com"}));

    let resp: PatchPublicMailboxResponse = parse_contract(json!({
        "public_mailbox_id": "pm_004",
        "update_time": "1700003000"
    }));
    assert_eq!(resp.public_mailbox_id, "pm_004");
}

#[test]
fn public_mailbox_delete_response_contract() {
    let resp: DeletePublicMailboxResponse = parse_contract(json!({"success": true}));
    assert!(resp.success);
}

#[test]
fn public_mailbox_list_response_contract() {
    let resp: PublicMailboxListResponse = parse_contract(json!({
        "items": [
            {
                "public_mailbox_id": "pm_1",
                "name": "support",
                "is_deleted": false,
                "created_time": "1700000000"
            },
            {
                "public_mailbox_id": "pm_2",
                "name": "info",
                "description": "信息邮箱",
                "owner_user_id": "ou_1",
                "is_deleted": true,
                "created_time": "1700000001"
            }
        ],
        "page_token": "next",
        "has_more": false
    }));
    assert_eq!(resp.items.len(), 2);
    assert_eq!(resp.items[0].public_mailbox_id, "pm_1");
    assert_eq!(resp.items[0].description, None);
    assert!(!resp.items[0].is_deleted);
    assert_eq!(resp.items[1].description.as_deref(), Some("信息邮箱"));
    assert!(resp.items[1].is_deleted);
    assert_eq!(resp.page_token.as_deref(), Some("next"));
    assert_eq!(resp.has_more, Some(false));
}

#[test]
fn public_mailbox_item_roundtrip() {
    let item: PublicMailboxItem = parse_contract(json!({
        "public_mailbox_id": "pm_round",
        "name": "roundtrip",
        "is_deleted": false,
        "created_time": "1700000000"
    }));
    assert_eq!(item.public_mailbox_id, "pm_round");
}

// ── 公共邮箱别名（Public Mailbox Alias）──

#[test]
fn public_mailbox_alias_create_body_contract() {
    let body = CreatePublicMailboxAliasBody {
        email: "alias@example.com".to_string(),
    };
    assert_json_contract(&body, json!({"email": "alias@example.com"}));
}

#[test]
fn public_mailbox_alias_response_contracts() {
    let create_resp: CreatePublicMailboxAliasResponse = parse_contract(json!({
        "alias_id": "pa_001",
        "email": "alias@example.com",
        "create_time": "1700000000"
    }));
    assert_eq!(create_resp.alias_id, "pa_001");
    assert_eq!(create_resp.email, "alias@example.com");

    let delete_resp: DeletePublicMailboxAliasResponse = parse_contract(json!({"success": true}));
    assert!(delete_resp.success);
}

#[test]
fn public_mailbox_alias_list_response_contract() {
    let resp: PublicMailboxAliasListResponse = parse_contract(json!({
        "items": [
            {"alias_id": "a1", "email": "a1@example.com", "create_time": "1700000000"},
            {"alias_id": "a2", "email": "a2@example.com", "create_time": "1700000001"}
        ],
        "page_token": "next",
        "has_more": true
    }));
    assert_eq!(resp.items.len(), 2);
    assert_eq!(resp.items[0].alias_id, "a1");
    assert_eq!(resp.items[1].email, "a2@example.com");
    assert_eq!(resp.has_more, Some(true));

    let item: PublicMailboxAliasItem = parse_contract(json!({
        "alias_id": "a_round",
        "email": "round@example.com",
        "create_time": "1700000000"
    }));
    assert_eq!(item.alias_id, "a_round");
}

// ── 公共邮箱成员（Public Mailbox Member）──

#[test]
fn public_mailbox_member_create_body_contract() {
    let body = CreatePublicMailboxMemberBody {
        email: "member@example.com".to_string(),
        member_type: Some(1),
    };
    assert_json_contract(
        &body,
        json!({"email": "member@example.com", "member_type": 1}),
    );

    let minimal = CreatePublicMailboxMemberBody {
        email: "bare@example.com".to_string(),
        member_type: None,
    };
    assert_json_contract(&minimal, json!({"email": "bare@example.com"}));
}

#[test]
fn public_mailbox_member_response_contracts() {
    let create_resp: CreatePublicMailboxMemberResponse = parse_contract(json!({
        "member_id": "mem_001",
        "email": "member@example.com",
        "member_type": 1,
        "create_time": "1700000000"
    }));
    assert_eq!(create_resp.member_id, "mem_001");
    assert_eq!(create_resp.member_type, 1);

    let get_resp: GetPublicMailboxMemberResponse = parse_contract(json!({
        "member_id": "mem_002",
        "email": "getter@example.com",
        "member_type": 2,
        "create_time": "1700000001"
    }));
    assert_eq!(get_resp.member_id, "mem_002");
    assert_eq!(get_resp.member_type, 2);

    let delete_resp: DeletePublicMailboxMemberResponse = parse_contract(json!({"success": true}));
    assert!(delete_resp.success);

    let clear_resp: ClearPublicMailboxMemberResponse = parse_contract(json!({"success": false}));
    assert!(!clear_resp.success);
}

#[test]
fn public_mailbox_member_list_response_contract() {
    let resp: PublicMailboxMemberListResponse = parse_contract(json!({
        "items": [
            {"member_id": "m1", "email": "m1@example.com", "member_type": 1, "create_time": "1700000000"},
            {"member_id": "m2", "email": "m2@example.com", "member_type": 2, "create_time": "1700000001"}
        ],
        "has_more": false
    }));
    assert_eq!(resp.items.len(), 2);
    assert_eq!(resp.items[0].member_id, "m1");
    assert_eq!(resp.items[1].member_type, 2);
    assert_eq!(resp.page_token, None);
    assert_eq!(resp.has_more, Some(false));
}

#[test]
fn public_mailbox_member_batch_bodies_contract() {
    let batch_create = BatchCreatePublicMailboxMemberBody {
        emails: vec!["a@x.com".to_string(), "b@x.com".to_string()],
    };
    assert_json_contract(&batch_create, json!({"emails": ["a@x.com", "b@x.com"]}));

    let batch_delete = BatchDeletePublicMailboxMemberBody {
        member_ids: vec!["id1".to_string(), "id2".to_string()],
    };
    assert_json_contract(&batch_delete, json!({"member_ids": ["id1", "id2"]}));
}

#[test]
fn public_mailbox_member_item_roundtrip() {
    let item: PublicMailboxMemberItem = parse_contract(json!({
        "member_id": "m_round",
        "email": "round@example.com",
        "member_type": 3,
        "create_time": "1700000000"
    }));
    assert_eq!(item.member_id, "m_round");
    assert_eq!(item.email, "round@example.com");
    assert_eq!(item.member_type, 3);
}

// ── 用户邮箱消息（User Mailbox Message）──

#[test]
fn message_data_contract() {
    let msg: MessageData = parse_contract(json!({
        "message_id": "msg_001",
        "subject": "测试邮件",
        "body": "<p>邮件内容</p>"
    }));
    assert_eq!(msg.message_id, "msg_001");
    assert_eq!(msg.subject, "测试邮件");
    assert_eq!(msg.body, "<p>邮件内容</p>");

    assert_json_contract(
        &msg,
        json!({
            "message_id": "msg_001",
            "subject": "测试邮件",
            "body": "<p>邮件内容</p>"
        }),
    );
}

// ── skip_serializing_if 验证 ──

#[test]
fn optional_fields_skip_serialization() {
    let body = CreateMailGroupBody {
        mail_group_id: "skip@example.com".to_string(),
        description: None,
        owner: None,
        members: None,
        only_admins_send: None,
    };
    let val = to_value(&body).unwrap();
    assert!(!val.as_object().unwrap().contains_key("description"));
    assert!(!val.as_object().unwrap().contains_key("owner"));
    assert!(!val.as_object().unwrap().contains_key("members"));
    assert!(!val.as_object().unwrap().contains_key("only_admins_send"));
    assert!(val.as_object().unwrap().contains_key("mail_group_id"));
}

#[test]
fn public_mailbox_update_optional_fields_skip() {
    let body = UpdatePublicMailboxBody {
        name: None,
        description: None,
        owner_user_id: None,
    };
    let val = to_value(&body).unwrap();
    assert!(val.as_object().unwrap().is_empty());
}
