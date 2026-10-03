//! 工作台访问数据
//!
//! 文档: <https://open.feishu.cn/document/workplace-v1/workplace_access_data/search>
//! docPath: <https://open.feishu.cn/document/workplace-v1/workplace_access_data/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 工作台访问数据查询 Builder
#[derive(Debug, Clone)]
pub struct AccessDataSearchWorkplaceRequestBuilder {
    config: Config,
    start_date: String,
    end_date: String,
}

impl AccessDataSearchWorkplaceRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            start_date: String::new(),
            end_date: String::new(),
        }
    }

    /// 设置开始日期
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    /// 设置结束日期
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AccessDataSearchWorkplaceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AccessDataSearchWorkplaceResponse> {
        let url = "/open-apis/workplace/v1/workplace_access_data/search";

        let request = AccessDataSearchWorkplaceRequest {
            start_date: self.start_date,
            end_date: self.end_date,
        };

        let req: ApiRequest<AccessDataSearchWorkplaceResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request)?);
        Transport::request_typed(req, &self.config, Some(option), "Operation").await
    }
}

/// 工作台访问数据查询请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccessDataSearchWorkplaceRequest {
    /// 开始日期 (YYYY-MM-DD)
    #[serde(rename = "start_date")]
    start_date: String,
    /// 结束日期 (YYYY-MM-DD)
    #[serde(rename = "end_date")]
    end_date: String,
}

/// 工作台访问数据查询响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccessDataSearchWorkplaceResponse {
    /// 访问数据列表
    pub items: Vec<WorkplaceAccessData>,
}

/// 工作台访问数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkplaceAccessData {
    /// 日期
    pub date: String,
    /// 访问量
    #[serde(rename = "visit_count")]
    pub visit_count: i64,
    /// 访客数
    #[serde(rename = "visitor_count")]
    pub visitor_count: i64,
}

impl ApiResponseTrait for AccessDataSearchWorkplaceResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to AccessDataSearchWorkplaceRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type AccessDataSearchWorkplaceBuilder = AccessDataSearchWorkplaceRequestBuilder;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：POST .../workplace_access_data/search → 强类型 AccessDataSearchWorkplaceResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_search_workplace_access_data_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/workplace/v1/workplace_access_data/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "items": [ { "date": "2026-01-01", "visit_count": 10, "visitor_count": 5 } ] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = AccessDataSearchWorkplaceRequestBuilder::new(config)
            .start_date("2026-01-01")
            .end_date("2026-01-02")
            .execute()
            .await
            .expect("工作台访问数据查询应成功");
        assert_eq!(resp.items[0].date, "2026-01-01");
        assert_eq!(resp.items[0].visit_count, 10);
        assert_eq!(resp.items[0].visitor_count, 5);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/workplace/v1/workplace_access_data/search"
        );
    }
}
