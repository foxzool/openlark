//! 删除系统状态
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/personal_settings-v1/system_status/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 删除系统状态的请求。
#[derive(Debug, Clone)]
pub struct SystemStatusDeleteRequest {
    config: Arc<Config>,
    /// 路径参数 `system_status_id`。
    system_status_id: String,
}

/// 删除系统状态响应 `data`（文档为空对象）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SystemStatusDeleteResponse {}

impl ApiResponseTrait for SystemStatusDeleteResponse {
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl SystemStatusDeleteRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            system_status_id: String::new(),
        }
    }

    /// 设置系统状态 ID（路径参数）。
    pub fn system_status_id(mut self, system_status_id: impl Into<String>) -> Self {
        self.system_status_id = system_status_id.into();
        self
    }

    /// 执行删除系统状态请求。
    pub async fn execute(self) -> SDKResult<SystemStatusDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SystemStatusDeleteResponse> {
        validate_required!(self.system_status_id.trim(), "system_status_id 不能为空");
        let path = format!(
            "/open-apis/personal_settings/v1/system_statuses/{}",
            self.system_status_id
        );
        let req: ApiRequest<SystemStatusDeleteResponse> = ApiRequest::delete(&path)
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "删除系统状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../system_statuses/{id} → 空 data。
    #[tokio::test]
    async fn test_delete_system_status_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/personal_settings/v1/system_statuses/ss_001",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
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

        let _resp = SystemStatusDeleteRequest::new(config)
            .system_status_id("ss_001")
            .execute()
            .await
            .expect("删除系统状态应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(
            received[0].url.path(),
            "/open-apis/personal_settings/v1/system_statuses/ss_001"
        );
    }
}
