//! 示例：使用工作流 API。
//!
//! 此示例展示如何通过 openlark 根 crate 使用任务型 workflow / approval helper。

use open_lark::CoreConfig;
use open_lark::workflow::{
    ApprovalTaskAction, ApprovalTaskQuery, WorkflowService, WorkflowTaskListQuery,
    WorkflowTaskMutation,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = CoreConfig::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    let workflow_service = WorkflowService::new(config);

    let tasks = workflow_service
        .list_tasks_all(
            WorkflowTaskListQuery::for_tasklist("tasklist_guid")
                .filter("status = incomplete")
                .page_size(50),
        )
        .await?;

    println!("任务列表数量: {}", tasks.len());

    let response = workflow_service
        .mutate_task(
            "task_guid",
            WorkflowTaskMutation::new()
                .summary("完成项目文档")
                .description("编写并完成 OpenLark SDK 的工作流模块文档")
                .priority(3),
        )
        .await?;

    println!("任务更新成功: {}", response.task_guid);

    let approval_tasks = workflow_service
        .query_approval_tasks(
            ApprovalTaskQuery::new("ou_example_user", "1")
                .user_id_type("open_id")
                .status("Todo"),
        )
        .await?;
    println!("待审批任务数量: {}", approval_tasks.len());

    if let Some(task) = approval_tasks.first() {
        workflow_service
            .approve_task(
                ApprovalTaskAction::new(
                    task.approval_code.clone(),
                    task.instance_code.clone(),
                    "ou_example_user",
                    task.task_id.clone(),
                )
                .user_id_type("open_id")
                .comment("自动化示例通过"),
            )
            .await?;
        println!("审批处理成功");
    }

    Ok(())
}
