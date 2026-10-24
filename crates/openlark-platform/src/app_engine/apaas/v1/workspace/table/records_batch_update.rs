//! 批量更新数据表中的记录
//!
//! URL: PATCH:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records_batch_update
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/records_batch_update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsBatchUpdateRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 要更新的记录列表（每个元素为主键 `_id` + 平级更新字段）
    records: Vec<serde_json::Value>,
}

impl TableRecordsBatchUpdateRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
            records: Vec::new(),
        }
    }

    /// 添加要更新的记录（`record_id` 为主键 `_id`，`data` 中的更新字段与其平级展开）
    pub fn record(
        mut self,
        record_id: impl Into<String>,
        data: impl Into<serde_json::Value>,
    ) -> Self {
        self.records
            .push(flatten_record(record_id.into(), data.into()));
        self
    }

    /// 添加多条要更新的记录
    pub fn records(
        mut self,
        records: impl IntoIterator<Item = (impl Into<String>, impl Into<serde_json::Value>)>,
    ) -> Self {
        self.records.extend(
            records
                .into_iter()
                .map(|(id, data)| flatten_record(id.into(), data.into())),
        );
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsBatchUpdateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsBatchUpdateResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        validate_required_list!(self.records, 500, "records 不能为空且单次最多 500 条");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records_batch_update",
            self.workspace_id, self.table_name
        );

        // 官方契约：请求体 records 为数组序列化后的 JSON 字符串，每行以主键 _id 标识
        let request = TableRecordsBatchUpdateRequest {
            records: serde_json::to_string(&self.records)?,
        };

        let req: ApiRequest<TableRecordsBatchUpdateResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);

        Transport::request_typed(req, &self.config, Some(option), "批量更新记录").await
    }
}

/// 将主键与更新字段组装为官方契约的平级记录结构 `{\"_id\": ..., ...更新字段}`
fn flatten_record(record_id: String, data: serde_json::Value) -> serde_json::Value {
    let mut item = serde_json::Map::new();
    item.insert("_id".to_string(), serde_json::Value::String(record_id));
    if let serde_json::Value::Object(fields) = data {
        for (key, value) in fields {
            item.insert(key, value);
        }
    }
    serde_json::Value::Object(item)
}

/// 批量更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsBatchUpdateRequest {
    /// 要更新的数据记录列表（JSON 序列化字符串，单次最多 500 条，
    /// 每行 record 都必须包含主键 `_id`，且不同行要更新的字段需保持一致）
    #[serde(rename = "records")]
    records: String,
}

/// 批量更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsBatchUpdateResponse {
    /// 更新的记录唯一 ID 列表
    #[serde(rename = "record_ids")]
    pub record_ids: Vec<String>,
}

impl ApiResponseTrait for TableRecordsBatchUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to TableRecordsBatchUpdateRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type TableRecordsBatchUpdateBuilder = TableRecordsBatchUpdateRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../tables/{table}/records_batch_update → 强类型 TableRecordsBatchUpdateResponse。
    /// 官方契约：请求体 records 为字符串化 JSON 数组（元素含主键 _id + 平级更新字段），响应为 record_ids。
    #[tokio::test]
    async fn test_batch_update_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records_batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "record_ids": [
                        "657fade8-394d-4d86-aa35-0129e3bd7614",
                        "1cbb280d-fc3d-4dca-9db5-adb14c4c83ec"
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = TableRecordsBatchUpdateRequestBuilder::new(config, "ws_001", "user")
            .record("657fade8-394d-4d86-aa35-0129e3bd7614", json!({"age": 10}))
            .record("1cbb280d-fc3d-4dca-9db5-adb14c4c83ec", json!({"age": 20}))
            .execute()
            .await
            .expect("批量更新记录应成功");
        assert_eq!(resp.record_ids.len(), 2);
        assert_eq!(resp.record_ids[0], "657fade8-394d-4d86-aa35-0129e3bd7614");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records_batch_update"
        );
        assert_eq!(received[0].method, "PATCH");
        // 请求体 records 应为字符串化 JSON 数组，元素为 _id + 平级更新字段
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert!(body["records"].is_string(), "records 应为字符串: {body}");
        let records: Vec<serde_json::Value> =
            serde_json::from_str(body["records"].as_str().unwrap())
                .expect("records 字符串应为 JSON 数组");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["_id"], "657fade8-394d-4d86-aa35-0129e3bd7614");
        assert_eq!(records[0]["age"], 10);
    }
}
