//! 获取数据表详细信息
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/tables/:table_name
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-table/table_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
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
        validate_required!(self.workspace_id, "workspace_id 不能为空");
        validate_required!(self.table_name, "table_name 不能为空");

        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/tables/{}",
            self.workspace_id, self.table_name
        );

        let req: ApiRequest<TableGetResponse> = ApiRequest::get(&url);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 数据表列信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ColumnInfo {
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

/// 数据表详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TableGetResponse {
    /// 数据表名，如 student
    #[serde(rename = "name")]
    pub name: String,
    /// 数据表描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 数据表列
    #[serde(rename = "columns")]
    pub columns: Vec<ColumnInfo>,
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
    /// 官方示例：data 含 name/description/columns（列含 10 个契约字段）。
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
                    "name": "student",
                    "description": "student数据表",
                    "columns": [
                        {
                            "name": "id",
                            "description": "主键",
                            "data_type": "int",
                            "is_primary_key": true,
                            "is_unique": true,
                            "is_auto_increment": true,
                            "is_array": false,
                            "is_allow_null": false,
                            "default_value": null
                        },
                        {
                            "name": "name",
                            "data_type": "varchar",
                            "is_primary_key": false,
                            "is_unique": false,
                            "is_auto_increment": false,
                            "is_array": false,
                            "is_allow_null": true,
                            "default_value": " "
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

        let resp = TableGetRequestBuilder::new(config, "ws_001", "user")
            .execute()
            .await
            .expect("获取数据表详情应成功");
        assert_eq!(resp.name, "student");
        assert_eq!(resp.description.as_deref(), Some("student数据表"));
        assert_eq!(resp.columns.len(), 2);
        assert_eq!(resp.columns[0].name, "id");
        assert!(resp.columns[0].is_primary_key);
        assert!(resp.columns[0].is_auto_increment);
        assert!(!resp.columns[1].is_primary_key);
        assert!(resp.columns[1].is_allow_null);
        assert_eq!(resp.columns[1].default_value.as_deref(), Some(" "));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/tables/user"
        );
        assert_eq!(received[0].method, "GET");
    }
}
