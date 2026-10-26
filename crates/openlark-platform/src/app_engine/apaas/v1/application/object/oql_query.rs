//! 执行 OQL
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/oql_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 执行 OQL Builder
#[derive(Debug, Clone)]
pub struct OqlQueryRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 请求体
    body: OqlQueryBody,
}

impl OqlQueryRequestBuilder {
    /// 创建新的 Builder（query 为待执行的 OQL 语句，必填）
    pub fn new(config: Config, namespace: impl Into<String>, query: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            body: OqlQueryBody {
                query: query.into(),
                args: None,
                named_args: None,
            },
        }
    }

    /// 设置待执行的 OQL 语句（必填）
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置匿名参数具体值（JSON 字符串，如 "[\"_employee\"]"）
    pub fn args(mut self, args: impl Into<String>) -> Self {
        self.body.args = Some(args.into());
        self
    }

    /// 设置具名参数具体值（JSON 字符串，如 "{\"user_status\" : \"_used\"}"）
    pub fn named_args(mut self, named_args: impl Into<String>) -> Self {
        self.body.named_args = Some(named_args.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OqlQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<OqlQueryResponse> {
        validate_required!(self.body.query, "OQL 语句不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/oql_query",
            self.namespace
        );

        let req: ApiRequest<OqlQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "执行 OQL").await
    }
}

/// 执行 OQL 请求体（官方字段：query 必填，args/named_args 可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OqlQueryBody {
    /// 待执行的 OQL 语句
    pub query: String,
    /// 匿名参数具体值（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    /// 具名参数具体值（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_args: Option<String>,
}

/// 执行 OQL 响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OqlQueryResponse {
    /// 每一列的标题
    pub columns: Vec<String>,
    /// 每一行的值，以「key-value」的形式返回（JSON 字符串）
    pub rows: String,
}

impl ApiResponseTrait for OqlQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to OqlQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type OqlQueryBuilder = OqlQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../applications/{ns}/objects/oql_query → 官方响应形状（columns + rows）。
    #[tokio::test]
    async fn test_oql_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/oql_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "columns": ["_id"],
                    "rows": "[       {         \"_name\": \"Sample Text\",         \"_id\": 1234567890       }     ]"
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

        let resp = OqlQueryRequestBuilder::new(
            config,
            "ns_test",
            "SELECT _id, _name FROM _user WHERE _type = $1 LIMIT 10",
        )
        .args("[\"_employee\"]")
        .named_args("{\"user_status\" : \"_used\"}")
        .execute()
        .await
        .expect("执行 OQL 查询应成功");
        assert_eq!(resp.columns, vec!["_id".to_string()]);
        assert!(resp.rows.contains("Sample Text"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/oql_query"
        );
        // 请求体须与官方文档一致：query + args + named_args
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "query": "SELECT _id, _name FROM _user WHERE _type = $1 LIMIT 10",
                "args": "[\"_employee\"]",
                "named_args": "{\"user_status\" : \"_used\"}"
            })
        );
    }
}
