//! Snapshot tests for high-value workflow helper outputs.
#![cfg(feature = "v2")]

use insta::assert_json_snapshot;
use openlark_workflow::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowTaskListQuery, WorkflowTaskMutation,
};
use serde_json::json;

#[test]
fn workflow_helper_outputs_snapshot() {
    let list_query = WorkflowTaskListQuery::for_tasklist("tasklist_123")
        .section_guid("section_456")
        .filter(r#"{"status":"Todo"}"#)
        .sort(json!([{ "field": "updated_at", "order": "desc" }]))
        .user_type("open_id")
        .page_size(50);
    let mutation = WorkflowTaskMutation::new()
        .summary("完成项目文档")
        .description("补齐 helper 快照测试")
        .due("2026-12-31T23:59:59Z")
        .priority(3)
        .assignee("ou_assignee")
        .status("InProgress");
    let approval_query = ApprovalTaskQuery::new("ou_xxx", "1")
        .user_id_type("open_id")
        .status("Todo")
        .instance_code("instance_123")
        .page_size(100);
    let approval_action =
        ApprovalTaskAction::new("approval_code", "instance_code", "ou_xxx", "task_123")
            .user_id_type("open_id")
            .comment("同意")
            .form(r#"[{"field":"result","value":"approved"}]"#);

    assert_json_snapshot!(
        "workflow_helper_outputs",
        json!({
            "workflow_task_list_query": {
                "tasklist_guid": list_query.tasklist_guid,
                "section_guid": list_query.section_guid,
                "filter": list_query.filter,
                "sort": list_query.sort,
                "user_type": list_query.user_type,
                "page_size": list_query.page_size,
            },
            "workflow_task_mutation": {
                "summary": mutation.summary,
                "description": mutation.description,
                "due": mutation.due,
                "priority": mutation.priority,
                "assignee": mutation.assignee,
                "status": mutation.status,
            },
            "approval_task_query": {
                "user_id": approval_query.user_id,
                "topic": approval_query.topic,
                "user_id_type": approval_query.user_id_type,
                "status": approval_query.status,
                "instance_code": approval_query.instance_code,
                "page_size": approval_query.page_size,
            },
            "approval_task_action": {
                "approval_code": approval_action.approval_code,
                "instance_code": approval_action.instance_code,
                "user_id": approval_action.user_id,
                "task_id": approval_action.task_id,
                "user_id_type": approval_action.user_id_type,
                "comment": approval_action.comment,
                "form": approval_action.form,
            }
        })
    );
}
