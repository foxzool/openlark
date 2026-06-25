//! Representative contract tests for high-frequency Docs request/response models.
#![cfg(all(feature = "ccm-core", feature = "bitable"))]

use openlark_core::config::Config;
use openlark_docs::base::bitable::v1::{SearchRecordRequestBody, SearchRecordResponse};
use openlark_docs::ccm::docx::v1::document::{
    GetDocumentRawContentParams, GetDocumentRawContentResponse,
};
use openlark_docs::ccm::drive::v1::file::{UploadPrepareRequest, UploadPrepareResponse};
use openlark_docs::ccm::sheets::v3::{CreateSpreadsheetParams, GetSpreadsheetResponse};
use openlark_docs::ccm::wiki::v2::get_node::{GetWikiSpaceNodeParams, GetWikiSpaceNodeResponse};
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
fn drive_upload_prepare_request_and_response_contract() {
    let request = UploadPrepareRequest::new(
        Config::default(),
        "quarterly-report.xlsx",
        "fld_drive_root",
        4096,
    );
    assert_json_contract(
        &request,
        json!({
            "file_name": "quarterly-report.xlsx",
            "parent_type": "explorer",
            "parent_node": "fld_drive_root",
            "size": 4096
        }),
    );

    let response: UploadPrepareResponse = parse_contract(json!({
        "upload_id": "upload_123",
        "block_size": 4194304,
        "block_num": 2
    }));
    assert_eq!(response.upload_id, "upload_123");
    assert_json_contract(
        &response,
        json!({
            "upload_id": "upload_123",
            "block_size": 4194304,
            "block_num": 2
        }),
    );
}

#[test]
fn docx_request_and_response_contract() {
    let params = GetDocumentRawContentParams {
        document_id: "doccnxxxxxxxx".to_string(),
        lang: Some(1),
    };
    assert_json_contract(
        &params,
        json!({
            "document_id": "doccnxxxxxxxx",
            "lang": 1
        }),
    );

    let response: GetDocumentRawContentResponse = parse_contract(json!({
        "content": "第一行\n第二行"
    }));
    assert_eq!(response.content, "第一行\n第二行");
    assert_json_contract(
        &response,
        json!({
            "content": "第一行\n第二行"
        }),
    );
}

#[test]
fn sheets_request_and_response_contract() {
    let params = CreateSpreadsheetParams {
        title: Some("运营周报".to_string()),
        folder_token: Some("fld_sheet_root".to_string()),
    };
    assert_json_contract(
        &params,
        json!({
            "title": "运营周报",
            "folder_token": "fld_sheet_root"
        }),
    );

    let response: GetSpreadsheetResponse = parse_contract(json!({
        "spreadsheet": {
            "title": "运营周报",
            "owner_id": "ou_xxx",
            "token": "shtcnxxxx",
            "url": "https://open.feishu.cn/sheets/shtcnxxxx"
        }
    }));
    assert_eq!(response.spreadsheet.token, "shtcnxxxx");
    assert_json_contract(
        &response,
        json!({
            "spreadsheet": {
                "title": "运营周报",
                "owner_id": "ou_xxx",
                "token": "shtcnxxxx",
                "url": "https://open.feishu.cn/sheets/shtcnxxxx"
            }
        }),
    );
}

#[test]
fn bitable_request_and_response_contract() {
    let request_json = json!({
        "view_id": "vew_status",
        "field_names": ["状态", "负责人"],
        "sort": [
            {
                "field_name": "更新时间",
                "desc": true
            }
        ],
        "filter": {
            "conjunction": "and",
            "conditions": [
                {
                    "field_name": "状态",
                    "operator": "is",
                    "value": ["进行中"]
                }
            ]
        },
        "automatic_fields": true
    });
    let request: SearchRecordRequestBody = parse_contract(request_json.clone());
    assert_json_contract(&request, request_json);

    let response: SearchRecordResponse = parse_contract(json!({
        "items": [
            {
                "record_id": "rec001",
                "fields": {
                    "状态": "进行中",
                    "负责人": [{"id": "ou_123"}]
                },
                "created_by": {
                    "id": "ou_123",
                    "name": "张三"
                },
                "created_time": 1710000000000i64,
                "last_modified_by": {
                    "id": "ou_456",
                    "name": "李四"
                },
                "last_modified_time": 1710000001000i64,
                "shared_url": "https://open.feishu.cn/bitable/shared/rec001",
                "record_url": "https://open.feishu.cn/bitable/record/rec001"
            }
        ],
        "has_more": false,
        "page_token": null,
        "total": 1
    }));
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.items[0].record_id, "rec001");
    assert_eq!(response.total, 1);
    assert_json_contract(
        &response,
        json!({
            "items": [
                {
                    "record_id": "rec001",
                    "fields": {
                        "状态": "进行中",
                        "负责人": [{"id": "ou_123"}]
                    },
                    "created_by": {
                        "id": "ou_123",
                        "name": "张三"
                    },
                    "created_time": 1710000000000i64,
                    "last_modified_by": {
                        "id": "ou_456",
                        "name": "李四"
                    },
                    "last_modified_time": 1710000001000i64,
                    "shared_url": "https://open.feishu.cn/bitable/shared/rec001",
                    "record_url": "https://open.feishu.cn/bitable/record/rec001"
                }
            ],
            "has_more": false,
            "page_token": null,
            "total": 1
        }),
    );
}

#[test]
fn wiki_request_and_response_contract() {
    let params = GetWikiSpaceNodeParams {
        token: "wiki_node_token".to_string(),
        obj_type: Some("docx".to_string()),
    };
    assert_json_contract(
        &params,
        json!({
            "token": "wiki_node_token",
            "obj_type": "docx"
        }),
    );

    let response: GetWikiSpaceNodeResponse = parse_contract(json!({
        "node": {
            "space_id": "space_123",
            "node_token": "wiki_node_token",
            "obj_token": "doccnxxxx",
            "obj_type": "docx",
            "parent_node_token": "parent_node_token",
            "title": "发布计划",
            "url": "https://open.feishu.cn/wiki/wiki_node_token"
        }
    }));
    let node = response.node.expect("wiki node should exist");
    assert_eq!(node.space_id, "space_123");
    assert_eq!(node.title.as_deref(), Some("发布计划"));
    assert_json_contract(
        &GetWikiSpaceNodeResponse { node: Some(node) },
        json!({
            "node": {
                "space_id": "space_123",
                "node_token": "wiki_node_token",
                "obj_token": "doccnxxxx",
                "obj_type": "docx",
                "parent_node_token": "parent_node_token",
                "title": "发布计划",
                "url": "https://open.feishu.cn/wiki/wiki_node_token"
            }
        }),
    );
}
