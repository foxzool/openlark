//! Representative contract tests for analytics search request/response models.
#![cfg(feature = "search")]

use openlark_analytics::common::{SearchResult, SearchStats};
use openlark_analytics::search::search::v2::data_source::get::{
    DataSource, GetDataSourceResponse, I18nMeta,
};
use openlark_analytics::search::search::v2::data_source::item::get::{
    DataSourceItemData, GetDataSourceItemResponse,
};
use openlark_analytics::search::search::v2::schema::get::{
    GetSchemaResponse, Schema, SchemaDisplay, SchemaDisplayFieldMapping, SchemaProperty,
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
    // 官方响应形状：data 内容为 { "schema": { schema_id, properties, display } }
    let response: GetSchemaResponse = parse_contract(json!({
        "schema": {
            "schema_id": "sch_001",
            "display": {
                "card_key": "search_common_card",
                "fields_mapping": [
                    { "display_field": "tag1", "data_field": "${priority}" },
                    { "display_field": "summary", "data_field": "${description}" }
                ]
            },
            "properties": [
                {
                    "name": "description",
                    "type": "text",
                    "is_searchable": true,
                    "is_returnable": true,
                    "search_options": { "enable_semantic_match": true }
                },
                {
                    "name": "priority",
                    "type": "tag",
                    "is_returnable": true,
                    "type_definitions": {
                        "tag": [ { "name": "HIGH", "color": "red", "text": "紧急" } ]
                    }
                }
            ]
        }
    }));
    let schema = response.schema.expect("schema should exist");
    assert_eq!(schema.schema_id.as_deref(), Some("sch_001"));
    let properties = schema.properties.expect("properties should exist");
    assert_eq!(properties.len(), 2);
    assert_eq!(properties[0].name.as_deref(), Some("description"));
    assert_eq!(properties[0].type_.as_deref(), Some("text"));
    assert_eq!(properties[0].is_searchable, Some(true));
    assert_eq!(
        properties[1]
            .type_definitions
            .as_ref()
            .and_then(|d| d.tag.as_ref())
            .map(|t| t.len()),
        Some(1)
    );
    assert_eq!(
        schema.display.as_ref().and_then(|d| d.card_key.as_deref()),
        Some("search_common_card")
    );
    assert_json_contract(
        &GetSchemaResponse {
            schema: Some(Schema {
                schema_id: Some("sch_001".to_string()),
                properties: Some(vec![
                    SchemaProperty {
                        name: Some("description".to_string()),
                        type_: Some("text".to_string()),
                        is_searchable: Some(true),
                        is_sortable: None,
                        is_returnable: Some(true),
                        sort_options: None,
                        type_definitions: None,
                        search_options: None,
                    },
                    SchemaProperty {
                        name: Some("priority".to_string()),
                        type_: Some("tag".to_string()),
                        is_searchable: None,
                        is_sortable: None,
                        is_returnable: Some(true),
                        sort_options: None,
                        type_definitions: None,
                        search_options: None,
                    },
                ]),
                display: Some(SchemaDisplay {
                    card_key: Some("search_common_card".to_string()),
                    fields_mapping: Some(vec![
                        SchemaDisplayFieldMapping {
                            display_field: Some("tag1".to_string()),
                            data_field: Some("${priority}".to_string()),
                        },
                        SchemaDisplayFieldMapping {
                            display_field: Some("summary".to_string()),
                            data_field: Some("${description}".to_string()),
                        },
                    ]),
                }),
            }),
        },
        // skip_serializing_if 不出现在 to_value 输出中
        json!({
            "schema": {
                "schema_id": "sch_001",
                "properties": [
                    {
                        "name": "description",
                        "type": "text",
                        "is_searchable": true,
                        "is_returnable": true
                    },
                    {
                        "name": "priority",
                        "type": "tag",
                        "is_returnable": true
                    }
                ],
                "display": {
                    "card_key": "search_common_card",
                    "fields_mapping": [
                        { "display_field": "tag1", "data_field": "${priority}" },
                        { "display_field": "summary", "data_field": "${description}" }
                    ]
                }
            }
        }),
    );
}

