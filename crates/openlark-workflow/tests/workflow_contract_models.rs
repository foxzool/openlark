//! Representative contract tests for high-frequency workflow request/response models.
#![cfg(feature = "v2")]

use openlark_workflow::v2::comment::{CreateCommentBody, ListCommentsResponse};
use openlark_workflow::v2::custom_field::{
    CreateCustomFieldBody, CustomFieldConfig, CustomFieldType, ListCustomFieldsResponse,
};
use openlark_workflow::v2::task::{CreateTaskBody, ListTasksResponse};
use openlark_workflow::v2::tasklist::{CreateTasklistBody, ListTasklistsResponse, TasklistIcon};
use openlark_workflow::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowTaskListQuery, WorkflowTaskMutation,
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

#[test]
fn workflow_helper_models_preserve_builder_contracts() {
    let query = WorkflowTaskListQuery::for_tasklist("tasklist_guid")
        .section_guid("section_guid")
        .filter(r#"{"status":"Todo"}"#)
        .sort(json!([{ "field": "updated_at", "order": "desc" }]))
        .user_type("open_id")
        .page_size(50);
    assert_eq!(
        query,
        WorkflowTaskListQuery {
            tasklist_guid: Some("tasklist_guid".to_string()),
            section_guid: Some("section_guid".to_string()),
            filter: Some(r#"{"status":"Todo"}"#.to_string()),
            sort: Some(json!([{ "field": "updated_at", "order": "desc" }])),
            user_type: Some("open_id".to_string()),
            page_size: Some(50),
        }
    );

    let mutation = WorkflowTaskMutation::new()
        .summary("完成项目文档")
        .description("补齐 contract tests")
        .due("2026-12-31T23:59:59Z")
        .priority(3)
        .assignee("ou_xxx")
        .status("InProgress");
    assert_eq!(mutation.summary.as_deref(), Some("完成项目文档"));
    assert_eq!(mutation.status.as_deref(), Some("InProgress"));

    let approval_query = ApprovalTaskQuery::new("ou_xxx", "1")
        .user_id_type("open_id")
        .status("Todo")
        .instance_code("instance_code")
        .page_size(100);
    assert_eq!(approval_query.user_id, "ou_xxx");
    assert_eq!(approval_query.topic, "1");
    assert_eq!(approval_query.user_id_type.as_deref(), Some("open_id"));
    assert_eq!(approval_query.status.as_deref(), Some("Todo"));

    let action = ApprovalTaskAction::new("approval_code", "instance_code", "ou_xxx", "task_123")
        .user_id_type("open_id")
        .comment("同意")
        .form(r#"[{"field":"status","value":"approved"}]"#);
    assert_eq!(action.approval_code, "approval_code");
    assert_eq!(action.task_id, "task_123");
    assert_eq!(action.comment.as_deref(), Some("同意"));
}

#[test]
fn task_request_and_list_response_contract() {
    let request = CreateTaskBody {
        summary: "完成发布清单".to_string(),
        description: Some("对齐 README / examples / contract tests".to_string()),
        start: Some("2026-12-01T09:00:00Z".to_string()),
        due: Some("2026-12-07T18:00:00Z".to_string()),
        tasklist_guid: Some("tasklist_guid".to_string()),
        section_guid: Some("section_guid".to_string()),
        priority: Some(3),
        custom_fields: Some(json!({"风险等级":"高"})),
        followers: Some(vec!["ou_follower".to_string()]),
        subtasks: Some(vec![
            json!({"summary":"补充测试"}),
            json!({"summary":"更新文档"}),
        ]),
        assignee: Some("ou_assignee".to_string()),
        remind_time: Some("2026-12-06T09:00:00Z".to_string()),
        repeat_rule: Some(json!({"type":"weekly"})),
    };
    assert_json_contract(
        &request,
        json!({
            "summary": "完成发布清单",
            "description": "对齐 README / examples / contract tests",
            "start": "2026-12-01T09:00:00Z",
            "due": "2026-12-07T18:00:00Z",
            "tasklist_guid": "tasklist_guid",
            "section_guid": "section_guid",
            "priority": 3,
            "custom_fields": {"风险等级":"高"},
            "followers": ["ou_follower"],
            "subtasks": [{"summary":"补充测试"}, {"summary":"更新文档"}],
            "assignee": "ou_assignee",
            "remind_time": "2026-12-06T09:00:00Z",
            "repeat_rule": {"type":"weekly"}
        }),
    );

    let response: ListTasksResponse = parse_contract(json!({
        "has_more": true,
        "page_token": "next_page",
        "total": 2,
        "items": [
            {
                "task_guid": "task_1",
                "summary": "完成发布清单",
                "description": "对齐 README / examples / contract tests",
                "status": "InProgress",
                "tasklist_guid": "tasklist_guid",
                "section_guid": "section_guid",
                "priority": 3,
                "start": "2026-12-01T09:00:00Z",
                "due": "2026-12-07T18:00:00Z",
                "created_at": "2026-12-01T09:00:00Z",
                "updated_at": "2026-12-01T10:00:00Z",
                "completed_at": null,
                "assignee": "ou_assignee",
                "creator": "ou_creator"
            }
        ]
    }));
    assert_eq!(response.items[0].task_guid, "task_1");
    assert_eq!(response.items[0].assignee.as_deref(), Some("ou_assignee"));
    assert_json_contract(
        &response,
        json!({
            "has_more": true,
            "page_token": "next_page",
            "total": 2,
            "items": [
                {
                    "task_guid": "task_1",
                    "summary": "完成发布清单",
                    "description": "对齐 README / examples / contract tests",
                    "status": "InProgress",
                    "tasklist_guid": "tasklist_guid",
                    "section_guid": "section_guid",
                    "priority": 3,
                    "start": "2026-12-01T09:00:00Z",
                    "due": "2026-12-07T18:00:00Z",
                    "created_at": "2026-12-01T09:00:00Z",
                    "updated_at": "2026-12-01T10:00:00Z",
                    "completed_at": null,
                    "assignee": "ou_assignee",
                    "creator": "ou_creator"
                }
            ]
        }),
    );
}

#[test]
fn tasklist_request_and_response_contract() {
    let request = CreateTasklistBody {
        summary: "Q4 发布".to_string(),
        description: Some("聚合本季度关键交付".to_string()),
        icon: Some(TasklistIcon::Emoji {
            emoji: "🚀".to_string(),
        }),
    };
    assert_json_contract(
        &request,
        json!({
            "summary": "Q4 发布",
            "description": "聚合本季度关键交付",
            "icon": {
                "type": "emoji",
                "emoji": "🚀"
            }
        }),
    );

    let response: ListTasklistsResponse = parse_contract(json!({
        "has_more": false,
        "page_token": null,
        "total": 1,
        "items": [
            {
                "tasklist_guid": "tasklist_1",
                "summary": "Q4 发布",
                "description": "聚合本季度关键交付",
                "icon": {
                    "type": "emoji",
                    "emoji": "🚀"
                },
                "created_at": "2026-12-01T09:00:00Z",
                "updated_at": "2026-12-01T10:00:00Z"
            }
        ]
    }));
    assert_eq!(response.items[0].tasklist_guid, "tasklist_1");
    assert_json_contract(
        &response,
        json!({
            "has_more": false,
            "total": 1,
            "items": [
                {
                    "tasklist_guid": "tasklist_1",
                    "summary": "Q4 发布",
                    "description": "聚合本季度关键交付",
                    "icon": {
                        "type": "emoji",
                        "emoji": "🚀"
                    },
                    "created_at": "2026-12-01T09:00:00Z",
                    "updated_at": "2026-12-01T10:00:00Z"
                }
            ]
        }),
    );
}

#[test]
fn comment_and_custom_field_contract() {
    let comment_request = CreateCommentBody {
        content: "请补充发布回滚说明".to_string(),
    };
    assert_json_contract(
        &comment_request,
        json!({
            "content": "请补充发布回滚说明"
        }),
    );

    let comment_response: ListCommentsResponse = parse_contract(json!({
        "has_more": false,
        "page_token": null,
        "total": 1,
        "items": [
            {
                "comment_guid": "comment_1",
                "task_guid": "task_1",
                "content": "请补充发布回滚说明",
                "creator": "ou_creator",
                "created_at": "2026-12-01T09:00:00Z",
                "updated_at": "2026-12-01T10:00:00Z"
            }
        ]
    }));
    assert_eq!(comment_response.items[0].comment_guid, "comment_1");

    let custom_field_request = CreateCustomFieldBody {
        name: "风险等级".to_string(),
        config: CustomFieldConfig {
            field_type: CustomFieldType::Select,
            options: Some(vec!["高".to_string(), "中".to_string(), "低".to_string()]),
        },
        resource_type: "tasklist".to_string(),
        resource_id: "tasklist_123".to_string(),
    };
    assert_json_contract(
        &custom_field_request,
        json!({
            "name": "风险等级",
            "config": {
                "type": "select",
                "options": ["高", "中", "低"]
            },
            "resource_type": "tasklist",
            "resource_id": "tasklist_123"
        }),
    );

    let custom_field_response: ListCustomFieldsResponse = parse_contract(json!({
        "has_more": false,
        "page_token": null,
        "total": 1,
        "items": [
            {
                "field_guid": "field_1",
                "name": "风险等级",
                "config": {
                    "type": "select",
                    "options": ["高", "中", "低"]
                },
                "created_at": "2026-12-01T09:00:00Z",
                "updated_at": "2026-12-01T10:00:00Z"
            }
        ]
    }));
    assert_eq!(custom_field_response.items[0].field_guid, "field_1");
}
