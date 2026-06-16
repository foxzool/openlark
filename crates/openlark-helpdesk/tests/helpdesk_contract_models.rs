//! 帮助台模块核心数据模型契约测试。
//!
//! 验证 Ticket、Category、FAQ、Notification、Agent 等主要结构的
//! 序列化/反序列化双向一致性。

use openlark_helpdesk::helpdesk::helpdesk::v1::agent::patch::{PatchAgentBody, PatchAgentResult};
use openlark_helpdesk::helpdesk::helpdesk::v1::category::create::{
    CreateCategoryBody, CreateCategoryResult,
};
use openlark_helpdesk::helpdesk::helpdesk::v1::faq::create::{CreateFaqBody, CreateFaqResult};
use openlark_helpdesk::helpdesk::helpdesk::v1::notification::create::{
    CreateNotificationBody, CreateNotificationResult,
};
use openlark_helpdesk::helpdesk::helpdesk::v1::ticket::message::create::CreateTicketMessageBody;
use openlark_helpdesk::helpdesk::helpdesk::v1::ticket::models::{
    CreateTicketBody, CreateTicketResponse, GetTicketResponse, TicketItem, TicketListResponse,
    UpdateTicketBody, UpdateTicketResponse,
};
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

// ── Ticket 模型 ──────────────────────────────────────────────

#[test]
fn ticket_create_body_serializes_all_fields() {
    let body = CreateTicketBody {
        title: "无法登录系统".to_string(),
        description: Some("账号被锁定，无法登录".to_string()),
        priority: Some("high".to_string()),
    };
    assert_json_contract(
        &body,
        json!({
            "title": "无法登录系统",
            "description": "账号被锁定，无法登录",
            "priority": "high"
        }),
    );
}

#[test]
fn ticket_create_body_skips_none_fields() {
    let body = CreateTicketBody {
        title: "密码重置".to_string(),
        description: None,
        priority: None,
    };
    let value = to_value(&body).unwrap();
    assert!(value.get("description").is_none());
    assert!(value.get("priority").is_none());
}

#[test]
fn ticket_create_response_roundtrip() {
    let resp: CreateTicketResponse = parse_contract(json!({
        "ticket_id": "t_1001",
        "title": "无法登录系统",
        "created_at": "2026-04-20T10:00:00Z"
    }));
    assert_eq!(resp.ticket_id, "t_1001");
    assert_eq!(resp.title, "无法登录系统");
    assert_eq!(resp.created_at, "2026-04-20T10:00:00Z");
}

#[test]
fn ticket_get_response_roundtrip() {
    let resp: GetTicketResponse = parse_contract(json!({
        "ticket_id": "t_2002",
        "title": "审批流异常",
        "description": "审批流程无法发起",
        "status": "processing",
        "created_at": "2026-04-19T08:30:00Z"
    }));
    assert_eq!(resp.ticket_id, "t_2002");
    assert_eq!(resp.status, "processing");
    assert_eq!(resp.description, Some("审批流程无法发起".to_string()));
}

#[test]
fn ticket_update_body_roundtrip() {
    let body = UpdateTicketBody {
        title: Some("更新后的标题".to_string()),
        description: None,
        status: Some(2),
        tag_names: None,
        comment: None,
        customized_fields: None,
        ticket_type: None,
        solved: None,
        channel: None,
    };
    assert_json_contract(
        &body,
        json!({
            "title": "更新后的标题",
            "status": 2
        }),
    );

    let serialized = to_value(&body).unwrap();
    assert_eq!(serialized["title"], "更新后的标题");
    assert_eq!(serialized["status"], 2);
}

#[test]
fn ticket_update_response_contract() {
    let resp: UpdateTicketResponse = parse_contract(json!({
        "ticket_id": "t_3003",
        "updated_at": "2026-04-20T12:00:00Z"
    }));
    assert_eq!(resp.ticket_id, "t_3003");
    assert_eq!(resp.updated_at, "2026-04-20T12:00:00Z");
}

#[test]
fn ticket_list_response_with_pagination() {
    let resp: TicketListResponse = parse_contract(json!({
        "tickets": [
            { "ticket_id": "t_1", "title": "登录问题", "status": "open" },
            { "ticket_id": "t_2", "title": "权限不足", "status": "closed" }
        ],
        "page_token": "cursor_abc"
    }));
    assert_eq!(resp.tickets.len(), 2);
    assert_eq!(resp.tickets[0].ticket_id, "t_1");
    assert_eq!(resp.tickets[1].status, "closed");
    assert_eq!(resp.page_token, Some("cursor_abc".to_string()));
}

#[test]
fn ticket_item_roundtrip() {
    let item = parse_contract::<TicketItem>(json!({
        "ticket_id": "t_99",
        "title": "网络异常",
        "status": "pending"
    }));
    assert_eq!(item.ticket_id, "t_99");
    assert_eq!(item.title, "网络异常");
    assert_eq!(item.status, "pending");
}

// ── Category 模型 ────────────────────────────────────────────

