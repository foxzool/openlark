//! 文档 helper 示例
//!
//! 演示 `openlark` 根 crate 在 `0.16.0` 中提供的通用 docs helper：
//! - 文件夹自动分页遍历
//! - 文件夹 typed pagination helper
//! - Drive 文件上传/下载 helper
//! - 按标题查找工作表
//! - 读取多个单元格范围
//! - 按条件筛选 bitable 记录
//! - 多维表格全量读取
//! - Wiki 路径导航 helper
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_app_id"
//! export OPENLARK_APP_SECRET="your_app_secret"
//! cargo run --example docs_helpers --features "auth,docs-bitable"
//! ```

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::from_env()?;

    println!("docs helper 示例");
    println!("当前 app_id: {}", client.config().app_id);

    if let Ok(folder_token) = std::env::var("OPENLARK_FOLDER_TOKEN") {
        let mut pager = client
            .docs
            .folder_children_pager(&folder_token)
            .page_size(50);
        let first_page = pager.fetch_next_page().await?;
        println!("第一页子项数量: {}", first_page.items.len());
        println!("第一页是否还有更多: {}", first_page.has_more);
        if let Some(next_page_token) = &first_page.next_page_token {
            println!("下一页 token: {next_page_token}");
        }

        let items = client
            .docs
            .list_folder_children_all(&folder_token, None)
            .await?;
        println!("文件夹子项数量: {}", items.len());

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
            println!("上传文件 token: {}", uploaded.file_token);
        }
    } else {
        println!("未设置 OPENLARK_FOLDER_TOKEN，跳过文件夹遍历示例");
    }

    if let Ok(file_token) = std::env::var("OPENLARK_DOWNLOAD_FILE_TOKEN") {
        let bytes = client.docs.download_drive_file(&file_token).await?;
        println!("下载文件字节数: {}", bytes.len());

        if std::env::var("OPENLARK_DOWNLOAD_RANGE_DEMO")
            .ok()
            .as_deref()
            == Some("1")
        {
            let partial = client
                .docs
                .download_drive_file_range(
                    &file_token,
                    open_lark::docs::DriveDownloadRange::from_start(0).with_end(1023),
                )
                .await?;
            println!("分片下载字节数: {}", partial.len());
        }
    } else {
        println!("未设置 OPENLARK_DOWNLOAD_FILE_TOKEN，跳过文件下载示例");
    }

    if let (Ok(spreadsheet_token), Ok(sheet_title)) = (
        std::env::var("OPENLARK_SPREADSHEET_TOKEN"),
        std::env::var("OPENLARK_SHEET_TITLE"),
    ) {
        let sheet = client
            .docs
            .find_sheet_by_title(&spreadsheet_token, &sheet_title)
            .await?;
        let range = open_lark::docs::SheetRange::from_range_expr(sheet.sheet_id.clone(), "A1:C5")?;
        let data = client
            .docs
            .read_sheet_ranges(&spreadsheet_token, vec![range.clone()])
            .await?;
        println!("工作表标题: {}", sheet.title);
        println!("读取范围数量: {}", data.value_ranges.len());

        if std::env::var("OPENLARK_SHEETS_WRITE_DEMO").ok().as_deref() == Some("1") {
            let write = open_lark::docs::SheetWriteRange::new(
                range.clone(),
                vec![vec![serde_json::json!("demo"), serde_json::json!(2026)]],
            );
            let result = client
                .docs
                .write_sheet_ranges(&spreadsheet_token, vec![write])
                .await?;
            println!("批量写入更新单元格数: {}", result.total_updated_cells);
        }
    } else {
        println!("未设置 OPENLARK_SPREADSHEET_TOKEN / OPENLARK_SHEET_TITLE，跳过表格示例");
    }

    if let (Ok(app_token), Ok(table_id)) = (
        std::env::var("OPENLARK_BITABLE_APP_TOKEN"),
        std::env::var("OPENLARK_BITABLE_TABLE_ID"),
    ) {
        let records = client
            .docs
            .search_bitable_records_all(&app_token, &table_id)
            .await?;
        println!("多维表格记录数: {}", records.len());

        if let (Ok(filter_field), Ok(filter_value)) = (
            std::env::var("OPENLARK_BITABLE_FILTER_FIELD"),
            std::env::var("OPENLARK_BITABLE_FILTER_VALUE"),
        ) {
            let filtered = client
                .docs
                .query_bitable_records(
                    open_lark::docs::BitableRecordQuery::new(app_token.clone(), table_id.clone())
                        .where_equals(filter_field, filter_value),
                )
                .await?;
            println!("筛选后记录数: {}", filtered.len());
        }
    } else {
        println!("未设置 OPENLARK_BITABLE_APP_TOKEN / OPENLARK_BITABLE_TABLE_ID，跳过多维表格示例");
    }

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
        println!("未设置 OPENLARK_WIKI_SPACE_ID / OPENLARK_WIKI_NODE_PATH，跳过 Wiki 导航示例");
    }

    Ok(())
}
