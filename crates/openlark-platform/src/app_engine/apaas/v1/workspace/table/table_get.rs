//! 获取数据表详细信息
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取数据表详情 Builder
#[derive(Debug, Clone)]
pub struct TableGetRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 数据表名称
    table_name: String,
}

impl TableGetRequestBuilder {
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
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<TableGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}",
            self.workspace_id, self.table_name
        );

        let req: ApiRequest<TableGetResponse> = ApiRequest::get(&url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 字段信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldInfo {
    /// 字段名称
    #[serde(rename = "field_name")]
    field_name: String,
    /// 字段类型
    #[serde(rename = "field_type")]
    field_type: String,
    /// 是否为主键
    #[serde(rename = "is_primary_key")]
    is_primary_key: bool,
    /// 字段描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// 数据表详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableGetResponse {
    /// 数据表名称
    #[serde(rename = "table_name")]
    pub table_name: String,
    /// 数据表描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 字段列表
    #[serde(rename = "fields")]
    pub fields: Vec<FieldInfo>,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    pub updated_time: i64,
}

impl ApiResponseTrait for TableGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableGetBuilder = TableGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../apaas/v1/workspaces/{ws}/tables/{table_name} → 强类型 TableGetResponse。
    #[tokio::test]
    async fn test_get_table_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/workspaces/ws_001/tables/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "table_name": "user",
                    "description": "用户表",
                    "fields": [
                        {"field_name": "id", "field_type": "int", "is_primary_key": true, "description": "主键"},
                        {"field_name": "name", "field_type": "string", "is_primary_key": false}
                    ],
                    "created_time": 1700000000,
                    "updated_time": 1700000100
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

        let resp = TableGetRequestBuilder::new(config, "ws_001", "user")
            .execute()
            .await
            .expect("获取数据表详情应成功");
        assert_eq!(resp.table_name, "user");
        assert_eq!(resp.description.as_deref(), Some("用户表"));
        assert_eq!(resp.fields.len(), 2);
        assert!(resp.fields[0].is_primary_key);
        assert_eq!(resp.created_time, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user"
        );
        assert_eq!(received[0].method, "GET");
    }
}
