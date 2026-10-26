//! 查询任务
//! docPath: <https://open.feishu.cn/document/server-docs/report-v1/task/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 查询任务请求体。
///
/// `commit_start_time`/`commit_end_time`/`page_size` 必填；
/// `page_token` 官方必填但首次调用须传空值（从头遍历），故为非 Option 且默认空串、不做非空校验；
/// `rule_id`/`user_id` 可选（官方文档 Required=No）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryReportTaskBody {
    /// 提交开始时间时间戳（必填）。
    pub commit_start_time: i64,
    /// 提交结束时间时间戳（必填）。
    pub commit_end_time: i64,
    /// 汇报规则 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 分页标记（必填，首次调用传空值表示从头遍历）。
    #[serde(default)]
    pub page_token: String,
    /// 单次分页返回的条数（必填，取值范围 0～20）。
    pub page_size: i64,
}

/// 查询任务请求。
#[derive(Debug, Clone)]
pub struct QueryReportTaskRequest {
    config: Arc<Config>,
    body: QueryReportTaskBody,
    /// 用户 ID 类型（查询参数，默认 open_id）。
    user_id_type: Option<String>,
}

/// 查询任务响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryReportTaskResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for QueryReportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl QueryReportTaskRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: QueryReportTaskBody::default(),
            user_id_type: None,
        }
    }

    /// 设置提交开始时间时间戳（必填）。
    pub fn commit_start_time(mut self, commit_start_time: i64) -> Self {
        self.body.commit_start_time = commit_start_time;
        self
    }

    /// 设置提交结束时间时间戳（必填）。
    pub fn commit_end_time(mut self, commit_end_time: i64) -> Self {
        self.body.commit_end_time = commit_end_time;
        self
    }

    /// 设置汇报规则 ID。
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.body.rule_id = Some(rule_id.into());
        self
    }

    /// 设置用户 ID。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = Some(user_id.into());
        self
    }

    /// 设置分页标记（必填，首次调用传空值表示从头遍历）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.body.page_token = page_token.into();
        self
    }

    /// 设置单次分页返回的条数（必填，取值范围 0～20）。
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.body.page_size = page_size;
        self
    }

    /// 设置用户 ID 类型（默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行查询任务请求。
    pub async fn execute(self) -> SDKResult<QueryReportTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行查询任务请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<QueryReportTaskResponse> {
        let path = "/open-apis/report/v1/tasks/query".to_string();
        let req: ApiRequest<QueryReportTaskResponse> = ApiRequest::post(&path)
            .body(serde_json::to_value(&self.body)?)
            .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(req, &self.config, Some(option), "查询任务").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/report/v1/tasks/query + 官方形状 body + query user_id_type → 响应解析。
    #[tokio::test]
    async fn test_query_report_task_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/report/v1/tasks/query"))
            .and(wiremock::matchers::body_partial_json(json!({
                "commit_start_time": 1618279343,
                "commit_end_time": 1623722543,
                "page_token": "6969507070261329948",
                "page_size": 10,
                "rule_id": "6894419345318182932",
                "user_id": "ou_c26134180ba2611aede183e5e3b531ed"
            })))
            .and(wiremock::matchers::query_param("user_id_type", "open_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = QueryReportTaskRequest::new(config)
            .commit_start_time(1618279343)
            .commit_end_time(1623722543)
            .rule_id("6894419345318182932")
            .user_id("ou_c26134180ba2611aede183e5e3b531ed")
            .page_token("6969507070261329948")
            .page_size(10)
            .user_id_type("open_id")
            .execute()
            .await
            .expect("查询任务应成功");
        assert!(resp.data.is_some());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/report/v1/tasks/query");
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["commit_start_time"], 1618279343);
        assert_eq!(sent["commit_end_time"], 1623722543);
        assert_eq!(sent["page_token"], "6969507070261329948");
        assert_eq!(sent["page_size"], 10);
        assert_eq!(sent["rule_id"], "6894419345318182932");
        assert_eq!(sent["user_id"], "ou_c26134180ba2611aede183e5e3b531ed");
    }

    /// 首次调用不设可选字段：body 仅含必填字段，page_token 传空值，可选字段不序列化。
    #[test]
    fn test_query_report_task_body_minimal_shape() {
        let body = QueryReportTaskBody {
            commit_start_time: 1618279343,
            commit_end_time: 1623722543,
            page_token: String::new(),
            page_size: 10,
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({
                "commit_start_time": 1618279343,
                "commit_end_time": 1623722543,
                "page_token": "",
                "page_size": 10
            })
        );
    }
}
