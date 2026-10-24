//! 获取工作空间下的数据表列表
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取数据表列表 Builder
#[derive(Debug, Clone)]
pub struct TableListRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 分页大小，默认 10，最大 500
    page_size: Option<u32>,
    /// 分页标记
    page_token: Option<String>,
}

impl TableListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, workspace_id: impl Into<String>) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小（取值范围 1-500，默认 10）
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TableListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<TableListResponse> {
        validate_required!(self.workspace_id, "workspace_id 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables",
            self.workspace_id
        );

        let mut req: ApiRequest<TableListResponse> = ApiRequest::get(&url);
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", &page_token);
        }
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 数据表列信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceDataTableColumnInfo {
    /// 列名
    #[serde(rename = "name")]
    pub name: String,
    /// 列描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据库数据类型
    #[serde(rename = "data_type")]
    pub data_type: String,
    /// 是否是主键
    #[serde(rename = "is_primary_key")]
    pub is_primary_key: bool,
    /// 是否唯一
    #[serde(rename = "is_unique")]
    pub is_unique: bool,
    /// 是否是自增
    #[serde(rename = "is_auto_increment")]
    pub is_auto_increment: bool,
    /// 是否是数组类型
    #[serde(rename = "is_array")]
    pub is_array: bool,
    /// 是否允许为空
    #[serde(rename = "is_allow_null")]
    pub is_allow_null: bool,
    /// 默认值
    #[serde(rename = "default_value", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

/// 数据表信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableInfo {
    /// 数据表名，如 student
    #[serde(rename = "name")]
    pub name: String,
    /// 数据表描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据表列
    #[serde(rename = "columns")]
    pub columns: Vec<WorkspaceDataTableColumnInfo>,
}

/// 数据表列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableListResponse {
    /// 数据表列表
    #[serde(rename = "items")]
    pub items: Vec<TableInfo>,
    /// 是否还有更多项
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时会同时返回新的 page_token
    #[serde(rename = "page_token", skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for TableListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to TableListRequestBuilder, will be removed in v1.0 (#271)")]
pub type TableListBuilder = TableListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = TableListRequestBuilder::new(config.clone(), "test".to_string());
        let _ = request;
    }

    /// 端到端：GET .../apaas/v1/workspaces/{ws}/tables → 强类型 TableListResponse。
    /// 官方示例：items 含 name/description/columns（列含 10 个契约字段），data 含 has_more/page_token。
    #[tokio::test]
    async fn test_list_tables_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/workspaces/ws_001/tables"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": true,
                    "page_token": "next_token",
                    "items": [
                        {
                            "name": "student",
                            "description": "student数据表",
                            "columns": [
                                {
                                    "name": "column_name",
                                    "description": "列描述",
                                    "data_type": "varchar",
                                    "is_primary_key": false,
                                    "is_unique": false,
                                    "is_auto_increment": false,
                                    "is_array": false,
                                    "is_allow_null": false,
                                    "default_value": " "
                                }
                            ]
                        }
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

        let resp = TableListRequestBuilder::new(config, "ws_001")
            .page_size(20)
            .page_token("token_001")
            .execute()
            .await
            .expect("获取数据表列表应成功");
        assert!(resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("next_token"));
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].name, "student");
        assert_eq!(resp.items[0].columns.len(), 1);
        assert_eq!(resp.items[0].columns[0].data_type, "varchar");
        assert!(!resp.items[0].columns[0].is_primary_key);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables"
        );
        assert_eq!(received[0].method, "GET");
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=20"),
            "query should contain page_size=20: {query}"
        );
        assert!(
            query.contains("page_token=token_001"),
            "query should contain page_token: {query}"
        );
    }
}
