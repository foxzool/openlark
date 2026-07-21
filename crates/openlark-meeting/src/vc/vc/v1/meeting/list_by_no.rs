//! 获取与会议号关联的会议列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 获取与会议号关联的会议列表请求
pub struct ListByNoMeetingRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListByNoMeetingRequest {
    /// 创建请求实例。
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/meetings/list_by_no
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/vc/v1/meetings/list_by_no");
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "获取与会议号关联的会议列表",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../meetings/list_by_no → 裸 Value 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_list_by_no_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/meetings/list_by_no"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "meeting_infos": [
                        { "meeting_id": "mtg_001", "meeting_no": "123456789" }
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

        let resp = ListByNoMeetingRequest::new(config)
            .query_param("meeting_no", "123456789")
            .execute()
            .await
            .expect("获取与会议号关联的会议列表应成功");
        assert_eq!(resp["meeting_infos"][0]["meeting_id"], "mtg_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/vc/v1/meetings/list_by_no"
        );
        assert_eq!(received[0].url.query(), Some("meeting_no=123456789"));
    }
}
