//! 向数据表中添加或更新记录
//!
//! URL: POST:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/records_post>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 添加或更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsPostRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 记录数据列表（发送时序列化为 JSON 字符串）
    records: Vec<serde_json::Value>,
    /// UPSERT 时使用，指定列，多列英文逗号拼接
    columns: Option<String>,
    /// UPSERT 时使用，指定作为冲突判断依据的唯一约束字段，默认为表主键
    on_conflict: Option<String>,
}

impl TableRecordsPostRequestBuilder {
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
            columns: None,
            on_conflict: None,
        }
    }

    /// 添加记录数据
    pub fn record(mut self, record: impl Into<serde_json::Value>) -> Self {
        self.records.push(record.into());
        self
    }

    /// 添加多条记录数据
    pub fn records(
        mut self,
        records: impl IntoIterator<Item = impl Into<serde_json::Value>>,
    ) -> Self {
        self.records.extend(records.into_iter().map(Into::into));
        self
    }

    /// 设置 UPSERT 指定列，多列英文逗号拼接
    pub fn columns(mut self, columns: impl Into<String>) -> Self {
        self.columns = Some(columns.into());
        self
    }

    /// 设置 UPSERT 冲突判断依据的唯一约束字段，默认为表主键
    pub fn on_conflict(mut self, on_conflict: impl Into<String>) -> Self {
        self.on_conflict = Some(on_conflict.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsPostResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsPostResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        validate_required_list!(self.records, 500, "records 不能为空且单次最多 500 条");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        // 官方契约：请求体 records 为数组序列化后的 JSON 字符串
        let request = TableRecordsPostRequest {
            records: serde_json::to_string(&self.records)?,
        };

        let mut req: ApiRequest<TableRecordsPostResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        if let Some(columns) = self.columns {
            req = req.query("columns", &columns);
        }
        if let Some(on_conflict) = self.on_conflict {
            req = req.query("on_conflict", &on_conflict);
        }
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 添加或更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsPostRequest {
    /// 要插入的数据记录列表（JSON 序列化字符串，单次最多 500 条）
    #[serde(rename = "records")]
    records: String,
}

/// 添加或更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsPostResponse {
    /// 按照记录顺序创建或更新的记录 ID 列表
    #[serde(rename = "record_ids")]
    pub record_ids: Vec<String>,
}

impl ApiResponseTrait for TableRecordsPostResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsPostRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsPostBuilder = TableRecordsPostRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../tables/{table}/records → 强类型 TableRecordsPostResponse。
    /// 官方契约：请求体 records 为字符串化 JSON 数组，UPSERT 用 columns/on_conflict 查询参数。
    #[tokio::test]
    async fn test_post_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "record_ids": [
                        "1cbb280d-fc3d-4dca-9db5-adb14c4c83ec",
                        "2dcb391e-0d4e-5edb-aec6-bec25d5d94fd"
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

        let resp = TableRecordsPostRequestBuilder::new(config, "ws_001", "user")
            .record(json!({"name": "王一一", "gender": "male", "age": 10}))
            .record(json!({"name": "王二二", "gender": "female", "age": 20}))
            .columns("name")
            .on_conflict("name")
            .execute()
            .await
            .expect("添加或更新记录应成功");
        assert_eq!(resp.record_ids.len(), 2);
        assert_eq!(resp.record_ids[0], "1cbb280d-fc3d-4dca-9db5-adb14c4c83ec");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "POST");
        // 请求体 records 应为字符串化 JSON 数组
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert!(body["records"].is_string(), "records 应为字符串: {body}");
        let records: Vec<serde_json::Value> =
            serde_json::from_str(body["records"].as_str().unwrap())
                .expect("records 字符串应为 JSON 数组");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0]["name"], "王一一");
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("columns=name"),
            "query should contain columns=name: {query}"
        );
        assert!(
            query.contains("on_conflict=name"),
            "query should contain on_conflict=name: {query}"
        );
    }
}