#[test]
fn category_create_body_full_roundtrip() {
    let body = CreateCategoryBody {
        name: "技术支持".to_string(),
        description: Some("技术相关问题分类".to_string()),
        parent_id: Some("cat_root".to_string()),
        order: Some(10),
    };
    assert_json_contract(
        &body,
        json!({
            "name": "技术支持",
            "description": "技术相关问题分类",
            "parent_id": "cat_root",
            "order": 10
        }),
    );

    let roundtrip: CreateCategoryBody = from_value(to_value(&body).unwrap()).unwrap();
    assert_eq!(roundtrip.name, body.name);
    assert_eq!(roundtrip.parent_id, body.parent_id);
}

#[test]
fn category_create_body_minimal() {
    let body = CreateCategoryBody {
        name: "常见问题".to_string(),
        description: None,
        parent_id: None,
        order: None,
    };
    let value = to_value(&body).unwrap();
    assert_eq!(value["name"], "常见问题");
    assert!(value.get("description").is_none());
    assert!(value.get("order").is_none());
}

#[test]
fn category_create_result_roundtrip() {
    let result = parse_contract::<CreateCategoryResult>(json!({
        "id": "cat_001",
        "name": "技术支持",
        "description": "技术相关问题",
        "parent_id": "cat_root",
        "order": 5
    }));
    assert_eq!(result.id, Some("cat_001".to_string()));
    assert_eq!(result.name, Some("技术支持".to_string()));
    assert_eq!(result.order, Some(5));
}

// ── FAQ 模型 ─────────────────────────────────────────────────

#[test]
fn faq_create_body_roundtrip() {
    let body = CreateFaqBody {
        title: "如何重置密码？".to_string(),
        content: "请在登录页面点击「忘记密码」...".to_string(),
        category_id: Some("cat_001".to_string()),
    };
    assert_json_contract(
        &body,
        json!({
            "title": "如何重置密码？",
            "content": "请在登录页面点击「忘记密码」...",
            "category_id": "cat_001"
        }),
    );

    let roundtrip: CreateFaqBody = from_value(to_value(&body).unwrap()).unwrap();
    assert_eq!(roundtrip.title, body.title);
    assert_eq!(roundtrip.content, body.content);
}

#[test]
fn faq_create_body_without_optional() {
    let body = CreateFaqBody {
        title: "常见问题".to_string(),
        content: "这是答案内容".to_string(),
        category_id: None,
    };
    let value = to_value(&body).unwrap();
    assert!(value.get("category_id").is_none());
}

#[test]
fn faq_create_result_roundtrip() {
    let result = parse_contract::<CreateFaqResult>(json!({
        "id": "faq_100",
        "title": "如何重置密码？",
        "content": "请在登录页面点击「忘记密码」...",
        "category_id": "cat_001",
        "status": "published"
    }));
    assert_eq!(result.id, Some("faq_100".to_string()));
    assert_eq!(result.status, Some("published".to_string()));
}

// ── Notification 模型 ────────────────────────────────────────

#[test]
fn notification_create_body_roundtrip() {
    let body = CreateNotificationBody {
        title: "系统维护通知".to_string(),
        content: "系统将于今晚 22:00 进行维护".to_string(),
    };
    assert_json_contract(
        &body,
        json!({
            "title": "系统维护通知",
            "content": "系统将于今晚 22:00 进行维护"
        }),
    );
}

#[test]
fn notification_create_result_roundtrip() {
    let result = parse_contract::<CreateNotificationResult>(json!({
        "id": "notify_001",
        "title": "系统维护通知",
        "status": "draft"
    }));
    assert_eq!(result.id, Some("notify_001".to_string()));
    assert_eq!(result.title, Some("系统维护通知".to_string()));
    assert_eq!(result.status, Some("draft".to_string()));
}

// ── Agent 模型 ───────────────────────────────────────────────

#[test]
fn agent_patch_body_roundtrip() {
    let body = PatchAgentBody {
        status: Some("online".to_string()),
    };
    assert_json_contract(&body, json!({ "status": "online" }));

    let roundtrip: PatchAgentBody = from_value(to_value(&body).unwrap()).unwrap();
    assert_eq!(roundtrip.status, Some("online".to_string()));
}

#[test]
fn agent_patch_body_empty() {
    let body = PatchAgentBody::default();
    let value = to_value(&body).unwrap();
    assert!(value.get("status").is_none());
}

#[test]
fn agent_patch_result_roundtrip() {
    let result = parse_contract::<PatchAgentResult>(json!({
        "agent_id": "agent_001",
        "status": "online"
    }));
    assert_eq!(result.agent_id, Some("agent_001".to_string()));
    assert_eq!(result.status, Some("online".to_string()));
}

// ── Ticket Message 模型 ─────────────────────────────────────

#[test]
fn ticket_message_body_roundtrip() {
    let body = CreateTicketMessageBody {
        content: "您好，请提供更多信息".to_string(),
        msg_type: Some("text".to_string()),
    };
    assert_json_contract(
        &body,
        json!({
            "content": "您好，请提供更多信息",
            "msg_type": "text"
        }),
    );
}

#[test]
fn ticket_message_body_minimal() {
    let body = CreateTicketMessageBody {
        content: "简单的消息".to_string(),
        msg_type: None,
    };
    let value = to_value(&body).unwrap();
    assert_eq!(value["content"], "简单的消息");
    assert!(value.get("msg_type").is_none());
}
