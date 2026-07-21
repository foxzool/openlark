//! 生成 CalDAV 配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

/// 生成 CalDAV 配置请求
pub struct GenerateCaldavConfRequest {
    config: Config,
}

impl GenerateCaldavConfRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/settings/generate_caldav_conf
        // 注意：此端点在 CalendarApiV4 中可能不存在，需要添加或使用常量
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/calendar/v4/settings/generate_caldav_conf");

        Transport::request_typed(req, &self.config, Some(option), "生成 CalDAV 配置").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../calendar/v4/settings/generate_caldav_conf → 裸 Value 解析（data.caldav_url）。
    #[tokio::test]
    async fn test_generate_caldav_conf_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/calendar/v4/settings/generate_caldav_conf"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "caldav_url": "https://caldav.example.com/dav",
                    "username": "user_001"
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

        let resp = GenerateCaldavConfRequest::new(config)
            .execute()
            .await
            .expect("生成 CalDAV 配置应成功");
        assert_eq!(resp["caldav_url"], json!("https://caldav.example.com/dav"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/calendar/v4/settings/generate_caldav_conf"
        );
        assert_eq!(received[0].method, "POST");
    }
}
