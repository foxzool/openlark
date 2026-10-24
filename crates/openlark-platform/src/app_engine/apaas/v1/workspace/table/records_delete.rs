//! 删除数据表中的记录
//!
//! URL: DELETE:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name/records
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/records_delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除数据表记录 Builder
#[derive(Debug, Clone)]
pub struct TableRecordsDeleteRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
    /// 筛选条件（PostgREST 语法，作为必填查询参数发送）
    filter: String,
}

impl TableRecordsDeleteRequestBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableRecordsDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TableRecordsDeleteResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");
        validate_required!(self.filter, "filter 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}/records",
            self.workspace_id, self.table_name
        );

        // 官方契约：DELETE 无请求体，必填查询参数 filter，响应 data 为空对象
        let req: ApiRequest<TableRecordsDeleteResponse> =
            ApiRequest::delete(&url).query("filter", &self.filter);
        Transport::request_typed(req, &self.config, Some(option), "删除数据表记录").await
    }
}

/// 删除记录响应（官方 data 为空对象 `{"code": 0, "msg": "success", "data": {}}`）
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRecordsDeleteResponse {}

impl ApiResponseTrait for TableRecordsDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableRecordsDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableRecordsDeleteBuilder = TableRecordsDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../tables/{table}/records?filter=... → 官方 data 为空对象。
    #[tokio::test]
    async fn test_delete_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let _resp = TableRecordsDeleteRequestBuilder::new(config, "ws_001", "user", "age=gt.10")
            .execute()
            .await
            .expect("删除数据表记录应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user/records"
        );
        assert_eq!(received[0].method, "DELETE");
        // filter 应为查询参数（PostgREST 语法 `=` 会被 percent-encode 为 %3D）
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("filter=age%3Dgt.10"),
            "query should contain encoded filter: {query}"
        );
        // 官方契约：DELETE 无请求体
        assert!(
            received[0].body.is_empty(),
            "DELETE 请求不应携带请求体: {}",
            String::from_utf8_lossy(&received[0].body)
        );
    }
}
