//! 获取会议报告
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;

/// 获取会议报告请求
#[derive(Debug, Clone)]
pub struct GetDailyReportRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 获取会议报告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDailyReportResponse {
    /// 报告数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetDailyReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDailyReportRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily>
    pub async fn execute(self) -> SDKResult<GetDailyReportResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDailyReportResponse> {
        let mut req: ApiRequest<GetDailyReportResponse> =
            ApiRequest::get("/open-apis/vc/v1/reports/get_daily");
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会议报告")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../reports/get_daily → 强类型 GetDailyReportResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_get_daily_report_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reports/get_daily"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "meeting_count": 12, "participant_count": 36 } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetDailyReportRequest::new(config)
            .query_param("start_time", "1719888000")
            .query_param("end_time", "1719974400")
            .execute()
            .await
            .expect("获取会议报告应成功");
        assert_eq!(resp.data["meeting_count"], json!(12));
        assert_eq!(resp.data["participant_count"], json!(36));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/reports/get_daily");
        assert_eq!(
            received[0].url.query_pairs().count(),
            2,
            "应携带两个查询参数"
        );
    }
}
