//! 按条件更新数据表中的记录
//!
//! URL: PATCH:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/records_patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 按条件更新记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsPatchRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 筛选条件（PostgREST 语法，作为必填查询参数发送）
    filter: String,
    /// 要更新的数据记录（发送时序列化为 JSON 字符串）
    record: serde_json::Value,
}

impl TableRecordsPatchRequestBuilder {
    /// 创建新的 Builder
    ///
    /// `filter` 为筛选条件，遵循 PostgREST 语法（如 `age=gt.10`），URLEncode 后作为查询参数发送
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        table_name: impl Into<String>,
        filter: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            table_name: table_name.into(),
            filter: filter.into(),
            record: serde_json::json!({}),
        }
    }

    /// 设置要更新的数据记录（如 `json!({"age": 10})`）
    pub fn record(mut self, record: impl Into<serde_json::Value>) -> Self {
        self.record = record.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsPatchResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        validate_required!(self.filter, "filter 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        // 官方契约：请求体 record 为 JSON 序列化字符串（如 "{\"age\":10}"）
        let request = TableRecordsPatchRequest {
            record: serde_json::to_string(&self.record)?,
        };

        let req: ApiRequest<TableRecordsPatchResponse> = ApiRequest::patch(&url)
            .body(serde_json::to_value(&request)?)
            .query("filter", &self.filter);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 按条件更新记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct TableRecordsPatchRequest {
    /// 要更新的数据记录信息（JSON 序列化字符串）
    #[serde(rename = "record")]
    record: String,
}

/// 按条件更新记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableRecordsPatchResponse {
    /// 更新的记录唯一 ID 列表
    #[serde(rename = "record_ids")]
    pub record_ids: Vec<String>,
}

impl ApiResponseTrait for TableRecordsPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsPatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsPatchBuilder = TableRecordsPatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../tables/{table}/records?filter=... → 强类型 TableRecordsPatchResponse。
    /// 官方契约：filter 为必填查询参数，请求体仅含 record（JSON 字符串），响应为 record_ids。
    #[tokio::test]
    async fn test_patch_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "record_ids": [
                        "1cbb280d-fc3d-4dca-9db5-adb14c4c83ec",
                        "2dcb391e-0d4e-5edb-aec6-bec25d5d94fd",
                        "3edc4a2f-1e5f-6fce-bfd7-cfd36e6ea5ge"
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

        let resp = TableRecordsPatchRequestBuilder::new(config, "ws_001", "user", "age=gt.10")
            .record(json!({"age": 10}))
            .execute()
            .await
            .expect("按条件更新记录应成功");
        assert_eq!(resp.record_ids.len(), 3);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "PATCH");
        // filter 应为查询参数（PostgREST 语法 `=` 会被 percent-encode 为 %3D）
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("filter=age%3Dgt.10"),
            "query should contain encoded filter: {query}"
        );
        // 请求体应仅含 record 字符串字段
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(body, json!({"record": "{\"age\":10}"}));
    }
}
