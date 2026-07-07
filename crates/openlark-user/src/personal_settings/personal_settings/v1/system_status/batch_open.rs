//! system_status batch_open
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/batch_open>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量开启系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusBatchOpenRequest {
    config: Arc<Config>,
    status_id: String,
}

/// 批量开启系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusBatchOpenResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusBatchOpenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusBatchOpenRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, status_id: impl Into<String>) -> Self {
        Self {
            config,
            status_id: status_id.into(),
        }
    }

    /// 执行批量开启系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusBatchOpenResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusBatchOpenResponse> {
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}/batch_open",
            self.status_id
        );
        let req: ApiRequest<SystemStatusBatchOpenResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("system_status batch_open", "响应数据为空")
        })
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

    /// 端到端：POST .../system_statuses/{status_id}/batch_open → 响应解析。
    #[tokio::test]
    async fn test_batch_open_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_open",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "system_status_id": "ss_001" } }
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

        let resp = SystemStatusBatchOpenRequest::new(config, "ss_001")
            .execute()
            .await
            .expect("批量开启系统状态应成功");
        assert_eq!(resp.data.unwrap()["system_status_id"], "ss_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses/ss_001/batch_open"
        );
    }
}
