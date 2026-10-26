//! 查询应用统计数据
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/aily-v1/tenant-app_stat/list>

use crate::endpoints::AILY_V1_APP_STATS;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
    validate_required,
};

/// 查询应用统计数据请求。
///
/// 用于分页查询租户下的工作流/智能体应用使用情况的统计数据。
pub struct ListAppStatsRequest {
    config: Config,
    /// 统计起始时间，秒级时间戳（查询参数，必填）。
    start_at: String,
    /// 统计结束时间，秒级时间戳（查询参数，必填）。
    end_at: String,
    /// 数据筛选类型：`app` 工作流应用 / `agent` 智能体（查询参数，必填）。
    filter_type: String,
    /// 分页标记（查询参数，可选）。
    page_token: Option<String>,
    /// 每页返回的数据条数，默认 20，最大 100（查询参数，可选）。
    page_size: Option<i32>,
}

impl ListAppStatsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            start_at: String::new(),
            end_at: String::new(),
            filter_type: String::new(),
            page_token: None,
            page_size: None,
        }
    }

    /// 设置统计起始时间（查询参数，必填，秒级时间戳）。
    pub fn start_at(mut self, start_at: impl Into<String>) -> Self {
        self.start_at = start_at.into();
        self
    }

    /// 设置统计结束时间（查询参数，必填，秒级时间戳）。
    pub fn end_at(mut self, end_at: impl Into<String>) -> Self {
        self.end_at = end_at.into();
        self
    }

    /// 设置数据筛选类型（查询参数，必填，`app` 或 `agent`）。
    pub fn filter_type(mut self, filter_type: impl Into<String>) -> Self {
        self.filter_type = filter_type.into();
        self
    }

    /// 设置分页标记（查询参数，可选）。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置每页返回的数据条数（查询参数，可选，默认 20，最大 100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.start_at, "start_at 不能为空");
        validate_required!(self.end_at, "end_at 不能为空");
        validate_required!(self.filter_type, "filter_type 不能为空");

        // url: GET:/open-apis/aily/v1/app_stats
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(AILY_V1_APP_STATS)
            .query("start_at", self.start_at)
            .query("end_at", self.end_at)
            .query("filter_type", self.filter_type);
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }

        Transport::request_typed(req, &self.config, Some(option), "查询应用统计数据").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn app_stats_issue_194_builder_access() {
        let request = ListAppStatsRequest::new(Config::default())
            .start_at("1698729600")
            .end_at("1701321599")
            .filter_type("agent");

        assert_eq!(request.start_at, "1698729600");
        assert_eq!(request.end_at, "1701321599");
        assert_eq!(request.filter_type, "agent");
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_app_stats_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_APP_STATS;
        let url = AILY_V1_APP_STATS.to_string();
        assert_eq!(url, "/open-apis/aily/v1/app_stats");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }

    /// 端到端：GET /open-apis/aily/v1/app_stats 携带官方必填查询参数。
    #[tokio::test]
    async fn test_list_app_stats_sends_required_query_params() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/aily/v1/app_stats"))
            .and(query_param("start_at", "1698729600"))
            .and(query_param("end_at", "1701321599"))
            .and(query_param("filter_type", "agent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "items": [], "has_more": false }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ListAppStatsRequest::new(config)
            .start_at("1698729600")
            .end_at("1701321599")
            .filter_type("agent")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
