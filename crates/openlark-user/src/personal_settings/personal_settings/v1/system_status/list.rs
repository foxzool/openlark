//! system_status list
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取系统状态列表的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusListRequest {
    config: Arc<Config>,
}

/// 获取系统状态列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusListResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusListRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 执行获取系统状态列表请求。
    pub async fn execute(self) -> SDKResult<SystemStatusListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusListResponse> {
        let req: ApiRequest<SystemStatusListResponse> =
            ApiRequest::get("/open-apis/personal_settings/v1/system_statuses");

        Transport::request_typed(req, &self.config, Some(option), "system_status list").await
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

    /// 端到端：GET .../system_statuses → 响应解析（data:{data:{...}} 双层信封）。
    #[tokio::test]
    async fn test_list_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/personal_settings/v1/system_statuses"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "items": [{ "system_status_id": "ss_001" }] } }
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

        let resp = SystemStatusListRequest::new(config)
            .execute()
            .await
            .expect("获取系统状态列表应成功");
        let data = resp.data.expect("响应 data 不应为空");
        assert_eq!(data["items"].as_array().unwrap().len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses"
        );
    }
}