#[test]
fn schema_data_roundtrip() {
    let original = Schema {
        schema_id: Some("sch_roundtrip".to_string()),
        properties: Some(vec![SchemaProperty {
            name: Some("url".to_string()),
            type_: Some("text".to_string()),
            is_searchable: None,
            is_sortable: None,
            is_returnable: Some(true),
            sort_options: None,
            type_definitions: None,
            search_options: None,
        }]),
        display: None,
    };
    let json_val = to_value(&original).unwrap();
    let restored: Schema = from_value(json_val.clone()).unwrap();
    assert_eq!(restored.schema_id, original.schema_id);
    assert_eq!(restored.properties.as_ref().map(|p| p.len()), Some(1));
    assert_json_contract(&restored, json_val);
}

#[test]
fn data_source_data_contract() {
    // 官方响应形状：data 内容为 { "data_source": { id, name, state, ... } }
    let response: GetDataSourceResponse = parse_contract(json!({
        "data_source": {
            "id": "ds_001",
            "name": "内部知识库",
            "state": 0,
            "description": "企业内部知识库数据源",
            "create_time": "1618829192",
            "update_time": "1618829192",
            "is_exceed_quota": false,
            "icon_url": "https://www.xxx.com/open.jpg",
            "template": "search_common_card",
            "i18n_name": { "zh_cn": "任务", "en_us": "TODO", "ja_jp": "タスク" },
            "schema_id": "custom_schema",
            "app_id": "cli_a1306bed4738d01b"
        }
    }));
    let ds = response.data_source.expect("data source should exist");
    assert_eq!(ds.id.as_deref(), Some("ds_001"));
    assert_eq!(ds.name.as_deref(), Some("内部知识库"));
    assert_eq!(ds.state, Some(0));
    assert_eq!(ds.is_exceed_quota, Some(false));
    assert_eq!(
        ds.i18n_name.as_ref().and_then(|m| m.en_us.as_deref()),
        Some("TODO")
    );
    assert_eq!(ds.schema_id.as_deref(), Some("custom_schema"));
    assert_json_contract(
        &GetDataSourceResponse {
            data_source: Some(DataSource {
                id: Some("ds_001".to_string()),
                name: Some("内部知识库".to_string()),
                state: Some(0),
                description: Some("企业内部知识库数据源".to_string()),
                create_time: Some("1618829192".to_string()),
                update_time: Some("1618829192".to_string()),
                is_exceed_quota: Some(false),
                icon_url: Some("https://www.xxx.com/open.jpg".to_string()),
                template: Some("search_common_card".to_string()),
                searchable_fields: None,
                i18n_name: Some(I18nMeta {
                    zh_cn: Some("任务".to_string()),
                    en_us: Some("TODO".to_string()),
                    ja_jp: Some("タスク".to_string()),
                }),
                i18n_description: None,
                schema_id: Some("custom_schema".to_string()),
                app_id: Some("cli_a1306bed4738d01b".to_string()),
            }),
        },
        // skip_serializing_if 不出现在 to_value 输出中
        json!({
            "data_source": {
                "id": "ds_001",
                "name": "内部知识库",
                "state": 0,
                "description": "企业内部知识库数据源",
                "create_time": "1618829192",
                "update_time": "1618829192",
                "is_exceed_quota": false,
                "icon_url": "https://www.xxx.com/open.jpg",
                "template": "search_common_card",
                "i18n_name": { "zh_cn": "任务", "en_us": "TODO", "ja_jp": "タスク" },
                "schema_id": "custom_schema",
                "app_id": "cli_a1306bed4738d01b"
            }
        }),
    );
}

#[test]
fn data_source_data_roundtrip() {
    let original = DataSource {
        id: Some("ds_roundtrip".to_string()),
        name: Some("测试数据源".to_string()),
        state: Some(1),
        description: Some("用于契约测试".to_string()),
        create_time: None,
        update_time: None,
        is_exceed_quota: None,
        icon_url: None,
        template: None,
        searchable_fields: None,
        i18n_name: None,
        i18n_description: None,
        schema_id: None,
        app_id: None,
    };
    let json_val = to_value(&original).unwrap();
    let restored: DataSource = from_value(json_val.clone()).unwrap();
    assert_eq!(restored.id, original.id);
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
    let schema = Schema {
        schema_id: Some("sch_empty".to_string()),
        properties: Some(vec![]),
        display: None,
    };
    let json_val = to_value(&schema).unwrap();
    let restored: Schema = from_value(json_val).unwrap();
    assert!(restored.properties.as_ref().is_some_and(|p| p.is_empty()));
    assert_eq!(restored.schema_id.as_deref(), Some("sch_empty"));
}
