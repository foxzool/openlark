//! Representative contract tests for analytics search request/response models.
#![cfg(feature = "search")]

use openlark_analytics::common::{SearchResult, SearchStats};
use openlark_analytics::search::search::v2::data_source::get::{
    DataSourceData, GetDataSourceResponse,
};
use openlark_analytics::search::search::v2::data_source::item::get::{
    DataSourceItemData, GetDataSourceItemResponse,
};
use openlark_analytics::search::search::v2::schema::get::{
    GetSchemaResponse, SchemaData, SchemaField,
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
fn search_result_contract() {
    let result = SearchResult {
        id: "result_001".to_string(),
        title: "季度报告".to_string(),
        summary: Some("2025年Q4季度业绩报告".to_string()),
        result_type: "doc".to_string(),
        score: 0.95,
    };
    assert_json_contract(
        &result,
        json!({
            "id": "result_001",
            "title": "季度报告",
            "summary": "2025年Q4季度业绩报告",
            "result_type": "doc",
            "score": 0.95
        }),
    );

    let roundtrip: SearchResult = parse_contract(json!({
        "id": "result_002",
        "title": "会议纪要",
        "summary": null,
        "result_type": "wiki",
        "score": 0.78
    }));
    assert_eq!(roundtrip.id, "result_002");
    assert!(roundtrip.summary.is_none());
    assert_json_contract(
        &roundtrip,
        json!({
            "id": "result_002",
            "title": "会议纪要",
            "summary": null,
            "result_type": "wiki",
            "score": 0.78
        }),
    );
}

#[test]
fn search_stats_contract() {
    let stats = SearchStats {
        total: 128,
        query_time_ms: 45,
        page_count: 5,
    };
    assert_json_contract(
        &stats,
        json!({
            "total": 128,
            "query_time_ms": 45,
            "page_count": 5
        }),
    );

    let parsed: SearchStats = parse_contract(json!({
        "total": 0,
        "query_time_ms": 12,
        "page_count": 0
    }));
    assert_eq!(parsed.total, 0);
    assert_eq!(parsed.query_time_ms, 12);
    assert_eq!(parsed.page_count, 0);
    assert_json_contract(
        &parsed,
        json!({
            "total": 0,
            "query_time_ms": 12,
            "page_count": 0
        }),
    );
}

#[test]
fn schema_data_contract() {
    let response: GetSchemaResponse = parse_contract(json!({
        "data": {
            "schema_id": "sch_001",
            "name": "文档索引范式",
            "fields": [
                {
                    "field_name": "title",
                    "field_type": "TEXT"
                },
                {
                    "field_name": "created_at",
                    "field_type": "DATETIME"
                }
            ]
        }
    }));
    let schema = response.data.expect("schema data should exist");
    assert_eq!(schema.schema_id, "sch_001");
    assert_eq!(schema.name, "文档索引范式");
    assert_eq!(schema.fields.len(), 2);
    assert_eq!(schema.fields[0].field_name, "title");
    assert_eq!(schema.fields[0].field_type, "TEXT");
    assert_eq!(schema.fields[1].field_name, "created_at");
    assert_eq!(schema.fields[1].field_type, "DATETIME");
    assert_json_contract(
        &GetSchemaResponse {
            data: Some(SchemaData {
                schema_id: "sch_001".to_string(),
                name: "文档索引范式".to_string(),
                fields: vec![
                    SchemaField {
                        field_name: "title".to_string(),
                        field_type: "TEXT".to_string(),
                    },
                    SchemaField {
                        field_name: "created_at".to_string(),
                        field_type: "DATETIME".to_string(),
                    },
                ],
            }),
        },
        json!({
            "data": {
                "schema_id": "sch_001",
                "name": "文档索引范式",
                "fields": [
                    {
                        "field_name": "title",
                        "field_type": "TEXT"
                    },
                    {
                        "field_name": "created_at",
                        "field_type": "DATETIME"
                    }
                ]
            }
        }),
    );
}

#[test]
fn schema_data_roundtrip() {
    let original = SchemaData {
        schema_id: "sch_roundtrip".to_string(),
        name: "测试范式".to_string(),
        fields: vec![SchemaField {
            field_name: "url".to_string(),
            field_type: "URL".to_string(),
        }],
    };
    let json_val = to_value(&original).unwrap();
    let restored: SchemaData = from_value(json_val.clone()).unwrap();
    assert_eq!(restored.schema_id, original.schema_id);
    assert_eq!(restored.fields.len(), 1);
    assert_json_contract(&restored, json_val);
}

#[test]
fn data_source_data_contract() {
    let response: GetDataSourceResponse = parse_contract(json!({
        "data": {
            "data_source_id": "ds_001",
            "name": "内部知识库",
            "description": "企业内部知识库数据源"
        }
    }));
    let ds = response.data.expect("data source should exist");
    assert_eq!(ds.data_source_id, "ds_001");
    assert_eq!(ds.name, "内部知识库");
    assert_eq!(ds.description, "企业内部知识库数据源");
    assert_json_contract(
        &GetDataSourceResponse {
            data: Some(DataSourceData {
                data_source_id: "ds_001".to_string(),
                name: "内部知识库".to_string(),
                description: "企业内部知识库数据源".to_string(),
            }),
        },
        json!({
            "data": {
                "data_source_id": "ds_001",
                "name": "内部知识库",
                "description": "企业内部知识库数据源"
            }
        }),
    );
}

#[test]
fn data_source_data_roundtrip() {
    let original = DataSourceData {
        data_source_id: "ds_roundtrip".to_string(),
        name: "测试数据源".to_string(),
        description: "用于契约测试".to_string(),
    };
    let json_val = to_value(&original).unwrap();
    let restored: DataSourceData = from_value(json_val.clone()).unwrap();
    assert_eq!(restored.data_source_id, original.data_source_id);
    assert_eq!(restored.name, original.name);
    assert_eq!(restored.description, original.description);
    assert_json_contract(&restored, json_val);
}

#[test]
fn data_source_item_data_contract() {
    let response: GetDataSourceItemResponse = parse_contract(json!({
        "data": {
            "item_id": "item_001",
            "data": {
                "title": "飞书API文档",
                "url": "https://open.feishu.cn/document",
                "tags": ["API", "开发"]
            }
        }
    }));
    let item = response.data.expect("data source item should exist");
    assert_eq!(item.item_id, "item_001");
    assert_eq!(item.data["title"], "飞书API文档");
    assert_eq!(item.data["tags"][0], "API");
    assert_json_contract(
        &GetDataSourceItemResponse {
            data: Some(DataSourceItemData {
                item_id: "item_001".to_string(),
                data: json!({
                    "title": "飞书API文档",
                    "url": "https://open.feishu.cn/document",
                    "tags": ["API", "开发"]
                }),
            }),
        },
        json!({
            "data": {
                "item_id": "item_001",
                "data": {
                    "title": "飞书API文档",
                    "url": "https://open.feishu.cn/document",
                    "tags": ["API", "开发"]
                }
            }
        }),
    );
}

#[test]
fn data_source_item_roundtrip() {
    let original = DataSourceItemData {
        item_id: "item_roundtrip".to_string(),
        data: json!({"key": "value", "nested": {"a": 1}}),
    };
    let json_val = to_value(&original).unwrap();
    let restored: DataSourceItemData = from_value(json_val.clone()).unwrap();
    assert_eq!(restored.item_id, original.item_id);
    assert_eq!(restored.data["key"], "value");
    assert_eq!(restored.data["nested"]["a"], 1);
    assert_json_contract(&restored, json_val);
}

#[test]
fn optional_fields_search_result() {
    let result: SearchResult = parse_contract(json!({
        "id": "result_minimal",
        "title": "最小结果",
        "summary": null,
        "result_type": "message",
        "score": 0.0
    }));
    assert!(result.summary.is_none());
    assert_eq!(result.score, 0.0);

    let full: SearchResult = parse_contract(json!({
        "id": "result_full",
        "title": "完整结果",
        "summary": "包含摘要",
        "result_type": "app",
        "score": 1.0
    }));
    assert_eq!(full.summary.as_deref(), Some("包含摘要"));
    assert_eq!(full.score, 1.0);
}

#[test]
fn schema_empty_fields() {
    let schema = SchemaData {
        schema_id: "sch_empty".to_string(),
        name: "空字段范式".to_string(),
        fields: vec![],
    };
    let json_val = to_value(&schema).unwrap();
    let restored: SchemaData = from_value(json_val).unwrap();
    assert!(restored.fields.is_empty());
    assert_eq!(restored.schema_id, "sch_empty");
}
