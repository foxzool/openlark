//! Snapshot tests for high-value Docs helper outputs.
#![cfg(all(feature = "ccm-core", feature = "bitable"))]

use insta::{assert_json_snapshot, assert_snapshot};
use openlark_docs::{
    BitableRecordQuery, DriveDownloadRange, DriveUploadFile, SheetRange, SheetWriteRange,
    TypedPage, WikiNodePath,
};
use serde_json::json;

#[test]
fn docs_helper_outputs_snapshot() {
    let range = SheetRange::from_range_expr("sheet_001", "A1:C5").expect("range should parse");
    let write_range = SheetWriteRange::new(
        range.clone(),
        vec![
            vec![json!("状态"), json!("进行中")],
            vec![json!("负责人"), json!("张三")],
        ],
    );
    let download = DriveDownloadRange::from_start(0).with_end(1023);
    let upload = DriveUploadFile::new("report.csv", b"status,owner\nactive,zhangsan".to_vec())
        .checksum("adler32-demo");
    let wiki_path = WikiNodePath::parse("产品文档/发布计划").expect("wiki path should parse");
    let page = TypedPage::new(
        vec![json!({"title":"汇总表"}), json!({"title":"排期表"})],
        true,
        Some("next_page_token".to_string()),
    );

    assert_json_snapshot!(
        "docs_helper_outputs",
        json!({
            "sheet_range": {
                "a1_notation": range.to_a1_notation(),
                "range_expr": range.range_expr(),
            },
            "sheet_write_range": {
                "range": write_range.range.to_string(),
                "major_dimension": write_range.major_dimension,
                "values": write_range.values,
            },
            "drive_download_range": download.to_header_value(),
            "drive_upload_file": {
                "file_name": upload.file_name,
                "size": upload.size(),
                "checksum": upload.checksum,
            },
            "wiki_path": wiki_path.to_string(),
            "typed_page": {
                "items": page.items,
                "has_more": page.has_more,
                "next_page_token": page.next_page_token,
            }
        })
    );
}

#[test]
fn docs_bitable_query_debug_snapshot() {
    let query = BitableRecordQuery::new("app_token", "table_id")
        .field_names(vec!["状态".to_string(), "负责人".to_string()])
        .where_equals("状态", "进行中")
        .or()
        .where_in("负责人", vec!["ou_a".to_string(), "ou_b".to_string()]);

    assert_snapshot!("docs_bitable_query_debug", format!("{query:#?}"));
}
