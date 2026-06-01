//! Communication / Workflow 任务型工作流示例
//!
//! 该示例用 2 组任务流展示 helper 的组合方式：
//! 1. Communication 发送流：查人/查群 -> 发送文本或富文本消息
//! 2. Workflow 执行流：列任务 -> 更新任务 -> 查询审批任务并执行审批动作
//!
//! 运行方式：
//! ```bash
//! cargo run --example communication_workflows --features "auth,communication,workflow"
//! ```

use open_lark::prelude::*;
use open_lark::workflow::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowTaskListQuery, WorkflowTaskMutation,
};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::from_env()?;

    println!("Communication / Workflow 任务流示例");
    println!("当前 app_id: {}", client.config().app_id());

    communication_dispatch_flow(&client).await?;
    workflow_execution_flow(&client).await?;

    Ok(())
}

async fn communication_dispatch_flow(
    client: &Client,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n== 工作流 1：Communication 发送流 ==");

    if let (Ok(user_name), Ok(text)) = (
        std::env::var("OPENLARK_USER_SEARCH_NAME"),
        std::env::var("OPENLARK_TEXT_MESSAGE"),
    ) && !user_name.trim().is_empty()
        && !text.trim().is_empty()
    {
        let user = client
            .communication
            .contact
            .find_user_by_name(&user_name)
            .await?;
        let result = client
            .communication
            .im
            .send_text(
                open_lark::communication::MessageRecipient::open_id(user.open_id.clone()),
                text,
            )
            .await?;
        println!("已向用户 {} 发送文本消息: {}", user.name, result);
    }

    if let (Ok(chat_name), Ok(post_title), Ok(post_text)) = (
        std::env::var("OPENLARK_CHAT_SEARCH_NAME"),
        std::env::var("OPENLARK_POST_TITLE"),
        std::env::var("OPENLARK_POST_TEXT"),
    ) && !chat_name.trim().is_empty()
        && !post_title.trim().is_empty()
        && !post_text.trim().is_empty()
    {
        let chat = client
            .communication
            .im
            .find_chat_by_name(&chat_name)
            .await?;
        let result = client
            .communication
            .im
            .send_post(
                open_lark::communication::MessageRecipient::chat_id(chat.chat_id.clone()),
                open_lark::communication::PostMessage::zh_cn(post_title, post_text),
            )
            .await?;
        println!("已向群聊 {} 发送富文本消息: {}", chat.name, result);
    }

    Ok(())
}

async fn workflow_execution_flow(
    client: &Client,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n== 工作流 2：Workflow 执行流 ==");

    if let Ok(tasklist_guid) = std::env::var("OPENLARK_WORKFLOW_TASKLIST_GUID")
        && !tasklist_guid.trim().is_empty()
    {
        let tasks = client
            .workflow
            .list_tasks_all(
                WorkflowTaskListQuery::for_tasklist(&tasklist_guid)
                    .filter("status = incomplete")
                    .page_size(50),
            )
            .await?;
        println!("任务清单 {} 当前任务数: {}", tasklist_guid, tasks.len());
    }

    if let Ok(task_guid) = std::env::var("OPENLARK_WORKFLOW_TASK_GUID")
        && !task_guid.trim().is_empty()
    {
        let updated = client
            .workflow
            .mutate_task(
                &task_guid,
                WorkflowTaskMutation::new()
                    .summary("通过 helper 更新任务")
                    .priority(3),
            )
            .await?;
        println!("任务更新完成: {}", updated.task_guid);
    }

    if let (Ok(user_id), Ok(topic)) = (
        std::env::var("OPENLARK_APPROVAL_USER_ID"),
        std::env::var("OPENLARK_APPROVAL_TOPIC"),
    ) {
        if !user_id.trim().is_empty() && !topic.trim().is_empty() {
            let tasks = client
                .workflow
                .query_approval_tasks(
                    ApprovalTaskQuery::new(&user_id, &topic)
                        .user_id_type("open_id")
                        .status("Todo")
                        .page_size(50),
                )
                .await?;
            println!("审批任务数: {}", tasks.len());

            if let Some(task) = tasks.first() {
                let approved = client
                    .workflow
                    .approve_task(
                        ApprovalTaskAction::new(
                            task.approval_code.clone(),
                            task.instance_code.clone(),
                            user_id.clone(),
                            task.task_id.clone(),
                        )
                        .user_id_type("open_id")
                        .comment("workflow 示例审批通过"),
                    )
                    .await?;
                println!("审批处理结果: {}", approved.success);
            } else {
                println!("未查询到待处理审批任务，跳过审批动作");
            }
        } else {
            println!("OPENLARK_APPROVAL_USER_ID / OPENLARK_APPROVAL_TOPIC 为空，跳过审批流程");
        }
    } else {
        println!("未设置 OPENLARK_APPROVAL_USER_ID / OPENLARK_APPROVAL_TOPIC，跳过审批流程");
    }

    Ok(())
}
