//! Docs 任务型工作流示例
//!
//! 该示例用 3 组任务流展示 Docs helper 的组合方式：
//! 1. Drive 文件流转：列目录 -> 上传文件 -> 下载验证
//! 2. Spreadsheet 周报处理：定位工作表 -> 读取范围 -> 可选写入/追加
//! 3. 知识库巡检：按 Wiki 路径导航 -> 按条件筛选多维表格记录
//!
//! 运行方式：
//! ```bash
//! cargo run --example docs_workflows --features "auth,docs-bitable"
//! ```

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::from_env()?;

    println!("Docs 任务型工作流示例");
    println!("当前 app_id: {}", client.config().app_id());

    workflow_drive_file_flow(&client).await?;
    workflow_spreadsheet_reporting_flow(&client).await?;
    workflow_knowledge_review_flow(&client).await?;

    Ok(())
}

async fn workflow_drive_file_flow(
    client: &Client,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n== 工作流 1：Drive 文件流转 ==");

    let Ok(folder_token) = std::env::var("OPENLARK_FOLDER_TOKEN") else {
        println!("未设置 OPENLARK_FOLDER_TOKEN，跳过 Drive 文件流转示例");
        return Ok(());
    };

    let items = client
        .docs
        .list_folder_children_all(&folder_token, None)
        .await?;
    println!("目录子项数量: {}", items.len());

    let mut download_token = std::env::var("OPENLARK_DOWNLOAD_FILE_TOKEN").ok();

    if let Ok(upload_path) = std::env::var("OPENLARK_UPLOAD_FILE_PATH") {
        let file_name = std::path::Path::new(&upload_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("upload.bin")
            .to_string();
        let content = std::fs::read(&upload_path)?;
        let uploaded = client
            .docs
            .upload_drive_file(
                &folder_token,
                open_lark::docs::DriveUploadFile::new(file_name, content),
            )
            .await?;
        println!("上传完成，file_token = {}", uploaded.file_token);
        download_token = Some(uploaded.file_token);
    }

    if let Some(file_token) = download_token {
        let partial = client
            .docs
            .download_drive_file_range(
                &file_token,
                open_lark::docs::DriveDownloadRange::from_start(0).with_end(1023),
            )
            .await?;
        println!("下载校验完成，首段字节数 = {}", partial.len());
    } else {
        println!("未设置 OPENLARK_UPLOAD_FILE_PATH / OPENLARK_DOWNLOAD_FILE_TOKEN，跳过下载校验");
    }

    Ok(())
}

async fn workflow_spreadsheet_reporting_flow(
    client: &Client,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n== 工作流 2：Spreadsheet 周报处理 ==");

    let (Ok(spreadsheet_token), Ok(sheet_title)) = (
        std::env::var("OPENLARK_SPREADSHEET_TOKEN"),
        std::env::var("OPENLARK_SHEET_TITLE"),
    ) else {
        println!("未设置 OPENLARK_SPREADSHEET_TOKEN / OPENLARK_SHEET_TITLE，跳过 Spreadsheet 示例");
        return Ok(());
    };

    let summary_range = client
        .docs
        .resolve_sheet_range_by_title(&spreadsheet_token, &sheet_title, "A1:C10")
        .await?;
    let summary = client
        .docs
        .read_sheet_ranges(&spreadsheet_token, vec![summary_range.clone()])
        .await?;
    println!(
        "读取范围 {}，返回 {} 个 value_range",
        summary_range,
        summary.value_ranges.len()
    );

    if let Ok(write_expr) = std::env::var("OPENLARK_SHEETS_WRITE_RANGE") {
        let write_range = client
            .docs
            .resolve_sheet_range_by_title(&spreadsheet_token, &sheet_title, &write_expr)
            .await?;
        let result = client
            .docs
            .write_sheet_ranges(
                &spreadsheet_token,
                vec![open_lark::docs::SheetWriteRange::new(
                    write_range.clone(),
                    vec![vec![serde_json::json!("updated"), serde_json::json!(2026)]],
                )],
            )
            .await?;
        println!(
            "批量写入 {} 完成，更新单元格数 = {}",
            write_range, result.total_updated_cells
        );
    }

    if let Ok(append_expr) = std::env::var("OPENLARK_SHEETS_APPEND_RANGE") {
        let append_range = client
            .docs
            .resolve_sheet_range_by_title(&spreadsheet_token, &sheet_title, &append_expr)
            .await?;
        let appended = client
            .docs
            .append_sheet_range(
                &spreadsheet_token,
                append_range.clone(),
                vec![vec![
                    serde_json::json!("follow-up"),
                    serde_json::json!(true),
                ]],
            )
            .await?;
        println!(
            "追加 {} 完成，更新单元格数 = {}",
            append_range, appended.updated_cells
        );
    }

    Ok(())
}

async fn workflow_knowledge_review_flow(
    client: &Client,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("\n== 工作流 3：知识库巡检 ==");

    if let (Ok(space_id), Ok(node_path)) = (
        std::env::var("OPENLARK_WIKI_SPACE_ID"),
        std::env::var("OPENLARK_WIKI_NODE_PATH"),
    ) {
        let node = client
            .docs
            .find_wiki_node_by_path(&space_id, &node_path)
            .await?;
        println!(
            "Wiki 路径命中节点: {} ({})",
            node.title.unwrap_or_else(|| "<无标题>".to_string()),
            node.node_token
        );
    } else {
        println!("未设置 OPENLARK_WIKI_SPACE_ID / OPENLARK_WIKI_NODE_PATH，跳过 Wiki 路径导航");
    }

    if let (Ok(app_token), Ok(table_id), Ok(filter_field), Ok(filter_value)) = (
        std::env::var("OPENLARK_BITABLE_APP_TOKEN"),
        std::env::var("OPENLARK_BITABLE_TABLE_ID"),
        std::env::var("OPENLARK_BITABLE_FILTER_FIELD"),
        std::env::var("OPENLARK_BITABLE_FILTER_VALUE"),
    ) {
        let filtered = client
            .docs
            .query_bitable_records(
                open_lark::docs::BitableRecordQuery::new(app_token, table_id)
                    .where_equals(filter_field, filter_value),
            )
            .await?;
        println!("筛选到 {} 条待巡检记录", filtered.len());
    } else {
        println!(
            "未设置 OPENLARK_BITABLE_FILTER_FIELD / OPENLARK_BITABLE_FILTER_VALUE，跳过记录筛选"
        );
    }

    Ok(())
}
