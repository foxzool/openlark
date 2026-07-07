//! system_status patch
//! docPath: <https://open.feishu.cn/document/server-docs/personal_settings-v1/system_status/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 更新系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusPatchRequest {
    config: Arc<Config>,
    status_id: String,
}

/// 更新系统状态的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusPatchResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for SystemStatusPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SystemStatusPatchRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            status_id: String::new(),
        }
    }

    /// 设置状态 ID。
    pub fn status_id(mut self, status_id: impl Into<String>) -> Self {
        self.status_id = status_id.into();
        self
    }

    /// 执行更新系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusPatchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusPatchResponse> {
        validate_required!(self.status_id.trim(), "status_id 不能为空");
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            self.status_id
        );
        let req: ApiRequest<SystemStatusPatchResponse> = ApiRequest::patch(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("system_status patch", "响应数据为空")
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

    /// 端到端：PATCH .../system_statuses/{status_id} → 响应解析。
    #[tokio::test]
    async fn test_patch_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001",
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

        let resp = SystemStatusPatchRequest::new(config)
            .status_id("ss_001")
            .execute()
            .await
            .expect("更新系统状态应成功");
        assert_eq!(resp.data.unwrap()["system_status_id"], "ss_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses/ss_001"
        );
    }
}
