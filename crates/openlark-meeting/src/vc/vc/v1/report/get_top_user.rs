//! 获取 Top 用户列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 获取 Top 用户列表请求
pub struct GetTopUserReportRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl GetTopUserReportRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_top_user>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/reports/get_top_user
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/reports/get_top_user");
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(req, &self.config, Some(option), "获取 Top 用户列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../reports/get_top_user → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_top_user_report_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reports/get_top_user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "top_users": [
                        { "user_id": "user_001", "meeting_count": 42 }
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

        let resp = GetTopUserReportRequest::new(config)
            .query_param("start_time", "1719888000")
            .query_param("end_time", "1719974400")
            .query_param("limit", "10")
            .execute()
            .await
            .expect("获取 Top 用户列表应成功");
        assert_eq!(resp["top_users"][0]["user_id"], "user_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/reports/get_top_user"
        );
        assert_eq!(
            received[0].url.query_pairs().count(),
            3,
            "应携带三个查询参数"
        );
    }
}
